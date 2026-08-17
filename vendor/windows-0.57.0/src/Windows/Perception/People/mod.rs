windows_core::imp::define_interface!(IEyesPose, IEyesPose_Vtbl, 0x682a9b23_8a1e_5b86_a060_906ffacb62a4);
impl windows_core::RuntimeType for IEyesPose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEyesPose_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCalibrationValid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub Gaze: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    Gaze: usize,
    pub UpdateTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEyesPoseStatics, IEyesPoseStatics_Vtbl, 0x1cff7413_b21f_54c0_80c1_e60d994ca58c);
impl windows_core::RuntimeType for IEyesPoseStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEyesPoseStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    RequestAccessAsync: usize,
}
windows_core::imp::define_interface!(IHandMeshObserver, IHandMeshObserver_Vtbl, 0x85ae30cb_6fc3_55c4_a7b4_29e33896ca69);
impl windows_core::RuntimeType for IHandMeshObserver {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHandMeshObserver_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Input_Spatial")]
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input_Spatial"))]
    Source: usize,
    pub TriangleIndexCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub VertexCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetTriangleIndices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16) -> windows_core::HRESULT,
    pub GetVertexStateForPose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NeutralPose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NeutralPoseVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ModelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHandMeshVertexState, IHandMeshVertexState_Vtbl, 0x046c5fef_1d8b_55de_ab2c_1cd424886d8f);
impl windows_core::RuntimeType for IHandMeshVertexState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHandMeshVertexState_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Perception_Spatial")]
    pub CoordinateSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    CoordinateSystem: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetVertices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut HandMeshVertex) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetVertices: usize,
    pub UpdateTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHandPose, IHandPose_Vtbl, 0x4d98e79a_bb08_5d09_91de_df0dd3fae46c);
impl windows_core::RuntimeType for IHandPose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHandPose_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetJoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, HandJointKind, *mut JointPose, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetJoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetJoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const HandJointKind, u32, *mut JointPose, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetJoints: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetRelativeJoint: unsafe extern "system" fn(*mut core::ffi::c_void, HandJointKind, HandJointKind, *mut JointPose) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetRelativeJoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetRelativeJoints: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const HandJointKind, u32, *const HandJointKind, u32, *mut JointPose) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetRelativeJoints: usize,
}
windows_core::imp::define_interface!(IHeadPose, IHeadPose_Vtbl, 0x7f5ac5a5_49db_379f_9429_32a2faf34fa6);
impl windows_core::RuntimeType for IHeadPose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHeadPose_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ForwardDirection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ForwardDirection: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpDirection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpDirection: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct EyesPose(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EyesPose, windows_core::IUnknown, windows_core::IInspectable);
impl EyesPose {
    pub fn IsCalibrationValid(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCalibrationValid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn Gaze(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Spatial::SpatialRay>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gaze)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateTimestamp(&self) -> windows_core::Result<super::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IEyesPoseStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "UI_Input")]
    pub fn RequestAccessAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::UI::Input::GazeInputAccessStatus>> {
        Self::IEyesPoseStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IEyesPoseStatics<R, F: FnOnce(&IEyesPoseStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<EyesPose, IEyesPoseStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for EyesPose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEyesPose>();
}
unsafe impl windows_core::Interface for EyesPose {
    type Vtable = IEyesPose_Vtbl;
    const IID: windows_core::GUID = <IEyesPose as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EyesPose {
    const NAME: &'static str = "Windows.Perception.People.EyesPose";
}
unsafe impl Send for EyesPose {}
unsafe impl Sync for EyesPose {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HandMeshObserver(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HandMeshObserver, windows_core::IUnknown, windows_core::IInspectable);
impl HandMeshObserver {
    #[cfg(feature = "UI_Input_Spatial")]
    pub fn Source(&self) -> windows_core::Result<super::super::UI::Input::Spatial::SpatialInteractionSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TriangleIndexCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriangleIndexCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VertexCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VertexCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetTriangleIndices(&self, indices: &mut [u16]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetTriangleIndices)(windows_core::Interface::as_raw(this), indices.len().try_into().unwrap(), indices.as_mut_ptr()).ok() }
    }
    pub fn GetVertexStateForPose<P0>(&self, handpose: P0) -> windows_core::Result<HandMeshVertexState>
    where
        P0: windows_core::Param<HandPose>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVertexStateForPose)(windows_core::Interface::as_raw(this), handpose.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NeutralPose(&self) -> windows_core::Result<HandPose> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeutralPose)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NeutralPoseVersion(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeutralPoseVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ModelId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HandMeshObserver {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHandMeshObserver>();
}
unsafe impl windows_core::Interface for HandMeshObserver {
    type Vtable = IHandMeshObserver_Vtbl;
    const IID: windows_core::GUID = <IHandMeshObserver as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HandMeshObserver {
    const NAME: &'static str = "Windows.Perception.People.HandMeshObserver";
}
unsafe impl Send for HandMeshObserver {}
unsafe impl Sync for HandMeshObserver {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HandMeshVertexState(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HandMeshVertexState, windows_core::IUnknown, windows_core::IInspectable);
impl HandMeshVertexState {
    #[cfg(feature = "Perception_Spatial")]
    pub fn CoordinateSystem(&self) -> windows_core::Result<super::Spatial::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CoordinateSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetVertices(&self, vertices: &mut [HandMeshVertex]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetVertices)(windows_core::Interface::as_raw(this), vertices.len().try_into().unwrap(), vertices.as_mut_ptr()).ok() }
    }
    pub fn UpdateTimestamp(&self) -> windows_core::Result<super::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HandMeshVertexState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHandMeshVertexState>();
}
unsafe impl windows_core::Interface for HandMeshVertexState {
    type Vtable = IHandMeshVertexState_Vtbl;
    const IID: windows_core::GUID = <IHandMeshVertexState as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HandMeshVertexState {
    const NAME: &'static str = "Windows.Perception.People.HandMeshVertexState";
}
unsafe impl Send for HandMeshVertexState {}
unsafe impl Sync for HandMeshVertexState {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HandPose(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HandPose, windows_core::IUnknown, windows_core::IInspectable);
impl HandPose {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetJoint<P0>(&self, coordinatesystem: P0, joint: HandJointKind, jointpose: &mut JointPose) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetJoint)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), joint, jointpose, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetJoints<P0>(&self, coordinatesystem: P0, joints: &[HandJointKind], jointposes: &mut [JointPose]) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetJoints)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), joints.len().try_into().unwrap(), joints.as_ptr(), jointposes.len().try_into().unwrap(), jointposes.as_mut_ptr(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetRelativeJoint(&self, joint: HandJointKind, referencejoint: HandJointKind) -> windows_core::Result<JointPose> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRelativeJoint)(windows_core::Interface::as_raw(this), joint, referencejoint, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetRelativeJoints(&self, joints: &[HandJointKind], referencejoints: &[HandJointKind], jointposes: &mut [JointPose]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetRelativeJoints)(windows_core::Interface::as_raw(this), joints.len().try_into().unwrap(), joints.as_ptr(), referencejoints.len().try_into().unwrap(), referencejoints.as_ptr(), jointposes.len().try_into().unwrap(), jointposes.as_mut_ptr()).ok() }
    }
}
impl windows_core::RuntimeType for HandPose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHandPose>();
}
unsafe impl windows_core::Interface for HandPose {
    type Vtable = IHandPose_Vtbl;
    const IID: windows_core::GUID = <IHandPose as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HandPose {
    const NAME: &'static str = "Windows.Perception.People.HandPose";
}
unsafe impl Send for HandPose {}
unsafe impl Sync for HandPose {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HeadPose(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HeadPose, windows_core::IUnknown, windows_core::IInspectable);
impl HeadPose {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ForwardDirection(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForwardDirection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UpDirection(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpDirection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HeadPose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHeadPose>();
}
unsafe impl windows_core::Interface for HeadPose {
    type Vtable = IHeadPose_Vtbl;
    const IID: windows_core::GUID = <IHeadPose as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HeadPose {
    const NAME: &'static str = "Windows.Perception.People.HeadPose";
}
unsafe impl Send for HeadPose {}
unsafe impl Sync for HeadPose {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HandJointKind(pub i32);
impl HandJointKind {
    pub const Palm: Self = Self(0i32);
    pub const Wrist: Self = Self(1i32);
    pub const ThumbMetacarpal: Self = Self(2i32);
    pub const ThumbProximal: Self = Self(3i32);
    pub const ThumbDistal: Self = Self(4i32);
    pub const ThumbTip: Self = Self(5i32);
    pub const IndexMetacarpal: Self = Self(6i32);
    pub const IndexProximal: Self = Self(7i32);
    pub const IndexIntermediate: Self = Self(8i32);
    pub const IndexDistal: Self = Self(9i32);
    pub const IndexTip: Self = Self(10i32);
    pub const MiddleMetacarpal: Self = Self(11i32);
    pub const MiddleProximal: Self = Self(12i32);
    pub const MiddleIntermediate: Self = Self(13i32);
    pub const MiddleDistal: Self = Self(14i32);
    pub const MiddleTip: Self = Self(15i32);
    pub const RingMetacarpal: Self = Self(16i32);
    pub const RingProximal: Self = Self(17i32);
    pub const RingIntermediate: Self = Self(18i32);
    pub const RingDistal: Self = Self(19i32);
    pub const RingTip: Self = Self(20i32);
    pub const LittleMetacarpal: Self = Self(21i32);
    pub const LittleProximal: Self = Self(22i32);
    pub const LittleIntermediate: Self = Self(23i32);
    pub const LittleDistal: Self = Self(24i32);
    pub const LittleTip: Self = Self(25i32);
}
impl windows_core::TypeKind for HandJointKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HandJointKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HandJointKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HandJointKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.People.HandJointKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JointPoseAccuracy(pub i32);
impl JointPoseAccuracy {
    pub const High: Self = Self(0i32);
    pub const Approximate: Self = Self(1i32);
}
impl windows_core::TypeKind for JointPoseAccuracy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JointPoseAccuracy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JointPoseAccuracy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for JointPoseAccuracy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.People.JointPoseAccuracy;i4)");
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HandMeshVertex {
    pub Position: super::super::Foundation::Numerics::Vector3,
    pub Normal: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for HandMeshVertex {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeType for HandMeshVertex {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Perception.People.HandMeshVertex;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for HandMeshVertex {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JointPose {
    pub Orientation: super::super::Foundation::Numerics::Quaternion,
    pub Position: super::super::Foundation::Numerics::Vector3,
    pub Radius: f32,
    pub Accuracy: JointPoseAccuracy,
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::TypeKind for JointPose {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl windows_core::RuntimeType for JointPose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Perception.People.JointPose;struct(Windows.Foundation.Numerics.Quaternion;f4;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4;enum(Windows.Perception.People.JointPoseAccuracy;i4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl Default for JointPose {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
