windows_core::imp::define_interface!(IVariablePhotoCapturedEventArgs, IVariablePhotoCapturedEventArgs_Vtbl, 0xd1eb4c5c_1b53_4e4a_8b5c_db7887ac949b);
impl windows_core::RuntimeType for IVariablePhotoCapturedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVariablePhotoCapturedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CaptureTimeOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub UsedFrameControllerIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CapturedFrameControlValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVariablePhotoSequenceCapture, IVariablePhotoSequenceCapture_Vtbl, 0xd0112d1d_031e_4041_a6d6_bd742476a8ee);
impl windows_core::RuntimeType for IVariablePhotoSequenceCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVariablePhotoSequenceCapture_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinishAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PhotoCaptured: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePhotoCaptured: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVariablePhotoSequenceCapture2, IVariablePhotoSequenceCapture2_Vtbl, 0xfe2c62bc_50b0_43e3_917c_e3b92798942f);
impl windows_core::RuntimeType for IVariablePhotoSequenceCapture2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVariablePhotoSequenceCapture2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UpdateSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VariablePhotoCapturedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VariablePhotoCapturedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl VariablePhotoCapturedEventArgs {
    pub fn Frame(&self) -> windows_core::Result<super::CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Frame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CaptureTimeOffset(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureTimeOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsedFrameControllerIndex(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsedFrameControllerIndex)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CapturedFrameControlValues(&self) -> windows_core::Result<super::CapturedFrameControlValues> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturedFrameControlValues)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VariablePhotoCapturedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVariablePhotoCapturedEventArgs>();
}
unsafe impl windows_core::Interface for VariablePhotoCapturedEventArgs {
    type Vtable = IVariablePhotoCapturedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IVariablePhotoCapturedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VariablePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs";
}
unsafe impl Send for VariablePhotoCapturedEventArgs {}
unsafe impl Sync for VariablePhotoCapturedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VariablePhotoSequenceCapture(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VariablePhotoSequenceCapture, windows_core::IUnknown, windows_core::IInspectable);
impl VariablePhotoSequenceCapture {
    pub fn StartAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FinishAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FinishAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhotoCaptured<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, VariablePhotoCapturedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotoCaptured)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePhotoCaptured(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePhotoCaptured)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Stopped<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stopped)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStopped(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStopped)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UpdateSettingsAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IVariablePhotoSequenceCapture2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateSettingsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VariablePhotoSequenceCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVariablePhotoSequenceCapture>();
}
unsafe impl windows_core::Interface for VariablePhotoSequenceCapture {
    type Vtable = IVariablePhotoSequenceCapture_Vtbl;
    const IID: windows_core::GUID = <IVariablePhotoSequenceCapture as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VariablePhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.Core.VariablePhotoSequenceCapture";
}
