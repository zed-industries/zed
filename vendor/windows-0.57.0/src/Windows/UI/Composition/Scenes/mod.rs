windows_core::imp::define_interface!(ISceneBoundingBox, ISceneBoundingBox_Vtbl, 0x5d8ffc70_c618_4083_8251_9962593114aa);
impl windows_core::RuntimeType for ISceneBoundingBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneBoundingBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Center: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Center: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Extents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Extents: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Max: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Min: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Size: usize,
}
windows_core::imp::define_interface!(ISceneComponent, ISceneComponent_Vtbl, 0xae20fc96_226c_44bd_95cb_dd5ed9ebe9a5);
impl windows_core::RuntimeType for ISceneComponent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneComponent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ComponentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SceneComponentType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneComponentCollection, ISceneComponentCollection_Vtbl, 0xc483791c_5f46_45e4_b666_a3d2259f9b2e);
impl windows_core::RuntimeType for ISceneComponentCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneComponentCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISceneComponentFactory, ISceneComponentFactory_Vtbl, 0x5fbc5574_ddd8_5889_ab5b_d8fa716e7c9e);
impl windows_core::RuntimeType for ISceneComponentFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneComponentFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISceneMaterial, ISceneMaterial_Vtbl, 0x8ca74b7c_30df_4e07_9490_37875af1a123);
impl windows_core::RuntimeType for ISceneMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneMaterial_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISceneMaterialFactory, ISceneMaterialFactory_Vtbl, 0x67536c19_a707_5254_a495_7fdc799893b9);
impl windows_core::RuntimeType for ISceneMaterialFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneMaterialFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISceneMaterialInput, ISceneMaterialInput_Vtbl, 0x422a1642_1ef1_485c_97e9_ae6f95ad812f);
impl windows_core::RuntimeType for ISceneMaterialInput {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneMaterialInput_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISceneMaterialInputFactory, ISceneMaterialInputFactory_Vtbl, 0xa88feb74_7d0a_5e4c_a748_1015af9ca74f);
impl windows_core::RuntimeType for ISceneMaterialInputFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneMaterialInputFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISceneMesh, ISceneMesh_Vtbl, 0xee9a1530_1155_4c0c_92bd_40020cf78347);
impl windows_core::RuntimeType for ISceneMesh {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneMesh_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Bounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub PrimitiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PrimitiveTopology: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetPrimitiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetPrimitiveTopology: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub FillMeshAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, SceneAttributeSemantic, super::super::super::Graphics::DirectX::DirectXPixelFormat, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    FillMeshAttribute: usize,
}
windows_core::imp::define_interface!(ISceneMeshMaterialAttributeMap, ISceneMeshMaterialAttributeMap_Vtbl, 0xce843171_3d43_4855_aa69_31ff988d049d);
impl windows_core::RuntimeType for ISceneMeshMaterialAttributeMap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneMeshMaterialAttributeMap_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISceneMeshRendererComponent, ISceneMeshRendererComponent_Vtbl, 0x9929f7e3_6364_477e_98fe_74ed9fd4c2de);
impl windows_core::RuntimeType for ISceneMeshRendererComponent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneMeshRendererComponent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Material: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMaterial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Mesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UVMappings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneMeshRendererComponentStatics, ISceneMeshRendererComponentStatics_Vtbl, 0x4954f37a_4459_4521_bd6e_2b38b8d711ea);
impl windows_core::RuntimeType for ISceneMeshRendererComponentStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneMeshRendererComponentStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneMeshStatics, ISceneMeshStatics_Vtbl, 0x8412316c_7b57_473f_966b_81dc277b1751);
impl windows_core::RuntimeType for ISceneMeshStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneMeshStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneMetallicRoughnessMaterial, ISceneMetallicRoughnessMaterial_Vtbl, 0xc1d91446_799c_429e_a4e4_5da645f18e61);
impl windows_core::RuntimeType for ISceneMetallicRoughnessMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneMetallicRoughnessMaterial_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BaseColorInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBaseColorInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub BaseColorFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    BaseColorFactor: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetBaseColorFactor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetBaseColorFactor: usize,
    pub MetallicFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMetallicFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub MetallicRoughnessInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMetallicRoughnessInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RoughnessFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetRoughnessFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneMetallicRoughnessMaterialStatics, ISceneMetallicRoughnessMaterialStatics_Vtbl, 0x3bddca50_6d9d_4531_8dc4_b27e3e49b7ab);
impl windows_core::RuntimeType for ISceneMetallicRoughnessMaterialStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneMetallicRoughnessMaterialStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneModelTransform, ISceneModelTransform_Vtbl, 0xc05576c2_32b1_4269_980d_b98537100ae4);
impl windows_core::RuntimeType for ISceneModelTransform {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneModelTransform_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOrientation: usize,
    pub RotationAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub RotationAngleInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetRotationAngleInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub RotationAxis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RotationAxis: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRotationAxis: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRotationAxis: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Scale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Scale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetScale: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Translation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Translation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTranslation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTranslation: usize,
}
windows_core::imp::define_interface!(ISceneNode, ISceneNode_Vtbl, 0xacf2c247_f307_4581_9c41_af2e29c3b016);
impl windows_core::RuntimeType for ISceneNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Components: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Components: usize,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Transform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstComponentOfType: unsafe extern "system" fn(*mut core::ffi::c_void, SceneComponentType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneNodeCollection, ISceneNodeCollection_Vtbl, 0x29ada101_2dd9_4332_be63_60d2cf4269f2);
impl windows_core::RuntimeType for ISceneNodeCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneNodeCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISceneNodeStatics, ISceneNodeStatics_Vtbl, 0x579a0faa_be9d_4210_908c_93d15feed0b7);
impl windows_core::RuntimeType for ISceneNodeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneNodeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneObject, ISceneObject_Vtbl, 0x1e94249b_0f1b_49eb_a819_877d8450005b);
impl windows_core::RuntimeType for ISceneObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISceneObjectFactory, ISceneObjectFactory_Vtbl, 0x14fe799a_33e4_52ef_956c_44229d21f2c1);
impl windows_core::RuntimeType for ISceneObjectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneObjectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IScenePbrMaterial, IScenePbrMaterial_Vtbl, 0xaab6ebbe_d680_46df_8294_b6800a9f95e7);
impl windows_core::RuntimeType for IScenePbrMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScenePbrMaterial_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AlphaCutoff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetAlphaCutoff: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub AlphaMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SceneAlphaMode) -> windows_core::HRESULT,
    pub SetAlphaMode: unsafe extern "system" fn(*mut core::ffi::c_void, SceneAlphaMode) -> windows_core::HRESULT,
    pub EmissiveInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEmissiveInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub EmissiveFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    EmissiveFactor: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetEmissiveFactor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetEmissiveFactor: usize,
    pub IsDoubleSided: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDoubleSided: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub NormalInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNormalInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NormalScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetNormalScale: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub OcclusionInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOcclusionInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OcclusionStrength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetOcclusionStrength: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScenePbrMaterialFactory, IScenePbrMaterialFactory_Vtbl, 0x2e3f3dfe_0b85_5727_b5be_b7d3cbac37fa);
impl windows_core::RuntimeType for IScenePbrMaterialFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScenePbrMaterialFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISceneRendererComponent, ISceneRendererComponent_Vtbl, 0xf1acb857_cf4f_4025_9b25_a2d1944cf507);
impl windows_core::RuntimeType for ISceneRendererComponent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneRendererComponent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISceneRendererComponentFactory, ISceneRendererComponentFactory_Vtbl, 0x1db6ed6c_aa2c_5967_9035_56352dc69658);
impl windows_core::RuntimeType for ISceneRendererComponentFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneRendererComponentFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISceneSurfaceMaterialInput, ISceneSurfaceMaterialInput_Vtbl, 0x9937da5c_a9ca_4cfc_b3aa_088356518742);
impl windows_core::RuntimeType for ISceneSurfaceMaterialInput {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneSurfaceMaterialInput_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BitmapInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CompositionBitmapInterpolationMode) -> windows_core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::CompositionBitmapInterpolationMode) -> windows_core::HRESULT,
    pub Surface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WrappingUMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SceneWrappingMode) -> windows_core::HRESULT,
    pub SetWrappingUMode: unsafe extern "system" fn(*mut core::ffi::c_void, SceneWrappingMode) -> windows_core::HRESULT,
    pub WrappingVMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SceneWrappingMode) -> windows_core::HRESULT,
    pub SetWrappingVMode: unsafe extern "system" fn(*mut core::ffi::c_void, SceneWrappingMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneSurfaceMaterialInputStatics, ISceneSurfaceMaterialInputStatics_Vtbl, 0x5a2394d3_6429_4589_bbcf_b84f4f3cfbfe);
impl windows_core::RuntimeType for ISceneSurfaceMaterialInputStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneSurfaceMaterialInputStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneVisual, ISceneVisual_Vtbl, 0x8e672c1e_d734_47b1_be14_3d694ffa4301);
impl windows_core::RuntimeType for ISceneVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Root: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneVisualStatics, ISceneVisualStatics_Vtbl, 0xb8347e9a_50aa_4527_8d34_de4cb8ea88b4);
impl windows_core::RuntimeType for ISceneVisualStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneVisualStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneBoundingBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneBoundingBox, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneBoundingBox, super::IAnimationObject, super::super::super::Foundation::IClosable, SceneObject, super::CompositionObject);
impl SceneBoundingBox {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Center(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Center)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Extents(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Extents)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Max(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Max)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Min(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Min)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Size(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SceneBoundingBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneBoundingBox>();
}
unsafe impl windows_core::Interface for SceneBoundingBox {
    type Vtable = ISceneBoundingBox_Vtbl;
    const IID: windows_core::GUID = <ISceneBoundingBox as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneBoundingBox {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneBoundingBox";
}
unsafe impl Send for SceneBoundingBox {}
unsafe impl Sync for SceneBoundingBox {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneComponent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneComponent, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneComponent, super::IAnimationObject, super::super::super::Foundation::IClosable, SceneObject, super::CompositionObject);
impl SceneComponent {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn ComponentType(&self) -> windows_core::Result<SceneComponentType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComponentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SceneComponent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneComponent>();
}
unsafe impl windows_core::Interface for SceneComponent {
    type Vtable = ISceneComponent_Vtbl;
    const IID: windows_core::GUID = <ISceneComponent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneComponent";
}
unsafe impl Send for SceneComponent {}
unsafe impl Sync for SceneComponent {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneComponentCollection(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(SceneComponentCollection, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(SceneComponentCollection, super::IAnimationObject, super::super::super::Foundation::IClosable, super::super::super::Foundation::Collections::IIterable::<SceneComponent>, super::super::super::Foundation::Collections::IVector::<SceneComponent>, SceneObject, super::CompositionObject);
#[cfg(feature = "Foundation_Collections")]
impl SceneComponentCollection {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<SceneComponent>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<SceneComponent>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<SceneComponent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<SceneComponent>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<SceneComponent>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneComponent>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneComponent>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneComponent>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<SceneComponent>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[Option<SceneComponent>]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for SceneComponentCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IVector<SceneComponent>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for SceneComponentCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<SceneComponent>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IVector<SceneComponent> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for SceneComponentCollection {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneComponentCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for SceneComponentCollection {
    type Item = SceneComponent;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &SceneComponentCollection {
    type Item = SceneComponent;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(windows_core::Interface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for SceneComponentCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for SceneComponentCollection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneMaterial(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneMaterial, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneMaterial, super::IAnimationObject, super::super::super::Foundation::IClosable, SceneObject, super::CompositionObject);
impl SceneMaterial {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for SceneMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneMaterial>();
}
unsafe impl windows_core::Interface for SceneMaterial {
    type Vtable = ISceneMaterial_Vtbl;
    const IID: windows_core::GUID = <ISceneMaterial as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMaterial";
}
unsafe impl Send for SceneMaterial {}
unsafe impl Sync for SceneMaterial {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneMaterialInput(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneMaterialInput, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneMaterialInput, super::IAnimationObject, super::super::super::Foundation::IClosable, SceneObject, super::CompositionObject);
impl SceneMaterialInput {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for SceneMaterialInput {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneMaterialInput>();
}
unsafe impl windows_core::Interface for SceneMaterialInput {
    type Vtable = ISceneMaterialInput_Vtbl;
    const IID: windows_core::GUID = <ISceneMaterialInput as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneMaterialInput {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMaterialInput";
}
unsafe impl Send for SceneMaterialInput {}
unsafe impl Sync for SceneMaterialInput {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneMesh(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneMesh, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneMesh, super::IAnimationObject, super::super::super::Foundation::IClosable, SceneObject, super::CompositionObject);
impl SceneMesh {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn Bounds(&self) -> windows_core::Result<SceneBoundingBox> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bounds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn PrimitiveTopology(&self) -> windows_core::Result<super::super::super::Graphics::DirectX::DirectXPrimitiveTopology> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimitiveTopology)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetPrimitiveTopology(&self, value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrimitiveTopology)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn FillMeshAttribute<P0>(&self, semantic: SceneAttributeSemantic, format: super::super::super::Graphics::DirectX::DirectXPixelFormat, memory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::MemoryBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).FillMeshAttribute)(windows_core::Interface::as_raw(this), semantic, format, memory.param().abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> windows_core::Result<SceneMesh>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::ISceneMeshStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISceneMeshStatics<R, F: FnOnce(&ISceneMeshStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SceneMesh, ISceneMeshStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SceneMesh {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneMesh>();
}
unsafe impl windows_core::Interface for SceneMesh {
    type Vtable = ISceneMesh_Vtbl;
    const IID: windows_core::GUID = <ISceneMesh as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneMesh {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMesh";
}
unsafe impl Send for SceneMesh {}
unsafe impl Sync for SceneMesh {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneMeshMaterialAttributeMap(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneMeshMaterialAttributeMap, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(SceneMeshMaterialAttributeMap, super::IAnimationObject, super::super::super::Foundation::IClosable, super::super::super::Foundation::Collections::IIterable::<super::super::super::Foundation::Collections::IKeyValuePair::<windows_core::HSTRING, SceneAttributeSemantic>>, super::super::super::Foundation::Collections::IMap::<windows_core::HSTRING, SceneAttributeSemantic>, SceneObject, super::CompositionObject);
impl SceneMeshMaterialAttributeMap {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, SceneAttributeSemantic>>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, SceneAttributeSemantic>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<SceneAttributeSemantic> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, SceneAttributeSemantic>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert(&self, key: &windows_core::HSTRING, value: SceneAttributeSemantic) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for SceneMeshMaterialAttributeMap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneMeshMaterialAttributeMap>();
}
unsafe impl windows_core::Interface for SceneMeshMaterialAttributeMap {
    type Vtable = ISceneMeshMaterialAttributeMap_Vtbl;
    const IID: windows_core::GUID = <ISceneMeshMaterialAttributeMap as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneMeshMaterialAttributeMap {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for SceneMeshMaterialAttributeMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, SceneAttributeSemantic>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &SceneMeshMaterialAttributeMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, SceneAttributeSemantic>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl Send for SceneMeshMaterialAttributeMap {}
unsafe impl Sync for SceneMeshMaterialAttributeMap {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneMeshRendererComponent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneMeshRendererComponent, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneMeshRendererComponent, super::IAnimationObject, super::super::super::Foundation::IClosable, SceneRendererComponent, SceneComponent, SceneObject, super::CompositionObject);
impl SceneMeshRendererComponent {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn ComponentType(&self) -> windows_core::Result<SceneComponentType> {
        let this = &windows_core::Interface::cast::<ISceneComponent>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComponentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Material(&self) -> windows_core::Result<SceneMaterial> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Material)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMaterial<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneMaterial>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaterial)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Mesh(&self) -> windows_core::Result<SceneMesh> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mesh)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMesh<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneMesh>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMesh)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn UVMappings(&self) -> windows_core::Result<SceneMeshMaterialAttributeMap> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UVMappings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create<P0>(compositor: P0) -> windows_core::Result<SceneMeshRendererComponent>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::ISceneMeshRendererComponentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISceneMeshRendererComponentStatics<R, F: FnOnce(&ISceneMeshRendererComponentStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SceneMeshRendererComponent, ISceneMeshRendererComponentStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SceneMeshRendererComponent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneMeshRendererComponent>();
}
unsafe impl windows_core::Interface for SceneMeshRendererComponent {
    type Vtable = ISceneMeshRendererComponent_Vtbl;
    const IID: windows_core::GUID = <ISceneMeshRendererComponent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneMeshRendererComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMeshRendererComponent";
}
unsafe impl Send for SceneMeshRendererComponent {}
unsafe impl Sync for SceneMeshRendererComponent {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneMetallicRoughnessMaterial(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneMetallicRoughnessMaterial, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneMetallicRoughnessMaterial, super::IAnimationObject, super::super::super::Foundation::IClosable, ScenePbrMaterial, SceneMaterial, SceneObject, super::CompositionObject);
impl SceneMetallicRoughnessMaterial {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn BaseColorInput(&self) -> windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseColorInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBaseColorInput<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneMaterialInput>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBaseColorInput)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn BaseColorFactor(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector4> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseColorFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetBaseColorFactor(&self, value: super::super::super::Foundation::Numerics::Vector4) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBaseColorFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MetallicFactor(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MetallicFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMetallicFactor(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMetallicFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MetallicRoughnessInput(&self) -> windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MetallicRoughnessInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMetallicRoughnessInput<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneMaterialInput>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMetallicRoughnessInput)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RoughnessFactor(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoughnessFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRoughnessFactor(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRoughnessFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> windows_core::Result<SceneMetallicRoughnessMaterial>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::ISceneMetallicRoughnessMaterialStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AlphaCutoff(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlphaCutoff)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlphaCutoff(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAlphaCutoff)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlphaMode(&self) -> windows_core::Result<SceneAlphaMode> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlphaMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlphaMode(&self, value: SceneAlphaMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAlphaMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EmissiveInput(&self) -> windows_core::Result<SceneMaterialInput> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmissiveInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEmissiveInput<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneMaterialInput>,
    {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetEmissiveInput)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn EmissiveFactor(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmissiveFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetEmissiveFactor(&self, value: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetEmissiveFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleSided(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleSided)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDoubleSided(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDoubleSided)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NormalInput(&self) -> windows_core::Result<SceneMaterialInput> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNormalInput<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneMaterialInput>,
    {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNormalInput)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NormalScale(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNormalScale(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNormalScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OcclusionInput(&self) -> windows_core::Result<SceneMaterialInput> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OcclusionInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOcclusionInput<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneMaterialInput>,
    {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOcclusionInput)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OcclusionStrength(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OcclusionStrength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOcclusionStrength(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOcclusionStrength)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ISceneMetallicRoughnessMaterialStatics<R, F: FnOnce(&ISceneMetallicRoughnessMaterialStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SceneMetallicRoughnessMaterial, ISceneMetallicRoughnessMaterialStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SceneMetallicRoughnessMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneMetallicRoughnessMaterial>();
}
unsafe impl windows_core::Interface for SceneMetallicRoughnessMaterial {
    type Vtable = ISceneMetallicRoughnessMaterial_Vtbl;
    const IID: windows_core::GUID = <ISceneMetallicRoughnessMaterial as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneMetallicRoughnessMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial";
}
unsafe impl Send for SceneMetallicRoughnessMaterial {}
unsafe impl Sync for SceneMetallicRoughnessMaterial {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneModelTransform(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneModelTransform, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneModelTransform, super::IAnimationObject, super::super::super::Foundation::IClosable, super::CompositionTransform, super::CompositionObject);
impl SceneModelTransform {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetOrientation(&self, value: super::super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOrientation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAngle(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAngle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAngle(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAngle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAngleInDegrees(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAngleInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAngleInDegrees(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAngleInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RotationAxis(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRotationAxis(&self, value: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAxis)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Scale(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetScale(&self, value: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Translation(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTranslation(&self, value: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTranslation)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SceneModelTransform {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneModelTransform>();
}
unsafe impl windows_core::Interface for SceneModelTransform {
    type Vtable = ISceneModelTransform_Vtbl;
    const IID: windows_core::GUID = <ISceneModelTransform as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneModelTransform {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneModelTransform";
}
unsafe impl Send for SceneModelTransform {}
unsafe impl Sync for SceneModelTransform {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneNode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneNode, super::IAnimationObject, super::super::super::Foundation::IClosable, SceneObject, super::CompositionObject);
impl SceneNode {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> windows_core::Result<SceneNodeCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Components(&self) -> windows_core::Result<SceneComponentCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Components)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parent(&self) -> windows_core::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Transform(&self) -> windows_core::Result<SceneModelTransform> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transform)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FindFirstComponentOfType(&self, value: SceneComponentType) -> windows_core::Result<SceneComponent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindFirstComponentOfType)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create<P0>(compositor: P0) -> windows_core::Result<SceneNode>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::ISceneNodeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISceneNodeStatics<R, F: FnOnce(&ISceneNodeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SceneNode, ISceneNodeStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SceneNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneNode>();
}
unsafe impl windows_core::Interface for SceneNode {
    type Vtable = ISceneNode_Vtbl;
    const IID: windows_core::GUID = <ISceneNode as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneNode {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneNode";
}
unsafe impl Send for SceneNode {}
unsafe impl Sync for SceneNode {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneNodeCollection(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(SceneNodeCollection, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(SceneNodeCollection, super::IAnimationObject, super::super::super::Foundation::IClosable, super::super::super::Foundation::Collections::IIterable::<SceneNode>, super::super::super::Foundation::Collections::IVector::<SceneNode>, SceneObject, super::CompositionObject);
#[cfg(feature = "Foundation_Collections")]
impl SceneNodeCollection {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<SceneNode>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<SceneNode>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<SceneNode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<SceneNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneNode>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneNode>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneNode>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<SceneNode>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[Option<SceneNode>]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for SceneNodeCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IVector<SceneNode>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for SceneNodeCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<SceneNode>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IVector<SceneNode> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for SceneNodeCollection {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneNodeCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for SceneNodeCollection {
    type Item = SceneNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &SceneNodeCollection {
    type Item = SceneNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(windows_core::Interface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for SceneNodeCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for SceneNodeCollection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneObject, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneObject, super::IAnimationObject, super::super::super::Foundation::IClosable, super::CompositionObject);
impl SceneObject {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for SceneObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneObject>();
}
unsafe impl windows_core::Interface for SceneObject {
    type Vtable = ISceneObject_Vtbl;
    const IID: windows_core::GUID = <ISceneObject as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneObject {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneObject";
}
unsafe impl Send for SceneObject {}
unsafe impl Sync for SceneObject {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ScenePbrMaterial(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ScenePbrMaterial, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ScenePbrMaterial, super::IAnimationObject, super::super::super::Foundation::IClosable, SceneMaterial, SceneObject, super::CompositionObject);
impl ScenePbrMaterial {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn AlphaCutoff(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlphaCutoff)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlphaCutoff(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAlphaCutoff)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlphaMode(&self) -> windows_core::Result<SceneAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlphaMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlphaMode(&self, value: SceneAlphaMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAlphaMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EmissiveInput(&self) -> windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmissiveInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEmissiveInput<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneMaterialInput>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEmissiveInput)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn EmissiveFactor(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmissiveFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetEmissiveFactor(&self, value: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEmissiveFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleSided(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleSided)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDoubleSided(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDoubleSided)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NormalInput(&self) -> windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNormalInput<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneMaterialInput>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNormalInput)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NormalScale(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNormalScale(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNormalScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OcclusionInput(&self) -> windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OcclusionInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOcclusionInput<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneMaterialInput>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOcclusionInput)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OcclusionStrength(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OcclusionStrength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOcclusionStrength(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOcclusionStrength)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ScenePbrMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IScenePbrMaterial>();
}
unsafe impl windows_core::Interface for ScenePbrMaterial {
    type Vtable = IScenePbrMaterial_Vtbl;
    const IID: windows_core::GUID = <IScenePbrMaterial as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ScenePbrMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ScenePbrMaterial";
}
unsafe impl Send for ScenePbrMaterial {}
unsafe impl Sync for ScenePbrMaterial {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneRendererComponent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneRendererComponent, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneRendererComponent, super::IAnimationObject, super::super::super::Foundation::IClosable, SceneComponent, SceneObject, super::CompositionObject);
impl SceneRendererComponent {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn ComponentType(&self) -> windows_core::Result<SceneComponentType> {
        let this = &windows_core::Interface::cast::<ISceneComponent>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComponentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SceneRendererComponent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneRendererComponent>();
}
unsafe impl windows_core::Interface for SceneRendererComponent {
    type Vtable = ISceneRendererComponent_Vtbl;
    const IID: windows_core::GUID = <ISceneRendererComponent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneRendererComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneRendererComponent";
}
unsafe impl Send for SceneRendererComponent {}
unsafe impl Sync for SceneRendererComponent {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneSurfaceMaterialInput(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneSurfaceMaterialInput, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneSurfaceMaterialInput, super::IAnimationObject, super::super::super::Foundation::IClosable, SceneMaterialInput, SceneObject, super::CompositionObject);
impl SceneSurfaceMaterialInput {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn BitmapInterpolationMode(&self) -> windows_core::Result<super::CompositionBitmapInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapInterpolationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBitmapInterpolationMode(&self, value: super::CompositionBitmapInterpolationMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBitmapInterpolationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Surface(&self) -> windows_core::Result<super::ICompositionSurface> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Surface)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSurface<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionSurface>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSurface)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn WrappingUMode(&self) -> windows_core::Result<SceneWrappingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WrappingUMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWrappingUMode(&self, value: SceneWrappingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWrappingUMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WrappingVMode(&self) -> windows_core::Result<SceneWrappingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WrappingVMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWrappingVMode(&self, value: SceneWrappingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWrappingVMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> windows_core::Result<SceneSurfaceMaterialInput>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::ISceneSurfaceMaterialInputStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISceneSurfaceMaterialInputStatics<R, F: FnOnce(&ISceneSurfaceMaterialInputStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SceneSurfaceMaterialInput, ISceneSurfaceMaterialInputStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SceneSurfaceMaterialInput {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneSurfaceMaterialInput>();
}
unsafe impl windows_core::Interface for SceneSurfaceMaterialInput {
    type Vtable = ISceneSurfaceMaterialInput_Vtbl;
    const IID: windows_core::GUID = <ISceneSurfaceMaterialInput as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneSurfaceMaterialInput {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput";
}
unsafe impl Send for SceneSurfaceMaterialInput {}
unsafe impl Sync for SceneSurfaceMaterialInput {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneVisual(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneVisual, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SceneVisual, super::IAnimationObject, super::super::super::Foundation::IClosable, super::ContainerVisual, super::Visual, super::CompositionObject);
impl SceneVisual {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn Children(&self) -> windows_core::Result<super::VisualCollection> {
        let this = &windows_core::Interface::cast::<super::IContainerVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Root(&self) -> windows_core::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Root)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SceneNode>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRoot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> windows_core::Result<SceneVisual>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::ISceneVisualStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AnchorPoint(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AnchorPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetAnchorPoint(&self, value: super::super::super::Foundation::Numerics::Vector2) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAnchorPoint)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BackfaceVisibility(&self) -> windows_core::Result<super::CompositionBackfaceVisibility> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackfaceVisibility)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackfaceVisibility(&self, value: super::CompositionBackfaceVisibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackfaceVisibility)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BorderMode(&self) -> windows_core::Result<super::CompositionBorderMode> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBorderMode(&self, value: super::CompositionBorderMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CenterPoint(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetCenterPoint(&self, value: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCenterPoint)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Clip(&self) -> windows_core::Result<super::CompositionClip> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clip)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetClip<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionClip>,
    {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetClip)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CompositeMode(&self) -> windows_core::Result<super::CompositionCompositeMode> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositeMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCompositeMode(&self, value: super::CompositionCompositeMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompositeMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Offset(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Offset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetOffset(&self, value: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetOrientation(&self, value: super::super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOrientation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Parent(&self) -> windows_core::Result<super::ContainerVisual> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RotationAngle(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAngle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAngle(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAngle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAngleInDegrees(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAngleInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAngleInDegrees(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAngleInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RotationAxis(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRotationAxis(&self, value: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAxis)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Scale(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetScale(&self, value: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Size(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetSize(&self, value: super::super::super::Foundation::Numerics::Vector2) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TransformMatrix(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransformMatrix(&self, value: super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransformMatrix)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ParentForTransform(&self) -> windows_core::Result<super::Visual> {
        let this = &windows_core::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentForTransform)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetParentForTransform<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Visual>,
    {
        let this = &windows_core::Interface::cast::<super::IVisual2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetParentForTransform)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativeOffsetAdjustment(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelativeOffsetAdjustment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativeOffsetAdjustment(&self, value: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRelativeOffsetAdjustment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativeSizeAdjustment(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = &windows_core::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelativeSizeAdjustment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativeSizeAdjustment(&self, value: super::super::super::Foundation::Numerics::Vector2) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRelativeSizeAdjustment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IVisual3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHitTestVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPixelSnappingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IVisual4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPixelSnappingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPixelSnappingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IVisual4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsPixelSnappingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ISceneVisualStatics<R, F: FnOnce(&ISceneVisualStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SceneVisual, ISceneVisualStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SceneVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneVisual>();
}
unsafe impl windows_core::Interface for SceneVisual {
    type Vtable = ISceneVisual_Vtbl;
    const IID: windows_core::GUID = <ISceneVisual as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneVisual {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneVisual";
}
unsafe impl Send for SceneVisual {}
unsafe impl Sync for SceneVisual {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SceneAlphaMode(pub i32);
impl SceneAlphaMode {
    pub const Opaque: Self = Self(0i32);
    pub const AlphaTest: Self = Self(1i32);
    pub const Blend: Self = Self(2i32);
}
impl windows_core::TypeKind for SceneAlphaMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SceneAlphaMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SceneAlphaMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SceneAlphaMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneAlphaMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SceneAttributeSemantic(pub i32);
impl SceneAttributeSemantic {
    pub const Index: Self = Self(0i32);
    pub const Vertex: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
    pub const TexCoord0: Self = Self(3i32);
    pub const TexCoord1: Self = Self(4i32);
    pub const Color: Self = Self(5i32);
    pub const Tangent: Self = Self(6i32);
}
impl windows_core::TypeKind for SceneAttributeSemantic {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SceneAttributeSemantic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SceneAttributeSemantic").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SceneAttributeSemantic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneAttributeSemantic;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SceneComponentType(pub i32);
impl SceneComponentType {
    pub const MeshRendererComponent: Self = Self(0i32);
}
impl windows_core::TypeKind for SceneComponentType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SceneComponentType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SceneComponentType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SceneComponentType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneComponentType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SceneWrappingMode(pub i32);
impl SceneWrappingMode {
    pub const ClampToEdge: Self = Self(0i32);
    pub const MirroredRepeat: Self = Self(1i32);
    pub const Repeat: Self = Self(2i32);
}
impl windows_core::TypeKind for SceneWrappingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SceneWrappingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SceneWrappingMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SceneWrappingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneWrappingMode;i4)");
}
