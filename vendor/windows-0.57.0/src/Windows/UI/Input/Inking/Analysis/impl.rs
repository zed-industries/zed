#[cfg(feature = "Foundation_Collections")]
pub trait IInkAnalysisNode_Impl: Sized {
    fn Id(&self) -> windows_core::Result<u32>;
    fn Kind(&self) -> windows_core::Result<InkAnalysisNodeKind>;
    fn BoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect>;
    fn RotatedBoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>;
    fn Children(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>;
    fn Parent(&self) -> windows_core::Result<IInkAnalysisNode>;
    fn GetStrokeIds(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IInkAnalysisNode {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalysisNode";
}
#[cfg(feature = "Foundation_Collections")]
impl IInkAnalysisNode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>() -> IInkAnalysisNode_Vtbl {
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IInkAnalysisNode_Impl::Id(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut InkAnalysisNodeKind) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IInkAnalysisNode_Impl::Kind(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IInkAnalysisNode_Impl::BoundingRect(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotatedBoundingRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IInkAnalysisNode_Impl::RotatedBoundingRect(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IInkAnalysisNode_Impl::Children(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IInkAnalysisNode_Impl::Parent(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrokeIds<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IInkAnalysisNode_Impl::GetStrokeIds(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInkAnalysisNode, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            Kind: Kind::<Identity, Impl, OFFSET>,
            BoundingRect: BoundingRect::<Identity, Impl, OFFSET>,
            RotatedBoundingRect: RotatedBoundingRect::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            GetStrokeIds: GetStrokeIds::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkAnalysisNode as windows_core::Interface>::IID
    }
}
pub trait IInkAnalyzerFactory_Impl: Sized {
    fn CreateAnalyzer(&self) -> windows_core::Result<InkAnalyzer>;
}
impl windows_core::RuntimeName for IInkAnalyzerFactory {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.IInkAnalyzerFactory";
}
impl IInkAnalyzerFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalyzerFactory_Impl, const OFFSET: isize>() -> IInkAnalyzerFactory_Vtbl {
        unsafe extern "system" fn CreateAnalyzer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInkAnalyzerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IInkAnalyzerFactory_Impl::CreateAnalyzer(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInkAnalyzerFactory, OFFSET>(),
            CreateAnalyzer: CreateAnalyzer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInkAnalyzerFactory as windows_core::Interface>::IID
    }
}
