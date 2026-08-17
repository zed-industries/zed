windows_core::imp::define_interface!(IInkAnalysisInkBullet, IInkAnalysisInkBullet_Vtbl, 0xee049368_6110_4136_95f9_ee809fc20030);
impl windows_core::RuntimeType for IInkAnalysisInkBullet {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkAnalysisInkBullet_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkAnalysisInkDrawing, IInkAnalysisInkDrawing_Vtbl, 0x6a85ed1f_1fe4_4e15_898c_8e112377e021);
impl windows_core::RuntimeType for IInkAnalysisInkDrawing {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkAnalysisInkDrawing_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DrawingKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InkAnalysisDrawingKind) -> windows_core::HRESULT,
    pub Center: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Points: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Points: usize,
}
windows_core::imp::define_interface!(IInkAnalysisInkWord, IInkAnalysisInkWord_Vtbl, 0x4bd228ad_83af_4034_8f3b_f8687dfff436);
impl windows_core::RuntimeType for IInkAnalysisInkWord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkAnalysisInkWord_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TextAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TextAlternates: usize,
}
windows_core::imp::define_interface!(IInkAnalysisLine, IInkAnalysisLine_Vtbl, 0xa06d048d_2b8d_4754_ad5a_d0871193a956);
impl windows_core::RuntimeType for IInkAnalysisLine {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkAnalysisLine_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IndentLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkAnalysisListItem, IInkAnalysisListItem_Vtbl, 0xb4e3c23f_c4c3_4c3a_a1a6_9d85547ee586);
impl windows_core::RuntimeType for IInkAnalysisListItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkAnalysisListItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkAnalysisNode, IInkAnalysisNode_Vtbl, 0x30831f05_5f64_4a2c_ba37_4f4887879574);
impl core::ops::Deref for IInkAnalysisNode {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkAnalysisNode, windows_core::IUnknown, windows_core::IInspectable);
impl IInkAnalysisNode {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<InkAnalysisNodeKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotatedBoundingRect)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parent(&self) -> windows_core::Result<IInkAnalysisNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokeIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IInkAnalysisNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkAnalysisNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InkAnalysisNodeKind) -> windows_core::HRESULT,
    pub BoundingRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RotatedBoundingRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RotatedBoundingRect: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStrokeIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStrokeIds: usize,
}
windows_core::imp::define_interface!(IInkAnalysisParagraph, IInkAnalysisParagraph_Vtbl, 0xd9ad045c_0cd1_4dd4_a68b_eb1f12b3d727);
impl windows_core::RuntimeType for IInkAnalysisParagraph {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkAnalysisParagraph_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkAnalysisResult, IInkAnalysisResult_Vtbl, 0x8948ba79_a243_4aa3_a294_1f98bd0ff580);
impl windows_core::RuntimeType for IInkAnalysisResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkAnalysisResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InkAnalysisStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkAnalysisRoot, IInkAnalysisRoot_Vtbl, 0x3fb6a3c4_2fde_4061_8502_a90f32545b84);
impl windows_core::RuntimeType for IInkAnalysisRoot {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkAnalysisRoot_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindNodes: unsafe extern "system" fn(*mut core::ffi::c_void, InkAnalysisNodeKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindNodes: usize,
}
windows_core::imp::define_interface!(IInkAnalysisWritingRegion, IInkAnalysisWritingRegion_Vtbl, 0xdd6d6231_bd16_4663_b5ae_941d3043ef5b);
impl windows_core::RuntimeType for IInkAnalysisWritingRegion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkAnalysisWritingRegion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkAnalyzer, IInkAnalyzer_Vtbl, 0xf12b8f95_0866_4dc5_8c77_f88614dfe38c);
impl windows_core::RuntimeType for IInkAnalyzer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkAnalyzer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AnalysisRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsAnalyzing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AddDataForStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddDataForStrokes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddDataForStrokes: usize,
    pub ClearDataForAllStrokes: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveDataForStroke: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveDataForStrokes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveDataForStrokes: usize,
    pub ReplaceDataForStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStrokeDataKind: unsafe extern "system" fn(*mut core::ffi::c_void, u32, InkAnalysisStrokeKind) -> windows_core::HRESULT,
    pub AnalyzeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkAnalyzerFactory, IInkAnalyzerFactory_Vtbl, 0x29138686_1963_49d8_9589_e14384c769e3);
impl core::ops::Deref for IInkAnalyzerFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkAnalyzerFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IInkAnalyzerFactory {
    pub fn CreateAnalyzer(&self) -> windows_core::Result<InkAnalyzer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAnalyzer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IInkAnalyzerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkAnalyzerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateAnalyzer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkAnalysisInkBullet(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkAnalysisInkBullet, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkAnalysisInkBullet, IInkAnalysisNode);
impl InkAnalysisInkBullet {
    pub fn RecognizedText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizedText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<InkAnalysisNodeKind> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotatedBoundingRect)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parent(&self) -> windows_core::Result<IInkAnalysisNode> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokeIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkAnalysisInkBullet {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkAnalysisInkBullet>();
}
unsafe impl windows_core::Interface for InkAnalysisInkBullet {
    type Vtable = IInkAnalysisInkBullet_Vtbl;
    const IID: windows_core::GUID = <IInkAnalysisInkBullet as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkAnalysisInkBullet {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet";
}
unsafe impl Send for InkAnalysisInkBullet {}
unsafe impl Sync for InkAnalysisInkBullet {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkAnalysisInkDrawing(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkAnalysisInkDrawing, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkAnalysisInkDrawing, IInkAnalysisNode);
impl InkAnalysisInkDrawing {
    pub fn DrawingKind(&self) -> windows_core::Result<InkAnalysisDrawingKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DrawingKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Center(&self) -> windows_core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Center)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Points(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Points)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<InkAnalysisNodeKind> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotatedBoundingRect)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parent(&self) -> windows_core::Result<IInkAnalysisNode> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokeIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkAnalysisInkDrawing {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkAnalysisInkDrawing>();
}
unsafe impl windows_core::Interface for InkAnalysisInkDrawing {
    type Vtable = IInkAnalysisInkDrawing_Vtbl;
    const IID: windows_core::GUID = <IInkAnalysisInkDrawing as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkAnalysisInkDrawing {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing";
}
unsafe impl Send for InkAnalysisInkDrawing {}
unsafe impl Sync for InkAnalysisInkDrawing {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkAnalysisInkWord(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkAnalysisInkWord, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkAnalysisInkWord, IInkAnalysisNode);
impl InkAnalysisInkWord {
    pub fn RecognizedText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizedText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TextAlternates(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextAlternates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<InkAnalysisNodeKind> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotatedBoundingRect)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parent(&self) -> windows_core::Result<IInkAnalysisNode> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokeIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkAnalysisInkWord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkAnalysisInkWord>();
}
unsafe impl windows_core::Interface for InkAnalysisInkWord {
    type Vtable = IInkAnalysisInkWord_Vtbl;
    const IID: windows_core::GUID = <IInkAnalysisInkWord as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkAnalysisInkWord {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord";
}
unsafe impl Send for InkAnalysisInkWord {}
unsafe impl Sync for InkAnalysisInkWord {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkAnalysisLine(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkAnalysisLine, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkAnalysisLine, IInkAnalysisNode);
impl InkAnalysisLine {
    pub fn RecognizedText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizedText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndentLevel(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndentLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<InkAnalysisNodeKind> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotatedBoundingRect)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parent(&self) -> windows_core::Result<IInkAnalysisNode> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokeIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkAnalysisLine {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkAnalysisLine>();
}
unsafe impl windows_core::Interface for InkAnalysisLine {
    type Vtable = IInkAnalysisLine_Vtbl;
    const IID: windows_core::GUID = <IInkAnalysisLine as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkAnalysisLine {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisLine";
}
unsafe impl Send for InkAnalysisLine {}
unsafe impl Sync for InkAnalysisLine {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkAnalysisListItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkAnalysisListItem, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkAnalysisListItem, IInkAnalysisNode);
impl InkAnalysisListItem {
    pub fn RecognizedText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizedText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<InkAnalysisNodeKind> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotatedBoundingRect)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parent(&self) -> windows_core::Result<IInkAnalysisNode> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokeIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkAnalysisListItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkAnalysisListItem>();
}
unsafe impl windows_core::Interface for InkAnalysisListItem {
    type Vtable = IInkAnalysisListItem_Vtbl;
    const IID: windows_core::GUID = <IInkAnalysisListItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkAnalysisListItem {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisListItem";
}
unsafe impl Send for InkAnalysisListItem {}
unsafe impl Sync for InkAnalysisListItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkAnalysisNode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkAnalysisNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkAnalysisNode, IInkAnalysisNode);
impl InkAnalysisNode {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<InkAnalysisNodeKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotatedBoundingRect)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parent(&self) -> windows_core::Result<IInkAnalysisNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokeIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkAnalysisNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkAnalysisNode>();
}
unsafe impl windows_core::Interface for InkAnalysisNode {
    type Vtable = IInkAnalysisNode_Vtbl;
    const IID: windows_core::GUID = <IInkAnalysisNode as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkAnalysisNode {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisNode";
}
unsafe impl Send for InkAnalysisNode {}
unsafe impl Sync for InkAnalysisNode {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkAnalysisParagraph(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkAnalysisParagraph, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkAnalysisParagraph, IInkAnalysisNode);
impl InkAnalysisParagraph {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<InkAnalysisNodeKind> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotatedBoundingRect)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parent(&self) -> windows_core::Result<IInkAnalysisNode> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokeIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RecognizedText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizedText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkAnalysisParagraph {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkAnalysisParagraph>();
}
unsafe impl windows_core::Interface for InkAnalysisParagraph {
    type Vtable = IInkAnalysisParagraph_Vtbl;
    const IID: windows_core::GUID = <IInkAnalysisParagraph as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkAnalysisParagraph {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph";
}
unsafe impl Send for InkAnalysisParagraph {}
unsafe impl Sync for InkAnalysisParagraph {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkAnalysisResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkAnalysisResult, windows_core::IUnknown, windows_core::IInspectable);
impl InkAnalysisResult {
    pub fn Status(&self) -> windows_core::Result<InkAnalysisStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InkAnalysisResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkAnalysisResult>();
}
unsafe impl windows_core::Interface for InkAnalysisResult {
    type Vtable = IInkAnalysisResult_Vtbl;
    const IID: windows_core::GUID = <IInkAnalysisResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkAnalysisResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisResult";
}
unsafe impl Send for InkAnalysisResult {}
unsafe impl Sync for InkAnalysisResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkAnalysisRoot(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkAnalysisRoot, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkAnalysisRoot, IInkAnalysisNode);
impl InkAnalysisRoot {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<InkAnalysisNodeKind> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotatedBoundingRect)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parent(&self) -> windows_core::Result<IInkAnalysisNode> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokeIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RecognizedText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizedText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindNodes(&self, nodekind: InkAnalysisNodeKind) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindNodes)(windows_core::Interface::as_raw(this), nodekind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkAnalysisRoot {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkAnalysisRoot>();
}
unsafe impl windows_core::Interface for InkAnalysisRoot {
    type Vtable = IInkAnalysisRoot_Vtbl;
    const IID: windows_core::GUID = <IInkAnalysisRoot as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkAnalysisRoot {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisRoot";
}
unsafe impl Send for InkAnalysisRoot {}
unsafe impl Sync for InkAnalysisRoot {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkAnalysisWritingRegion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkAnalysisWritingRegion, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkAnalysisWritingRegion, IInkAnalysisNode);
impl InkAnalysisWritingRegion {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<InkAnalysisNodeKind> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotatedBoundingRect)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parent(&self) -> windows_core::Result<IInkAnalysisNode> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokeIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RecognizedText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizedText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkAnalysisWritingRegion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkAnalysisWritingRegion>();
}
unsafe impl windows_core::Interface for InkAnalysisWritingRegion {
    type Vtable = IInkAnalysisWritingRegion_Vtbl;
    const IID: windows_core::GUID = <IInkAnalysisWritingRegion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkAnalysisWritingRegion {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion";
}
unsafe impl Send for InkAnalysisWritingRegion {}
unsafe impl Sync for InkAnalysisWritingRegion {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkAnalyzer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkAnalyzer, windows_core::IUnknown, windows_core::IInspectable);
impl InkAnalyzer {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InkAnalyzer, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AnalysisRoot(&self) -> windows_core::Result<InkAnalysisRoot> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AnalysisRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsAnalyzing(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAnalyzing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AddDataForStroke<P0>(&self, stroke: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::InkStroke>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDataForStroke)(windows_core::Interface::as_raw(this), stroke.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddDataForStrokes<P0>(&self, strokes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Collections::IIterable<super::InkStroke>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDataForStrokes)(windows_core::Interface::as_raw(this), strokes.param().abi()).ok() }
    }
    pub fn ClearDataForAllStrokes(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearDataForAllStrokes)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RemoveDataForStroke(&self, strokeid: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDataForStroke)(windows_core::Interface::as_raw(this), strokeid).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveDataForStrokes<P0>(&self, strokeids: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Collections::IIterable<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDataForStrokes)(windows_core::Interface::as_raw(this), strokeids.param().abi()).ok() }
    }
    pub fn ReplaceDataForStroke<P0>(&self, stroke: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::InkStroke>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReplaceDataForStroke)(windows_core::Interface::as_raw(this), stroke.param().abi()).ok() }
    }
    pub fn SetStrokeDataKind(&self, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStrokeDataKind)(windows_core::Interface::as_raw(this), strokeid, strokekind).ok() }
    }
    pub fn AnalyzeAsync(&self) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<InkAnalysisResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AnalyzeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkAnalyzer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkAnalyzer>();
}
unsafe impl windows_core::Interface for InkAnalyzer {
    type Vtable = IInkAnalyzer_Vtbl;
    const IID: windows_core::GUID = <IInkAnalyzer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkAnalyzer {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalyzer";
}
unsafe impl Send for InkAnalyzer {}
unsafe impl Sync for InkAnalyzer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkAnalysisDrawingKind(pub i32);
impl InkAnalysisDrawingKind {
    pub const Drawing: Self = Self(0i32);
    pub const Circle: Self = Self(1i32);
    pub const Ellipse: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
    pub const IsoscelesTriangle: Self = Self(4i32);
    pub const EquilateralTriangle: Self = Self(5i32);
    pub const RightTriangle: Self = Self(6i32);
    pub const Quadrilateral: Self = Self(7i32);
    pub const Rectangle: Self = Self(8i32);
    pub const Square: Self = Self(9i32);
    pub const Diamond: Self = Self(10i32);
    pub const Trapezoid: Self = Self(11i32);
    pub const Parallelogram: Self = Self(12i32);
    pub const Pentagon: Self = Self(13i32);
    pub const Hexagon: Self = Self(14i32);
}
impl windows_core::TypeKind for InkAnalysisDrawingKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkAnalysisDrawingKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkAnalysisDrawingKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkAnalysisDrawingKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisDrawingKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkAnalysisNodeKind(pub i32);
impl InkAnalysisNodeKind {
    pub const UnclassifiedInk: Self = Self(0i32);
    pub const Root: Self = Self(1i32);
    pub const WritingRegion: Self = Self(2i32);
    pub const Paragraph: Self = Self(3i32);
    pub const Line: Self = Self(4i32);
    pub const InkWord: Self = Self(5i32);
    pub const InkBullet: Self = Self(6i32);
    pub const InkDrawing: Self = Self(7i32);
    pub const ListItem: Self = Self(8i32);
}
impl windows_core::TypeKind for InkAnalysisNodeKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkAnalysisNodeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkAnalysisNodeKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkAnalysisNodeKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisNodeKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkAnalysisStatus(pub i32);
impl InkAnalysisStatus {
    pub const Updated: Self = Self(0i32);
    pub const Unchanged: Self = Self(1i32);
}
impl windows_core::TypeKind for InkAnalysisStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkAnalysisStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkAnalysisStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkAnalysisStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkAnalysisStrokeKind(pub i32);
impl InkAnalysisStrokeKind {
    pub const Auto: Self = Self(0i32);
    pub const Writing: Self = Self(1i32);
    pub const Drawing: Self = Self(2i32);
}
impl windows_core::TypeKind for InkAnalysisStrokeKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkAnalysisStrokeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkAnalysisStrokeKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkAnalysisStrokeKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisStrokeKind;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
