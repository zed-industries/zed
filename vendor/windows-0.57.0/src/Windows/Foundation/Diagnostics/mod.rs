windows_core::imp::define_interface!(IAsyncCausalityTracerStatics, IAsyncCausalityTracerStatics_Vtbl, 0x50850b26_267e_451b_a890_ab6a370245ee);
impl windows_core::RuntimeType for IAsyncCausalityTracerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAsyncCausalityTracerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TraceOperationCreation: unsafe extern "system" fn(*mut core::ffi::c_void, CausalityTraceLevel, CausalitySource, windows_core::GUID, u64, core::mem::MaybeUninit<windows_core::HSTRING>, u64) -> windows_core::HRESULT,
    pub TraceOperationCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, CausalityTraceLevel, CausalitySource, windows_core::GUID, u64, super::AsyncStatus) -> windows_core::HRESULT,
    pub TraceOperationRelation: unsafe extern "system" fn(*mut core::ffi::c_void, CausalityTraceLevel, CausalitySource, windows_core::GUID, u64, CausalityRelation) -> windows_core::HRESULT,
    pub TraceSynchronousWorkStart: unsafe extern "system" fn(*mut core::ffi::c_void, CausalityTraceLevel, CausalitySource, windows_core::GUID, u64, CausalitySynchronousWork) -> windows_core::HRESULT,
    pub TraceSynchronousWorkCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, CausalityTraceLevel, CausalitySource, CausalitySynchronousWork) -> windows_core::HRESULT,
    pub TracingStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTracingStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IErrorDetails, IErrorDetails_Vtbl, 0x378cbb01_2cc9_428f_8c55_2c990d463e8f);
impl windows_core::RuntimeType for IErrorDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IErrorDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LongDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HelpUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IErrorDetailsStatics, IErrorDetailsStatics_Vtbl, 0xb7703750_0b1d_46c8_aa0e_4b8178e4fce9);
impl windows_core::RuntimeType for IErrorDetailsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IErrorDetailsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromHResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IErrorReportingSettings, IErrorReportingSettings_Vtbl, 0x16369792_b03e_4ba1_8bb8_d28f4ab4d2c0);
impl core::ops::Deref for IErrorReportingSettings {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IErrorReportingSettings, windows_core::IUnknown, windows_core::IInspectable);
impl IErrorReportingSettings {
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetErrorOptions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetErrorOptions(&self) -> windows_core::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetErrorOptions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IErrorReportingSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IErrorReportingSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetErrorOptions: unsafe extern "system" fn(*mut core::ffi::c_void, ErrorOptions) -> windows_core::HRESULT,
    pub GetErrorOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ErrorOptions) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileLoggingSession, IFileLoggingSession_Vtbl, 0x24c74216_fed2_404c_895f_1f9699cb02f7);
impl core::ops::Deref for IFileLoggingSession {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFileLoggingSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IFileLoggingSession, super::IClosable);
impl IFileLoggingSession {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddLoggingChannel<P0>(&self, loggingchannel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddLoggingChannel)(windows_core::Interface::as_raw(this), loggingchannel.param().abi()).ok() }
    }
    pub fn AddLoggingChannelWithLevel<P0>(&self, loggingchannel: P0, maxlevel: LoggingLevel) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddLoggingChannelWithLevel)(windows_core::Interface::as_raw(this), loggingchannel.param().abi(), maxlevel).ok() }
    }
    pub fn RemoveLoggingChannel<P0>(&self, loggingchannel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLoggingChannel)(windows_core::Interface::as_raw(this), loggingchannel.param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn CloseAndSaveToFileAsync(&self) -> windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloseAndSaveToFileAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LogFileGenerated<P0>(&self, handler: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LogFileGenerated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLogFileGenerated(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLogFileGenerated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IFileLoggingSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFileLoggingSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AddLoggingChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddLoggingChannelWithLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LoggingLevel) -> windows_core::HRESULT,
    pub RemoveLoggingChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CloseAndSaveToFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CloseAndSaveToFileAsync: usize,
    pub LogFileGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLogFileGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, super::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileLoggingSessionFactory, IFileLoggingSessionFactory_Vtbl, 0xeea08dce_8447_4daa_9133_12eb46f697d4);
impl windows_core::RuntimeType for IFileLoggingSessionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFileLoggingSessionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILogFileGeneratedEventArgs, ILogFileGeneratedEventArgs_Vtbl, 0x269e976f_0d38_4c1a_b53f_b395d881df84);
impl windows_core::RuntimeType for ILogFileGeneratedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILogFileGeneratedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
windows_core::imp::define_interface!(ILoggingActivity, ILoggingActivity_Vtbl, 0xbc032941_b766_4cb5_9848_97ac6ba6d60c);
impl windows_core::RuntimeType for ILoggingActivity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingActivity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingActivity2, ILoggingActivity2_Vtbl, 0x26c29808_6322_456a_af82_80c8642f178b);
impl windows_core::RuntimeType for ILoggingActivity2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingActivity2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopActivity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub StopActivityWithFields: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopActivityWithFieldsAndOptions: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingActivityFactory, ILoggingActivityFactory_Vtbl, 0x6b33b483_e10a_4c58_97d5_10fb451074fb);
impl windows_core::RuntimeType for ILoggingActivityFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingActivityFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateLoggingActivity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLoggingActivityWithLevel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, LoggingLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingChannel, ILoggingChannel_Vtbl, 0xe9a50343_11d7_4f01_b5ca_cf495278c0a8);
impl core::ops::Deref for ILoggingChannel {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILoggingChannel, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ILoggingChannel, super::IClosable);
impl ILoggingChannel {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Enabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Enabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Level(&self) -> windows_core::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Level)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LogMessage(&self, eventstring: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventstring)).ok() }
    }
    pub fn LogMessageWithLevel(&self, eventstring: &windows_core::HSTRING, level: LoggingLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogMessageWithLevel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventstring), level).ok() }
    }
    pub fn LogValuePair(&self, value1: &windows_core::HSTRING, value2: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogValuePair)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value1), value2).ok() }
    }
    pub fn LogValuePairWithLevel(&self, value1: &windows_core::HSTRING, value2: i32, level: LoggingLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogValuePairWithLevel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value1), value2, level).ok() }
    }
    pub fn LoggingEnabled<P0>(&self, handler: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<super::TypedEventHandler<ILoggingChannel, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoggingEnabled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLoggingEnabled(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLoggingEnabled)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ILoggingChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingChannel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Level: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LoggingLevel) -> windows_core::HRESULT,
    pub LogMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LogMessageWithLevel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, LoggingLevel) -> windows_core::HRESULT,
    pub LogValuePair: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i32) -> windows_core::HRESULT,
    pub LogValuePairWithLevel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i32, LoggingLevel) -> windows_core::HRESULT,
    pub LoggingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLoggingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingChannel2, ILoggingChannel2_Vtbl, 0x9f4c3cf3_0bac_45a5_9e33_baf3f3a246a5);
impl windows_core::RuntimeType for ILoggingChannel2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingChannel2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingChannelFactory, ILoggingChannelFactory_Vtbl, 0x4edc5b9c_af80_4a9b_b0dc_398f9ae5207b);
impl windows_core::RuntimeType for ILoggingChannelFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingChannelFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
windows_core::imp::define_interface!(ILoggingChannelFactory2, ILoggingChannelFactory2_Vtbl, 0x4c6ef5dd_3b27_4dc9_99f0_299c6e4603a1);
impl windows_core::RuntimeType for ILoggingChannelFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingChannelFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithOptionsAndId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingChannelOptions, ILoggingChannelOptions_Vtbl, 0xc3e847ff_0ebb_4a53_8c54_dec24926cb2c);
impl windows_core::RuntimeType for ILoggingChannelOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingChannelOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingChannelOptionsFactory, ILoggingChannelOptionsFactory_Vtbl, 0xa93151da_7faf_4191_8755_5e86dc65d896);
impl windows_core::RuntimeType for ILoggingChannelOptionsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingChannelOptionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingFields, ILoggingFields_Vtbl, 0xd7f6b7af_762d_4579_83bd_52c23bc333bc);
impl windows_core::RuntimeType for ILoggingFields {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingFields_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginStruct: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub BeginStructWithTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i32) -> windows_core::HRESULT,
    pub EndStruct: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddEmpty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AddEmptyWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddEmptyWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddUInt8: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u8) -> windows_core::HRESULT,
    pub AddUInt8WithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u8, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddUInt8WithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u8, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddUInt8Array: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u8) -> windows_core::HRESULT,
    pub AddUInt8ArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u8, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddUInt8ArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u8, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddInt16: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i16) -> windows_core::HRESULT,
    pub AddInt16WithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i16, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddInt16WithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i16, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddInt16Array: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const i16) -> windows_core::HRESULT,
    pub AddInt16ArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const i16, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddInt16ArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const i16, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddUInt16: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u16) -> windows_core::HRESULT,
    pub AddUInt16WithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u16, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddUInt16WithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u16, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddUInt16Array: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u16) -> windows_core::HRESULT,
    pub AddUInt16ArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u16, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddUInt16ArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u16, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddInt32: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i32) -> windows_core::HRESULT,
    pub AddInt32WithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i32, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddInt32WithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i32, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddInt32Array: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const i32) -> windows_core::HRESULT,
    pub AddInt32ArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const i32, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddInt32ArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const i32, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddUInt32: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32) -> windows_core::HRESULT,
    pub AddUInt32WithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddUInt32WithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddUInt32Array: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u32) -> windows_core::HRESULT,
    pub AddUInt32ArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u32, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddUInt32ArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u32, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddInt64: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i64) -> windows_core::HRESULT,
    pub AddInt64WithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i64, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddInt64WithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i64, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddInt64Array: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const i64) -> windows_core::HRESULT,
    pub AddInt64ArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const i64, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddInt64ArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const i64, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddUInt64: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u64) -> windows_core::HRESULT,
    pub AddUInt64WithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u64, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddUInt64WithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u64, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddUInt64Array: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u64) -> windows_core::HRESULT,
    pub AddUInt64ArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u64, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddUInt64ArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u64, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddSingle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, f32) -> windows_core::HRESULT,
    pub AddSingleWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, f32, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddSingleWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, f32, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddSingleArray: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const f32) -> windows_core::HRESULT,
    pub AddSingleArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const f32, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddSingleArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const f32, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddDouble: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, f64) -> windows_core::HRESULT,
    pub AddDoubleWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, f64, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddDoubleWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, f64, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddDoubleArray: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const f64) -> windows_core::HRESULT,
    pub AddDoubleArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const f64, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddDoubleArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const f64, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddChar16: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u16) -> windows_core::HRESULT,
    pub AddChar16WithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u16, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddChar16WithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u16, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddChar16Array: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u16) -> windows_core::HRESULT,
    pub AddChar16ArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u16, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddChar16ArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const u16, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, bool) -> windows_core::HRESULT,
    pub AddBooleanWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, bool, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddBooleanWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, bool, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddBooleanArray: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const bool) -> windows_core::HRESULT,
    pub AddBooleanArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const bool, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddBooleanArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const bool, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AddStringWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddStringWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddStringArray: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AddStringArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const core::mem::MaybeUninit<windows_core::HSTRING>, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddStringArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const core::mem::MaybeUninit<windows_core::HSTRING>, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddGuid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID) -> windows_core::HRESULT,
    pub AddGuidWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddGuidWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddGuidArray: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub AddGuidArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const windows_core::GUID, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddGuidArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const windows_core::GUID, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::DateTime) -> windows_core::HRESULT,
    pub AddDateTimeWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::DateTime, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddDateTimeWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::DateTime, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddDateTimeArray: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::DateTime) -> windows_core::HRESULT,
    pub AddDateTimeArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::DateTime, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddDateTimeArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::DateTime, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddTimeSpan: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::TimeSpan) -> windows_core::HRESULT,
    pub AddTimeSpanWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::TimeSpan, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddTimeSpanWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::TimeSpan, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddTimeSpanArray: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::TimeSpan) -> windows_core::HRESULT,
    pub AddTimeSpanArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::TimeSpan, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddTimeSpanArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::TimeSpan, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddPoint: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Point) -> windows_core::HRESULT,
    pub AddPointWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Point, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddPointWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Point, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddPointArray: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::Point) -> windows_core::HRESULT,
    pub AddPointArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::Point, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddPointArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::Point, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddSize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Size) -> windows_core::HRESULT,
    pub AddSizeWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Size, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddSizeWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Size, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddSizeArray: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::Size) -> windows_core::HRESULT,
    pub AddSizeArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::Size, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddSizeArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::Size, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddRect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Rect) -> windows_core::HRESULT,
    pub AddRectWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Rect, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddRectWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Rect, LoggingFieldFormat, i32) -> windows_core::HRESULT,
    pub AddRectArray: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::Rect) -> windows_core::HRESULT,
    pub AddRectArrayWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::Rect, LoggingFieldFormat) -> windows_core::HRESULT,
    pub AddRectArrayWithFormatAndTags: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *const super::Rect, LoggingFieldFormat, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingOptions, ILoggingOptions_Vtbl, 0x90bc7850_0192_4f5d_ac26_006adaca12d8);
impl windows_core::RuntimeType for ILoggingOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Keywords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub SetKeywords: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Tags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub Opcode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LoggingOpcode) -> windows_core::HRESULT,
    pub SetOpcode: unsafe extern "system" fn(*mut core::ffi::c_void, LoggingOpcode) -> windows_core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub RelatedActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetRelatedActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingOptionsFactory, ILoggingOptionsFactory_Vtbl, 0xd713c6cb_98ab_464b_9f22_a3268478368a);
impl windows_core::RuntimeType for ILoggingOptionsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingOptionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithKeywords: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingSession, ILoggingSession_Vtbl, 0x6221f306_9380_4ad7_baf5_41ea9310d768);
impl core::ops::Deref for ILoggingSession {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILoggingSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ILoggingSession, super::IClosable);
impl ILoggingSession {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SaveToFileAsync<P0>(&self, folder: P0, filename: &windows_core::HSTRING) -> windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveToFileAsync)(windows_core::Interface::as_raw(this), folder.param().abi(), core::mem::transmute_copy(filename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddLoggingChannel<P0>(&self, loggingchannel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddLoggingChannel)(windows_core::Interface::as_raw(this), loggingchannel.param().abi()).ok() }
    }
    pub fn AddLoggingChannelWithLevel<P0>(&self, loggingchannel: P0, maxlevel: LoggingLevel) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddLoggingChannelWithLevel)(windows_core::Interface::as_raw(this), loggingchannel.param().abi(), maxlevel).ok() }
    }
    pub fn RemoveLoggingChannel<P0>(&self, loggingchannel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLoggingChannel)(windows_core::Interface::as_raw(this), loggingchannel.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ILoggingSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SaveToFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SaveToFileAsync: usize,
    pub AddLoggingChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddLoggingChannelWithLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LoggingLevel) -> windows_core::HRESULT,
    pub RemoveLoggingChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingSessionFactory, ILoggingSessionFactory_Vtbl, 0x4e937ee5_58fd_45e0_8c2f_a132eff95c1e);
impl windows_core::RuntimeType for ILoggingSessionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingSessionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILoggingTarget, ILoggingTarget_Vtbl, 0x65f16c35_e388_4e26_b17a_f51cd3a83916);
impl core::ops::Deref for ILoggingTarget {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILoggingTarget, windows_core::IUnknown, windows_core::IInspectable);
impl ILoggingTarget {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabledWithLevel)(windows_core::Interface::as_raw(this), level, &mut result__).map(|| result__)
        }
    }
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabledWithLevelAndKeywords)(windows_core::Interface::as_raw(this), level, keywords, &mut result__).map(|| result__)
        }
    }
    pub fn LogEvent(&self, eventname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogEvent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventname)).ok() }
    }
    pub fn LogEventWithFields<P0>(&self, eventname: &windows_core::HSTRING, fields: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogEventWithFields)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventname), fields.param().abi()).ok() }
    }
    pub fn LogEventWithFieldsAndLevel<P0>(&self, eventname: &windows_core::HSTRING, fields: P0, level: LoggingLevel) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogEventWithFieldsAndLevel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventname), fields.param().abi(), level).ok() }
    }
    pub fn LogEventWithFieldsAndOptions<P0, P1>(&self, eventname: &windows_core::HSTRING, fields: P0, level: LoggingLevel, options: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LoggingFields>,
        P1: windows_core::Param<LoggingOptions>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogEventWithFieldsAndOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventname), fields.param().abi(), level, options.param().abi()).ok() }
    }
    pub fn StartActivity(&self, starteventname: &windows_core::HSTRING) -> windows_core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartActivity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(starteventname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartActivityWithFields<P0>(&self, starteventname: &windows_core::HSTRING, fields: P0) -> windows_core::Result<LoggingActivity>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartActivityWithFields)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(starteventname), fields.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartActivityWithFieldsAndLevel<P0>(&self, starteventname: &windows_core::HSTRING, fields: P0, level: LoggingLevel) -> windows_core::Result<LoggingActivity>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartActivityWithFieldsAndLevel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(starteventname), fields.param().abi(), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartActivityWithFieldsAndOptions<P0, P1>(&self, starteventname: &windows_core::HSTRING, fields: P0, level: LoggingLevel, options: P1) -> windows_core::Result<LoggingActivity>
    where
        P0: windows_core::Param<LoggingFields>,
        P1: windows_core::Param<LoggingOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartActivityWithFieldsAndOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(starteventname), fields.param().abi(), level, options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ILoggingTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILoggingTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsEnabledWithLevel: unsafe extern "system" fn(*mut core::ffi::c_void, LoggingLevel, *mut bool) -> windows_core::HRESULT,
    pub IsEnabledWithLevelAndKeywords: unsafe extern "system" fn(*mut core::ffi::c_void, LoggingLevel, i64, *mut bool) -> windows_core::HRESULT,
    pub LogEvent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LogEventWithFields: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogEventWithFieldsAndLevel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, LoggingLevel) -> windows_core::HRESULT,
    pub LogEventWithFieldsAndOptions: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, LoggingLevel, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartActivity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartActivityWithFields: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartActivityWithFieldsAndLevel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, LoggingLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartActivityWithFieldsAndOptions: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, LoggingLevel, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITracingStatusChangedEventArgs, ITracingStatusChangedEventArgs_Vtbl, 0x410b7711_ff3b_477f_9c9a_d2efda302dc3);
impl windows_core::RuntimeType for ITracingStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITracingStatusChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TraceLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CausalityTraceLevel) -> windows_core::HRESULT,
}
pub struct AsyncCausalityTracer;
impl AsyncCausalityTracer {
    pub fn TraceOperationCreation(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: windows_core::GUID, operationid: u64, operationname: &windows_core::HSTRING, relatedcontext: u64) -> windows_core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (windows_core::Interface::vtable(this).TraceOperationCreation)(windows_core::Interface::as_raw(this), tracelevel, source, platformid, operationid, core::mem::transmute_copy(operationname), relatedcontext).ok() })
    }
    pub fn TraceOperationCompletion(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: windows_core::GUID, operationid: u64, status: super::AsyncStatus) -> windows_core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (windows_core::Interface::vtable(this).TraceOperationCompletion)(windows_core::Interface::as_raw(this), tracelevel, source, platformid, operationid, status).ok() })
    }
    pub fn TraceOperationRelation(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: windows_core::GUID, operationid: u64, relation: CausalityRelation) -> windows_core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (windows_core::Interface::vtable(this).TraceOperationRelation)(windows_core::Interface::as_raw(this), tracelevel, source, platformid, operationid, relation).ok() })
    }
    pub fn TraceSynchronousWorkStart(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: windows_core::GUID, operationid: u64, work: CausalitySynchronousWork) -> windows_core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (windows_core::Interface::vtable(this).TraceSynchronousWorkStart)(windows_core::Interface::as_raw(this), tracelevel, source, platformid, operationid, work).ok() })
    }
    pub fn TraceSynchronousWorkCompletion(tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> windows_core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (windows_core::Interface::vtable(this).TraceSynchronousWorkCompletion)(windows_core::Interface::as_raw(this), tracelevel, source, work).ok() })
    }
    pub fn TracingStatusChanged<P0>(handler: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<super::EventHandler<TracingStatusChangedEventArgs>>,
    {
        Self::IAsyncCausalityTracerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TracingStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveTracingStatusChanged(cookie: super::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveTracingStatusChanged)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc(hidden)]
    pub fn IAsyncCausalityTracerStatics<R, F: FnOnce(&IAsyncCausalityTracerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AsyncCausalityTracer, IAsyncCausalityTracerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for AsyncCausalityTracer {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.AsyncCausalityTracer";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ErrorDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ErrorDetails, windows_core::IUnknown, windows_core::IInspectable);
impl ErrorDetails {
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LongDescription(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LongDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HelpUri(&self) -> windows_core::Result<super::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HelpUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFromHResultAsync(errorcode: i32) -> windows_core::Result<super::IAsyncOperation<ErrorDetails>> {
        Self::IErrorDetailsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromHResultAsync)(windows_core::Interface::as_raw(this), errorcode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IErrorDetailsStatics<R, F: FnOnce(&IErrorDetailsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ErrorDetails, IErrorDetailsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ErrorDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IErrorDetails>();
}
unsafe impl windows_core::Interface for ErrorDetails {
    type Vtable = IErrorDetails_Vtbl;
    const IID: windows_core::GUID = <IErrorDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ErrorDetails {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ErrorDetails";
}
unsafe impl Send for ErrorDetails {}
unsafe impl Sync for ErrorDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FileLoggingSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileLoggingSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(FileLoggingSession, super::IClosable, IFileLoggingSession);
impl FileLoggingSession {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddLoggingChannel<P0>(&self, loggingchannel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddLoggingChannel)(windows_core::Interface::as_raw(this), loggingchannel.param().abi()).ok() }
    }
    pub fn AddLoggingChannelWithLevel<P0>(&self, loggingchannel: P0, maxlevel: LoggingLevel) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddLoggingChannelWithLevel)(windows_core::Interface::as_raw(this), loggingchannel.param().abi(), maxlevel).ok() }
    }
    pub fn RemoveLoggingChannel<P0>(&self, loggingchannel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLoggingChannel)(windows_core::Interface::as_raw(this), loggingchannel.param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn CloseAndSaveToFileAsync(&self) -> windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloseAndSaveToFileAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LogFileGenerated<P0>(&self, handler: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LogFileGenerated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLogFileGenerated(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLogFileGenerated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Create(name: &windows_core::HSTRING) -> windows_core::Result<FileLoggingSession> {
        Self::IFileLoggingSessionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IFileLoggingSessionFactory<R, F: FnOnce(&IFileLoggingSessionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FileLoggingSession, IFileLoggingSessionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FileLoggingSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileLoggingSession>();
}
unsafe impl windows_core::Interface for FileLoggingSession {
    type Vtable = IFileLoggingSession_Vtbl;
    const IID: windows_core::GUID = <IFileLoggingSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.FileLoggingSession";
}
unsafe impl Send for FileLoggingSession {}
unsafe impl Sync for FileLoggingSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LogFileGeneratedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LogFileGeneratedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl LogFileGeneratedEventArgs {
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LogFileGeneratedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILogFileGeneratedEventArgs>();
}
unsafe impl windows_core::Interface for LogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ILogFileGeneratedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LogFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs";
}
unsafe impl Send for LogFileGeneratedEventArgs {}
unsafe impl Sync for LogFileGeneratedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LoggingActivity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LoggingActivity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LoggingActivity, super::IClosable, ILoggingTarget);
impl LoggingActivity {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Channel(&self) -> windows_core::Result<LoggingChannel> {
        let this = &windows_core::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Channel)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopActivity(&self, stopeventname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopActivity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(stopeventname)).ok() }
    }
    pub fn StopActivityWithFields<P0>(&self, stopeventname: &windows_core::HSTRING, fields: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = &windows_core::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopActivityWithFields)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(stopeventname), fields.param().abi()).ok() }
    }
    pub fn StopActivityWithFieldsAndOptions<P0, P1>(&self, stopeventname: &windows_core::HSTRING, fields: P0, options: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LoggingFields>,
        P1: windows_core::Param<LoggingOptions>,
    {
        let this = &windows_core::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopActivityWithFieldsAndOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(stopeventname), fields.param().abi(), options.param().abi()).ok() }
    }
    pub fn CreateLoggingActivity<P0>(activityname: &windows_core::HSTRING, loggingchannel: P0) -> windows_core::Result<LoggingActivity>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateLoggingActivity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activityname), loggingchannel.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateLoggingActivityWithLevel<P0>(activityname: &windows_core::HSTRING, loggingchannel: P0, level: LoggingLevel) -> windows_core::Result<LoggingActivity>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateLoggingActivityWithLevel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activityname), loggingchannel.param().abi(), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabledWithLevel)(windows_core::Interface::as_raw(this), level, &mut result__).map(|| result__)
        }
    }
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabledWithLevelAndKeywords)(windows_core::Interface::as_raw(this), level, keywords, &mut result__).map(|| result__)
        }
    }
    pub fn LogEvent(&self, eventname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (windows_core::Interface::vtable(this).LogEvent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventname)).ok() }
    }
    pub fn LogEventWithFields<P0>(&self, eventname: &windows_core::HSTRING, fields: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (windows_core::Interface::vtable(this).LogEventWithFields)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventname), fields.param().abi()).ok() }
    }
    pub fn LogEventWithFieldsAndLevel<P0>(&self, eventname: &windows_core::HSTRING, fields: P0, level: LoggingLevel) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (windows_core::Interface::vtable(this).LogEventWithFieldsAndLevel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventname), fields.param().abi(), level).ok() }
    }
    pub fn LogEventWithFieldsAndOptions<P0, P1>(&self, eventname: &windows_core::HSTRING, fields: P0, level: LoggingLevel, options: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LoggingFields>,
        P1: windows_core::Param<LoggingOptions>,
    {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (windows_core::Interface::vtable(this).LogEventWithFieldsAndOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventname), fields.param().abi(), level, options.param().abi()).ok() }
    }
    pub fn StartActivity(&self, starteventname: &windows_core::HSTRING) -> windows_core::Result<LoggingActivity> {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartActivity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(starteventname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartActivityWithFields<P0>(&self, starteventname: &windows_core::HSTRING, fields: P0) -> windows_core::Result<LoggingActivity>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartActivityWithFields)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(starteventname), fields.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartActivityWithFieldsAndLevel<P0>(&self, starteventname: &windows_core::HSTRING, fields: P0, level: LoggingLevel) -> windows_core::Result<LoggingActivity>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartActivityWithFieldsAndLevel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(starteventname), fields.param().abi(), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartActivityWithFieldsAndOptions<P0, P1>(&self, starteventname: &windows_core::HSTRING, fields: P0, level: LoggingLevel, options: P1) -> windows_core::Result<LoggingActivity>
    where
        P0: windows_core::Param<LoggingFields>,
        P1: windows_core::Param<LoggingOptions>,
    {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartActivityWithFieldsAndOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(starteventname), fields.param().abi(), level, options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn ILoggingActivityFactory<R, F: FnOnce(&ILoggingActivityFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LoggingActivity, ILoggingActivityFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LoggingActivity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILoggingActivity>();
}
unsafe impl windows_core::Interface for LoggingActivity {
    type Vtable = ILoggingActivity_Vtbl;
    const IID: windows_core::GUID = <ILoggingActivity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LoggingActivity {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingActivity";
}
unsafe impl Send for LoggingActivity {}
unsafe impl Sync for LoggingActivity {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LoggingChannel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LoggingChannel, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LoggingChannel, super::IClosable, ILoggingChannel, ILoggingTarget);
impl LoggingChannel {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Enabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Enabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Level(&self) -> windows_core::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Level)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LogMessage(&self, eventstring: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventstring)).ok() }
    }
    pub fn LogMessageWithLevel(&self, eventstring: &windows_core::HSTRING, level: LoggingLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogMessageWithLevel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventstring), level).ok() }
    }
    pub fn LogValuePair(&self, value1: &windows_core::HSTRING, value2: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogValuePair)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value1), value2).ok() }
    }
    pub fn LogValuePairWithLevel(&self, value1: &windows_core::HSTRING, value2: i32, level: LoggingLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogValuePairWithLevel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value1), value2, level).ok() }
    }
    pub fn LoggingEnabled<P0>(&self, handler: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<super::TypedEventHandler<ILoggingChannel, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoggingEnabled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLoggingEnabled(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLoggingEnabled)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<ILoggingChannel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create(name: &windows_core::HSTRING) -> windows_core::Result<LoggingChannel> {
        Self::ILoggingChannelFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithOptions<P0>(name: &windows_core::HSTRING, options: P0) -> windows_core::Result<LoggingChannel>
    where
        P0: windows_core::Param<LoggingChannelOptions>,
    {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithOptionsAndId<P0>(name: &windows_core::HSTRING, options: P0, id: windows_core::GUID) -> windows_core::Result<LoggingChannel>
    where
        P0: windows_core::Param<LoggingChannelOptions>,
    {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithOptionsAndId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), options.param().abi(), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabledWithLevel)(windows_core::Interface::as_raw(this), level, &mut result__).map(|| result__)
        }
    }
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabledWithLevelAndKeywords)(windows_core::Interface::as_raw(this), level, keywords, &mut result__).map(|| result__)
        }
    }
    pub fn LogEvent(&self, eventname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (windows_core::Interface::vtable(this).LogEvent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventname)).ok() }
    }
    pub fn LogEventWithFields<P0>(&self, eventname: &windows_core::HSTRING, fields: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (windows_core::Interface::vtable(this).LogEventWithFields)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventname), fields.param().abi()).ok() }
    }
    pub fn LogEventWithFieldsAndLevel<P0>(&self, eventname: &windows_core::HSTRING, fields: P0, level: LoggingLevel) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (windows_core::Interface::vtable(this).LogEventWithFieldsAndLevel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventname), fields.param().abi(), level).ok() }
    }
    pub fn LogEventWithFieldsAndOptions<P0, P1>(&self, eventname: &windows_core::HSTRING, fields: P0, level: LoggingLevel, options: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LoggingFields>,
        P1: windows_core::Param<LoggingOptions>,
    {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (windows_core::Interface::vtable(this).LogEventWithFieldsAndOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventname), fields.param().abi(), level, options.param().abi()).ok() }
    }
    pub fn StartActivity(&self, starteventname: &windows_core::HSTRING) -> windows_core::Result<LoggingActivity> {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartActivity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(starteventname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartActivityWithFields<P0>(&self, starteventname: &windows_core::HSTRING, fields: P0) -> windows_core::Result<LoggingActivity>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartActivityWithFields)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(starteventname), fields.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartActivityWithFieldsAndLevel<P0>(&self, starteventname: &windows_core::HSTRING, fields: P0, level: LoggingLevel) -> windows_core::Result<LoggingActivity>
    where
        P0: windows_core::Param<LoggingFields>,
    {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartActivityWithFieldsAndLevel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(starteventname), fields.param().abi(), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartActivityWithFieldsAndOptions<P0, P1>(&self, starteventname: &windows_core::HSTRING, fields: P0, level: LoggingLevel, options: P1) -> windows_core::Result<LoggingActivity>
    where
        P0: windows_core::Param<LoggingFields>,
        P1: windows_core::Param<LoggingOptions>,
    {
        let this = &windows_core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartActivityWithFieldsAndOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(starteventname), fields.param().abi(), level, options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn ILoggingChannelFactory<R, F: FnOnce(&ILoggingChannelFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LoggingChannel, ILoggingChannelFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILoggingChannelFactory2<R, F: FnOnce(&ILoggingChannelFactory2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LoggingChannel, ILoggingChannelFactory2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LoggingChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILoggingChannel>();
}
unsafe impl windows_core::Interface for LoggingChannel {
    type Vtable = ILoggingChannel_Vtbl;
    const IID: windows_core::GUID = <ILoggingChannel as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannel";
}
unsafe impl Send for LoggingChannel {}
unsafe impl Sync for LoggingChannel {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LoggingChannelOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LoggingChannelOptions, windows_core::IUnknown, windows_core::IInspectable);
impl LoggingChannelOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LoggingChannelOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Group(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Group)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGroup(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGroup)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(group: windows_core::GUID) -> windows_core::Result<LoggingChannelOptions> {
        Self::ILoggingChannelOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), group, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILoggingChannelOptionsFactory<R, F: FnOnce(&ILoggingChannelOptionsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LoggingChannelOptions, ILoggingChannelOptionsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LoggingChannelOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILoggingChannelOptions>();
}
unsafe impl windows_core::Interface for LoggingChannelOptions {
    type Vtable = ILoggingChannelOptions_Vtbl;
    const IID: windows_core::GUID = <ILoggingChannelOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LoggingChannelOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannelOptions";
}
unsafe impl Send for LoggingChannelOptions {}
unsafe impl Sync for LoggingChannelOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LoggingFields(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LoggingFields, windows_core::IUnknown, windows_core::IInspectable);
impl LoggingFields {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LoggingFields, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn BeginStruct(&self, name: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).BeginStruct)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name)).ok() }
    }
    pub fn BeginStructWithTags(&self, name: &windows_core::HSTRING, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).BeginStructWithTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), tags).ok() }
    }
    pub fn EndStruct(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).EndStruct)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AddEmpty(&self, name: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddEmpty)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name)).ok() }
    }
    pub fn AddEmptyWithFormat(&self, name: &windows_core::HSTRING, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddEmptyWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), format).ok() }
    }
    pub fn AddEmptyWithFormatAndTags(&self, name: &windows_core::HSTRING, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddEmptyWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), format, tags).ok() }
    }
    pub fn AddUInt8(&self, name: &windows_core::HSTRING, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt8)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddUInt8WithFormat(&self, name: &windows_core::HSTRING, value: u8, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt8WithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddUInt8WithFormatAndTags(&self, name: &windows_core::HSTRING, value: u8, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt8WithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddUInt8Array(&self, name: &windows_core::HSTRING, value: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt8Array)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddUInt8ArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[u8], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt8ArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddUInt8ArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[u8], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt8ArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddInt16(&self, name: &windows_core::HSTRING, value: i16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt16)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddInt16WithFormat(&self, name: &windows_core::HSTRING, value: i16, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt16WithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddInt16WithFormatAndTags(&self, name: &windows_core::HSTRING, value: i16, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt16WithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddInt16Array(&self, name: &windows_core::HSTRING, value: &[i16]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt16Array)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddInt16ArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[i16], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt16ArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddInt16ArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[i16], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt16ArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddUInt16(&self, name: &windows_core::HSTRING, value: u16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt16)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddUInt16WithFormat(&self, name: &windows_core::HSTRING, value: u16, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt16WithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddUInt16WithFormatAndTags(&self, name: &windows_core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt16WithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddUInt16Array(&self, name: &windows_core::HSTRING, value: &[u16]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt16Array)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddUInt16ArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[u16], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt16ArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddUInt16ArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[u16], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt16ArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddInt32(&self, name: &windows_core::HSTRING, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt32)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddInt32WithFormat(&self, name: &windows_core::HSTRING, value: i32, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt32WithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddInt32WithFormatAndTags(&self, name: &windows_core::HSTRING, value: i32, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt32WithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddInt32Array(&self, name: &windows_core::HSTRING, value: &[i32]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt32Array)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddInt32ArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[i32], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt32ArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddInt32ArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[i32], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt32ArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddUInt32(&self, name: &windows_core::HSTRING, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt32)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddUInt32WithFormat(&self, name: &windows_core::HSTRING, value: u32, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt32WithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddUInt32WithFormatAndTags(&self, name: &windows_core::HSTRING, value: u32, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt32WithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddUInt32Array(&self, name: &windows_core::HSTRING, value: &[u32]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt32Array)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddUInt32ArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[u32], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt32ArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddUInt32ArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[u32], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt32ArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddInt64(&self, name: &windows_core::HSTRING, value: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt64)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddInt64WithFormat(&self, name: &windows_core::HSTRING, value: i64, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt64WithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddInt64WithFormatAndTags(&self, name: &windows_core::HSTRING, value: i64, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt64WithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddInt64Array(&self, name: &windows_core::HSTRING, value: &[i64]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt64Array)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddInt64ArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[i64], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt64ArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddInt64ArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[i64], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt64ArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddUInt64(&self, name: &windows_core::HSTRING, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt64)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddUInt64WithFormat(&self, name: &windows_core::HSTRING, value: u64, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt64WithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddUInt64WithFormatAndTags(&self, name: &windows_core::HSTRING, value: u64, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt64WithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddUInt64Array(&self, name: &windows_core::HSTRING, value: &[u64]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt64Array)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddUInt64ArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[u64], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt64ArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddUInt64ArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[u64], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddUInt64ArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddSingle(&self, name: &windows_core::HSTRING, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddSingle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddSingleWithFormat(&self, name: &windows_core::HSTRING, value: f32, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddSingleWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddSingleWithFormatAndTags(&self, name: &windows_core::HSTRING, value: f32, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddSingleWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddSingleArray(&self, name: &windows_core::HSTRING, value: &[f32]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddSingleArray)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddSingleArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[f32], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddSingleArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddSingleArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[f32], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddSingleArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddDouble(&self, name: &windows_core::HSTRING, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDouble)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddDoubleWithFormat(&self, name: &windows_core::HSTRING, value: f64, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDoubleWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddDoubleWithFormatAndTags(&self, name: &windows_core::HSTRING, value: f64, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDoubleWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddDoubleArray(&self, name: &windows_core::HSTRING, value: &[f64]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDoubleArray)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddDoubleArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[f64], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDoubleArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddDoubleArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[f64], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDoubleArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddChar16(&self, name: &windows_core::HSTRING, value: u16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddChar16)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddChar16WithFormat(&self, name: &windows_core::HSTRING, value: u16, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddChar16WithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddChar16WithFormatAndTags(&self, name: &windows_core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddChar16WithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddChar16Array(&self, name: &windows_core::HSTRING, value: &[u16]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddChar16Array)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddChar16ArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[u16], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddChar16ArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddChar16ArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[u16], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddChar16ArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddBoolean(&self, name: &windows_core::HSTRING, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddBoolean)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddBooleanWithFormat(&self, name: &windows_core::HSTRING, value: bool, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddBooleanWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddBooleanWithFormatAndTags(&self, name: &windows_core::HSTRING, value: bool, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddBooleanWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddBooleanArray(&self, name: &windows_core::HSTRING, value: &[bool]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddBooleanArray)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddBooleanArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[bool], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddBooleanArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddBooleanArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[bool], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddBooleanArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddString(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value)).ok() }
    }
    pub fn AddStringWithFormat(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddStringWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value), format).ok() }
    }
    pub fn AddStringWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddStringWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value), format, tags).ok() }
    }
    pub fn AddStringArray(&self, name: &windows_core::HSTRING, value: &[windows_core::HSTRING]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddStringArray)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), core::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn AddStringArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[windows_core::HSTRING], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddStringArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), core::mem::transmute(value.as_ptr()), format).ok() }
    }
    pub fn AddStringArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[windows_core::HSTRING], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddStringArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    pub fn AddGuid(&self, name: &windows_core::HSTRING, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddGuid)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddGuidWithFormat(&self, name: &windows_core::HSTRING, value: windows_core::GUID, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddGuidWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddGuidWithFormatAndTags(&self, name: &windows_core::HSTRING, value: windows_core::GUID, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddGuidWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddGuidArray(&self, name: &windows_core::HSTRING, value: &[windows_core::GUID]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddGuidArray)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddGuidArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[windows_core::GUID], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddGuidArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddGuidArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[windows_core::GUID], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddGuidArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddDateTime(&self, name: &windows_core::HSTRING, value: super::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDateTime)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddDateTimeWithFormat(&self, name: &windows_core::HSTRING, value: super::DateTime, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDateTimeWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddDateTimeWithFormatAndTags(&self, name: &windows_core::HSTRING, value: super::DateTime, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDateTimeWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddDateTimeArray(&self, name: &windows_core::HSTRING, value: &[super::DateTime]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDateTimeArray)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddDateTimeArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[super::DateTime], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDateTimeArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddDateTimeArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[super::DateTime], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDateTimeArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddTimeSpan(&self, name: &windows_core::HSTRING, value: super::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddTimeSpan)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddTimeSpanWithFormat(&self, name: &windows_core::HSTRING, value: super::TimeSpan, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddTimeSpanWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddTimeSpanWithFormatAndTags(&self, name: &windows_core::HSTRING, value: super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddTimeSpanWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddTimeSpanArray(&self, name: &windows_core::HSTRING, value: &[super::TimeSpan]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddTimeSpanArray)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddTimeSpanArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[super::TimeSpan], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddTimeSpanArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddTimeSpanArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[super::TimeSpan], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddTimeSpanArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddPoint(&self, name: &windows_core::HSTRING, value: super::Point) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddPoint)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddPointWithFormat(&self, name: &windows_core::HSTRING, value: super::Point, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddPointWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddPointWithFormatAndTags(&self, name: &windows_core::HSTRING, value: super::Point, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddPointWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddPointArray(&self, name: &windows_core::HSTRING, value: &[super::Point]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddPointArray)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddPointArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[super::Point], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddPointArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddPointArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[super::Point], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddPointArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddSize(&self, name: &windows_core::HSTRING, value: super::Size) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddSize)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddSizeWithFormat(&self, name: &windows_core::HSTRING, value: super::Size, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddSizeWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddSizeWithFormatAndTags(&self, name: &windows_core::HSTRING, value: super::Size, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddSizeWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddSizeArray(&self, name: &windows_core::HSTRING, value: &[super::Size]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddSizeArray)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddSizeArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[super::Size], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddSizeArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddSizeArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[super::Size], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddSizeArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
    pub fn AddRect(&self, name: &windows_core::HSTRING, value: super::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddRect)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddRectWithFormat(&self, name: &windows_core::HSTRING, value: super::Rect, format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddRectWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddRectWithFormatAndTags(&self, name: &windows_core::HSTRING, value: super::Rect, format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddRectWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddRectArray(&self, name: &windows_core::HSTRING, value: &[super::Rect]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddRectArray)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn AddRectArrayWithFormat(&self, name: &windows_core::HSTRING, value: &[super::Rect], format: LoggingFieldFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddRectArrayWithFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format).ok() }
    }
    pub fn AddRectArrayWithFormatAndTags(&self, name: &windows_core::HSTRING, value: &[super::Rect], format: LoggingFieldFormat, tags: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddRectArrayWithFormatAndTags)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.len().try_into().unwrap(), value.as_ptr(), format, tags).ok() }
    }
}
impl windows_core::RuntimeType for LoggingFields {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILoggingFields>();
}
unsafe impl windows_core::Interface for LoggingFields {
    type Vtable = ILoggingFields_Vtbl;
    const IID: windows_core::GUID = <ILoggingFields as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LoggingFields {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingFields";
}
unsafe impl Send for LoggingFields {}
unsafe impl Sync for LoggingFields {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LoggingOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LoggingOptions, windows_core::IUnknown, windows_core::IInspectable);
impl LoggingOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LoggingOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Keywords(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Keywords)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeywords(&self, value: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeywords)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tags(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tags)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTags(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTags)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Task(&self) -> windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Task)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTask(&self, value: i16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTask)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opcode(&self) -> windows_core::Result<LoggingOpcode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opcode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpcode(&self, value: LoggingOpcode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOpcode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActivityId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetActivityId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetActivityId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RelatedActivityId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelatedActivityId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRelatedActivityId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRelatedActivityId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateWithKeywords(keywords: i64) -> windows_core::Result<LoggingOptions> {
        Self::ILoggingOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithKeywords)(windows_core::Interface::as_raw(this), keywords, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILoggingOptionsFactory<R, F: FnOnce(&ILoggingOptionsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LoggingOptions, ILoggingOptionsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LoggingOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILoggingOptions>();
}
unsafe impl windows_core::Interface for LoggingOptions {
    type Vtable = ILoggingOptions_Vtbl;
    const IID: windows_core::GUID = <ILoggingOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LoggingOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingOptions";
}
unsafe impl Send for LoggingOptions {}
unsafe impl Sync for LoggingOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LoggingSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LoggingSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LoggingSession, super::IClosable, ILoggingSession);
impl LoggingSession {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SaveToFileAsync<P0>(&self, folder: P0, filename: &windows_core::HSTRING) -> windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveToFileAsync)(windows_core::Interface::as_raw(this), folder.param().abi(), core::mem::transmute_copy(filename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddLoggingChannel<P0>(&self, loggingchannel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddLoggingChannel)(windows_core::Interface::as_raw(this), loggingchannel.param().abi()).ok() }
    }
    pub fn AddLoggingChannelWithLevel<P0>(&self, loggingchannel: P0, maxlevel: LoggingLevel) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddLoggingChannelWithLevel)(windows_core::Interface::as_raw(this), loggingchannel.param().abi(), maxlevel).ok() }
    }
    pub fn RemoveLoggingChannel<P0>(&self, loggingchannel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoggingChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLoggingChannel)(windows_core::Interface::as_raw(this), loggingchannel.param().abi()).ok() }
    }
    pub fn Create(name: &windows_core::HSTRING) -> windows_core::Result<LoggingSession> {
        Self::ILoggingSessionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILoggingSessionFactory<R, F: FnOnce(&ILoggingSessionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LoggingSession, ILoggingSessionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LoggingSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILoggingSession>();
}
unsafe impl windows_core::Interface for LoggingSession {
    type Vtable = ILoggingSession_Vtbl;
    const IID: windows_core::GUID = <ILoggingSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingSession";
}
unsafe impl Send for LoggingSession {}
unsafe impl Sync for LoggingSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RuntimeBrokerErrorSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RuntimeBrokerErrorSettings, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RuntimeBrokerErrorSettings, IErrorReportingSettings);
impl RuntimeBrokerErrorSettings {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RuntimeBrokerErrorSettings, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetErrorOptions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetErrorOptions(&self) -> windows_core::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetErrorOptions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RuntimeBrokerErrorSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IErrorReportingSettings>();
}
unsafe impl windows_core::Interface for RuntimeBrokerErrorSettings {
    type Vtable = IErrorReportingSettings_Vtbl;
    const IID: windows_core::GUID = <IErrorReportingSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RuntimeBrokerErrorSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings";
}
unsafe impl Send for RuntimeBrokerErrorSettings {}
unsafe impl Sync for RuntimeBrokerErrorSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TracingStatusChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TracingStatusChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TracingStatusChangedEventArgs {
    pub fn Enabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Enabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TraceLevel(&self) -> windows_core::Result<CausalityTraceLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TraceLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TracingStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITracingStatusChangedEventArgs>();
}
unsafe impl windows_core::Interface for TracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ITracingStatusChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TracingStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs";
}
unsafe impl Send for TracingStatusChangedEventArgs {}
unsafe impl Sync for TracingStatusChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CausalityRelation(pub i32);
impl CausalityRelation {
    pub const AssignDelegate: Self = Self(0i32);
    pub const Join: Self = Self(1i32);
    pub const Choice: Self = Self(2i32);
    pub const Cancel: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
impl windows_core::TypeKind for CausalityRelation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CausalityRelation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CausalityRelation").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CausalityRelation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalityRelation;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CausalitySource(pub i32);
impl CausalitySource {
    pub const Application: Self = Self(0i32);
    pub const Library: Self = Self(1i32);
    pub const System: Self = Self(2i32);
}
impl windows_core::TypeKind for CausalitySource {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CausalitySource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CausalitySource").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CausalitySource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalitySource;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CausalitySynchronousWork(pub i32);
impl CausalitySynchronousWork {
    pub const CompletionNotification: Self = Self(0i32);
    pub const ProgressNotification: Self = Self(1i32);
    pub const Execution: Self = Self(2i32);
}
impl windows_core::TypeKind for CausalitySynchronousWork {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CausalitySynchronousWork {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CausalitySynchronousWork").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CausalitySynchronousWork {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalitySynchronousWork;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CausalityTraceLevel(pub i32);
impl CausalityTraceLevel {
    pub const Required: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
    pub const Verbose: Self = Self(2i32);
}
impl windows_core::TypeKind for CausalityTraceLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CausalityTraceLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CausalityTraceLevel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CausalityTraceLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalityTraceLevel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ErrorOptions(pub u32);
impl ErrorOptions {
    pub const None: Self = Self(0u32);
    pub const SuppressExceptions: Self = Self(1u32);
    pub const ForceExceptions: Self = Self(2u32);
    pub const UseSetErrorInfo: Self = Self(4u32);
    pub const SuppressSetErrorInfo: Self = Self(8u32);
}
impl windows_core::TypeKind for ErrorOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ErrorOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ErrorOptions").field(&self.0).finish()
    }
}
impl ErrorOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ErrorOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ErrorOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ErrorOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ErrorOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ErrorOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for ErrorOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.ErrorOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LoggingFieldFormat(pub i32);
impl LoggingFieldFormat {
    pub const Default: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const String: Self = Self(2i32);
    pub const Boolean: Self = Self(3i32);
    pub const Hexadecimal: Self = Self(4i32);
    pub const ProcessId: Self = Self(5i32);
    pub const ThreadId: Self = Self(6i32);
    pub const Port: Self = Self(7i32);
    pub const Ipv4Address: Self = Self(8i32);
    pub const Ipv6Address: Self = Self(9i32);
    pub const SocketAddress: Self = Self(10i32);
    pub const Xml: Self = Self(11i32);
    pub const Json: Self = Self(12i32);
    pub const Win32Error: Self = Self(13i32);
    pub const NTStatus: Self = Self(14i32);
    pub const HResult: Self = Self(15i32);
    pub const FileTime: Self = Self(16i32);
    pub const Signed: Self = Self(17i32);
    pub const Unsigned: Self = Self(18i32);
}
impl windows_core::TypeKind for LoggingFieldFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LoggingFieldFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LoggingFieldFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LoggingFieldFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingFieldFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LoggingLevel(pub i32);
impl LoggingLevel {
    pub const Verbose: Self = Self(0i32);
    pub const Information: Self = Self(1i32);
    pub const Warning: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Critical: Self = Self(4i32);
}
impl windows_core::TypeKind for LoggingLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LoggingLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LoggingLevel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LoggingLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingLevel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LoggingOpcode(pub i32);
impl LoggingOpcode {
    pub const Info: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const Stop: Self = Self(2i32);
    pub const Reply: Self = Self(6i32);
    pub const Resume: Self = Self(7i32);
    pub const Suspend: Self = Self(8i32);
    pub const Send: Self = Self(9i32);
}
impl windows_core::TypeKind for LoggingOpcode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LoggingOpcode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LoggingOpcode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LoggingOpcode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingOpcode;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
