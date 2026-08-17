windows_core::imp::define_interface!(IErrorReceivedEventArgs, IErrorReceivedEventArgs_Vtbl, 0xfcc6bf59_1283_4d8a_bfdf_566b33ddb28f);
impl windows_core::RuntimeType for IErrorReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IErrorReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SerialError) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPinChangedEventArgs, IPinChangedEventArgs_Vtbl, 0xa2bf1db0_fc9c_4607_93d0_fa5e8343ee22);
impl windows_core::RuntimeType for IPinChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPinChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PinChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SerialPinChange) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISerialDevice, ISerialDevice_Vtbl, 0xe187ccc6_2210_414f_b65a_f5553a03372a);
impl windows_core::RuntimeType for ISerialDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISerialDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BaudRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBaudRate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BreakSignalState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetBreakSignalState: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CarrierDetectState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ClearToSendState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DataBits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetDataBits: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub DataSetReadyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Handshake: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SerialHandshake) -> windows_core::HRESULT,
    pub SetHandshake: unsafe extern "system" fn(*mut core::ffi::c_void, SerialHandshake) -> windows_core::HRESULT,
    pub IsDataTerminalReadyEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDataTerminalReadyEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsRequestToSendEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsRequestToSendEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Parity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SerialParity) -> windows_core::HRESULT,
    pub SetParity: unsafe extern "system" fn(*mut core::ffi::c_void, SerialParity) -> windows_core::HRESULT,
    pub PortName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReadTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetReadTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub StopBits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SerialStopBitCount) -> windows_core::HRESULT,
    pub SetStopBits: unsafe extern "system" fn(*mut core::ffi::c_void, SerialStopBitCount) -> windows_core::HRESULT,
    pub UsbVendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub UsbProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub WriteTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetWriteTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    pub ErrorReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveErrorReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PinChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePinChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISerialDeviceStatics, ISerialDeviceStatics_Vtbl, 0x058c4a70_0836_4993_ae1a_b61ae3be056b);
impl windows_core::RuntimeType for ISerialDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISerialDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromPortName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromUsbVidPid: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ErrorReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ErrorReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ErrorReceivedEventArgs {
    pub fn Error(&self) -> windows_core::Result<SerialError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ErrorReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IErrorReceivedEventArgs>();
}
unsafe impl windows_core::Interface for ErrorReceivedEventArgs {
    type Vtable = IErrorReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IErrorReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ErrorReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.ErrorReceivedEventArgs";
}
unsafe impl Send for ErrorReceivedEventArgs {}
unsafe impl Sync for ErrorReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PinChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PinChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PinChangedEventArgs {
    pub fn PinChange(&self) -> windows_core::Result<SerialPinChange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PinChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PinChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPinChangedEventArgs>();
}
unsafe impl windows_core::Interface for PinChangedEventArgs {
    type Vtable = IPinChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPinChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PinChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.PinChangedEventArgs";
}
unsafe impl Send for PinChangedEventArgs {}
unsafe impl Sync for PinChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SerialDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SerialDevice, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SerialDevice, super::super::Foundation::IClosable);
impl SerialDevice {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn BaudRate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaudRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBaudRate(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBaudRate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BreakSignalState(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BreakSignalState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBreakSignalState(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBreakSignalState)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BytesReceived(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesReceived)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CarrierDetectState(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CarrierDetectState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ClearToSendState(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearToSendState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DataBits(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataBits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDataBits(&self, value: u16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDataBits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DataSetReadyState(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataSetReadyState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Handshake(&self) -> windows_core::Result<SerialHandshake> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handshake)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandshake(&self, value: SerialHandshake) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandshake)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDataTerminalReadyEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDataTerminalReadyEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDataTerminalReadyEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDataTerminalReadyEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRequestToSendEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequestToSendEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRequestToSendEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsRequestToSendEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Parity(&self) -> windows_core::Result<SerialParity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetParity(&self, value: SerialParity) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetParity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PortName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PortName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadTimeout(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReadTimeout(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReadTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StopBits(&self) -> windows_core::Result<SerialStopBitCount> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopBits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStopBits(&self, value: SerialStopBitCount) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStopBits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UsbVendorId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsbVendorId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsbProductId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsbProductId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WriteTimeout(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWriteTimeout(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWriteTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ErrorReceived<P0>(&self, reporthandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SerialDevice, ErrorReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorReceived)(windows_core::Interface::as_raw(this), reporthandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveErrorReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveErrorReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PinChanged<P0>(&self, reporthandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SerialDevice, PinChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PinChanged)(windows_core::Interface::as_raw(this), reporthandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePinChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePinChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISerialDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorFromPortName(portname: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        Self::ISerialDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromPortName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(portname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorFromUsbVidPid(vendorid: u16, productid: u16) -> windows_core::Result<windows_core::HSTRING> {
        Self::ISerialDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromUsbVidPid)(windows_core::Interface::as_raw(this), vendorid, productid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SerialDevice>> {
        Self::ISerialDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISerialDeviceStatics<R, F: FnOnce(&ISerialDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SerialDevice, ISerialDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SerialDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISerialDevice>();
}
unsafe impl windows_core::Interface for SerialDevice {
    type Vtable = ISerialDevice_Vtbl;
    const IID: windows_core::GUID = <ISerialDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SerialDevice {
    const NAME: &'static str = "Windows.Devices.SerialCommunication.SerialDevice";
}
unsafe impl Send for SerialDevice {}
unsafe impl Sync for SerialDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SerialError(pub i32);
impl SerialError {
    pub const Frame: Self = Self(0i32);
    pub const BufferOverrun: Self = Self(1i32);
    pub const ReceiveFull: Self = Self(2i32);
    pub const ReceiveParity: Self = Self(3i32);
    pub const TransmitFull: Self = Self(4i32);
}
impl windows_core::TypeKind for SerialError {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SerialError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SerialError").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SerialError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialError;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SerialHandshake(pub i32);
impl SerialHandshake {
    pub const None: Self = Self(0i32);
    pub const RequestToSend: Self = Self(1i32);
    pub const XOnXOff: Self = Self(2i32);
    pub const RequestToSendXOnXOff: Self = Self(3i32);
}
impl windows_core::TypeKind for SerialHandshake {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SerialHandshake {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SerialHandshake").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SerialHandshake {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialHandshake;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SerialParity(pub i32);
impl SerialParity {
    pub const None: Self = Self(0i32);
    pub const Odd: Self = Self(1i32);
    pub const Even: Self = Self(2i32);
    pub const Mark: Self = Self(3i32);
    pub const Space: Self = Self(4i32);
}
impl windows_core::TypeKind for SerialParity {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SerialParity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SerialParity").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SerialParity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialParity;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SerialPinChange(pub i32);
impl SerialPinChange {
    pub const BreakSignal: Self = Self(0i32);
    pub const CarrierDetect: Self = Self(1i32);
    pub const ClearToSend: Self = Self(2i32);
    pub const DataSetReady: Self = Self(3i32);
    pub const RingIndicator: Self = Self(4i32);
}
impl windows_core::TypeKind for SerialPinChange {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SerialPinChange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SerialPinChange").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SerialPinChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialPinChange;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SerialStopBitCount(pub i32);
impl SerialStopBitCount {
    pub const One: Self = Self(0i32);
    pub const OnePointFive: Self = Self(1i32);
    pub const Two: Self = Self(2i32);
}
impl windows_core::TypeKind for SerialStopBitCount {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SerialStopBitCount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SerialStopBitCount").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SerialStopBitCount {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SerialCommunication.SerialStopBitCount;i4)");
}
