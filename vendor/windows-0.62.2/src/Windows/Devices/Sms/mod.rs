#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CellularClass(pub i32);
impl CellularClass {
    pub const None: Self = Self(0i32);
    pub const Gsm: Self = Self(1i32);
    pub const Cdma: Self = Self(2i32);
}
impl windows_core::TypeKind for CellularClass {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CellularClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.CellularClass;i4)");
}
pub type DeleteSmsMessageOperation = windows_future::IAsyncAction;
pub type DeleteSmsMessagesOperation = windows_future::IAsyncAction;
pub type GetSmsDeviceOperation = windows_future::IAsyncOperation<SmsDevice>;
pub type GetSmsMessageOperation = windows_future::IAsyncOperation<ISmsMessage>;
pub type GetSmsMessagesOperation = windows_future::IAsyncOperationWithProgress<windows_collections::IVectorView<ISmsMessage>, i32>;
windows_core::imp::define_interface!(ISmsAppMessage, ISmsAppMessage_Vtbl, 0xe8bb8494_d3a0_4a0a_86d7_291033a8cf54);
impl windows_core::RuntimeType for ISmsAppMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsAppMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub To: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub From: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CallbackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCallbackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsDeliveryNotificationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDeliveryNotificationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RetryAttemptCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRetryAttemptCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Encoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsEncoding) -> windows_core::HRESULT,
    pub SetEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, SmsEncoding) -> windows_core::HRESULT,
    pub PortNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPortNumber: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub TeleserviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTeleserviceId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ProtocolId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProtocolId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub BinaryBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BinaryBody: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetBinaryBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBinaryBody: usize,
}
windows_core::imp::define_interface!(ISmsBinaryMessage, ISmsBinaryMessage_Vtbl, 0x5bf4e813_3b53_4c6e_b61a_d86a63755650);
impl windows_core::RuntimeType for ISmsBinaryMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISmsBinaryMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ISmsBinaryMessage, ISmsMessage);
impl ISmsBinaryMessage {
    pub fn Format(&self) -> windows_core::Result<SmsDataFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFormat(&self, value: SmsDataFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetData(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetData)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SetData(&self, value: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = &windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for ISmsBinaryMessage {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsBinaryMessage";
}
pub trait ISmsBinaryMessage_Impl: ISmsMessage_Impl {
    fn Format(&self) -> windows_core::Result<SmsDataFormat>;
    fn SetFormat(&self, value: SmsDataFormat) -> windows_core::Result<()>;
    fn GetData(&self) -> windows_core::Result<windows_core::Array<u8>>;
    fn SetData(&self, value: &[u8]) -> windows_core::Result<()>;
}
impl ISmsBinaryMessage_Vtbl {
    pub const fn new<Identity: ISmsBinaryMessage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Format<Identity: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SmsDataFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsBinaryMessage_Impl::Format(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SmsDataFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISmsBinaryMessage_Impl::SetFormat(this, value).into()
            }
        }
        unsafe extern "system" fn GetData<Identity: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsBinaryMessage_Impl::GetData(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetData<Identity: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: u32, value: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISmsBinaryMessage_Impl::SetData(this, core::slice::from_raw_parts(core::mem::transmute_copy(&value), value_array_size as usize)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISmsBinaryMessage, OFFSET>(),
            Format: Format::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISmsBinaryMessage as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsBinaryMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsDataFormat) -> windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, SmsDataFormat) -> windows_core::HRESULT,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsBroadcastMessage, ISmsBroadcastMessage_Vtbl, 0x75aebbf1_e4b7_4874_a09c_2956e592f957);
impl windows_core::RuntimeType for ISmsBroadcastMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsBroadcastMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub To: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Channel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GeographicalScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsGeographicalScope) -> windows_core::HRESULT,
    pub MessageCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UpdateNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BroadcastType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsBroadcastType) -> windows_core::HRESULT,
    pub IsEmergencyAlert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsUserPopupRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsDevice, ISmsDevice_Vtbl, 0x091791ed_872b_4eec_9c72_ab11627b34ec);
impl windows_core::RuntimeType for ISmsDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISmsDevice, windows_core::IUnknown, windows_core::IInspectable);
impl ISmsDevice {
    pub fn SendMessageAsync<P0>(&self, message: P0) -> windows_core::Result<SendSmsMessageOperation>
    where
        P0: windows_core::Param<ISmsMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendMessageAsync)(windows_core::Interface::as_raw(this), message.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CalculateLength<P0>(&self, message: P0) -> windows_core::Result<SmsEncodedLength>
    where
        P0: windows_core::Param<SmsTextMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CalculateLength)(windows_core::Interface::as_raw(this), message.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn AccountPhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountPhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CellularClass(&self) -> windows_core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageStore(&self) -> windows_core::Result<SmsDeviceMessageStore> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageStore)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceStatus(&self) -> windows_core::Result<SmsDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SmsMessageReceived<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<SmsMessageReceivedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmsMessageReceived)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSmsMessageReceived(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSmsMessageReceived)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn SmsDeviceStatusChanged<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<SmsDeviceStatusChangedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmsDeviceStatusChanged)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSmsDeviceStatusChanged(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSmsDeviceStatusChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
}
impl windows_core::RuntimeName for ISmsDevice {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsDevice";
}
pub trait ISmsDevice_Impl: windows_core::IUnknownImpl {
    fn SendMessageAsync(&self, message: windows_core::Ref<ISmsMessage>) -> windows_core::Result<SendSmsMessageOperation>;
    fn CalculateLength(&self, message: windows_core::Ref<SmsTextMessage>) -> windows_core::Result<SmsEncodedLength>;
    fn AccountPhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn CellularClass(&self) -> windows_core::Result<CellularClass>;
    fn MessageStore(&self) -> windows_core::Result<SmsDeviceMessageStore>;
    fn DeviceStatus(&self) -> windows_core::Result<SmsDeviceStatus>;
    fn SmsMessageReceived(&self, eventHandler: windows_core::Ref<SmsMessageReceivedEventHandler>) -> windows_core::Result<i64>;
    fn RemoveSmsMessageReceived(&self, eventCookie: i64) -> windows_core::Result<()>;
    fn SmsDeviceStatusChanged(&self, eventHandler: windows_core::Ref<SmsDeviceStatusChangedEventHandler>) -> windows_core::Result<i64>;
    fn RemoveSmsDeviceStatusChanged(&self, eventCookie: i64) -> windows_core::Result<()>;
}
impl ISmsDevice_Vtbl {
    pub const fn new<Identity: ISmsDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendMessageAsync<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsDevice_Impl::SendMessageAsync(this, core::mem::transmute_copy(&message)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CalculateLength<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut core::ffi::c_void, result__: *mut SmsEncodedLength) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsDevice_Impl::CalculateLength(this, core::mem::transmute_copy(&message)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AccountPhoneNumber<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsDevice_Impl::AccountPhoneNumber(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CellularClass<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CellularClass) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsDevice_Impl::CellularClass(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MessageStore<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsDevice_Impl::MessageStore(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceStatus<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SmsDeviceStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsDevice_Impl::DeviceStatus(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SmsMessageReceived<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsDevice_Impl::SmsMessageReceived(this, core::mem::transmute_copy(&eventhandler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveSmsMessageReceived<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISmsDevice_Impl::RemoveSmsMessageReceived(this, eventcookie).into()
            }
        }
        unsafe extern "system" fn SmsDeviceStatusChanged<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsDevice_Impl::SmsDeviceStatusChanged(this, core::mem::transmute_copy(&eventhandler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveSmsDeviceStatusChanged<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISmsDevice_Impl::RemoveSmsDeviceStatusChanged(this, eventcookie).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISmsDevice, OFFSET>(),
            SendMessageAsync: SendMessageAsync::<Identity, OFFSET>,
            CalculateLength: CalculateLength::<Identity, OFFSET>,
            AccountPhoneNumber: AccountPhoneNumber::<Identity, OFFSET>,
            CellularClass: CellularClass::<Identity, OFFSET>,
            MessageStore: MessageStore::<Identity, OFFSET>,
            DeviceStatus: DeviceStatus::<Identity, OFFSET>,
            SmsMessageReceived: SmsMessageReceived::<Identity, OFFSET>,
            RemoveSmsMessageReceived: RemoveSmsMessageReceived::<Identity, OFFSET>,
            SmsDeviceStatusChanged: SmsDeviceStatusChanged::<Identity, OFFSET>,
            RemoveSmsDeviceStatusChanged: RemoveSmsDeviceStatusChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISmsDevice as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SendMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CalculateLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut SmsEncodedLength) -> windows_core::HRESULT,
    pub AccountPhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CellularClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CellularClass) -> windows_core::HRESULT,
    pub MessageStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsDeviceStatus) -> windows_core::HRESULT,
    pub SmsMessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSmsMessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SmsDeviceStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSmsDeviceStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsDevice2, ISmsDevice2_Vtbl, 0xbd8a5c13_e522_46cb_b8d5_9ead30fb6c47);
impl windows_core::RuntimeType for ISmsDevice2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDevice2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SmscAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSmscAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParentDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AccountPhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CellularClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CellularClass) -> windows_core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsDeviceStatus) -> windows_core::HRESULT,
    pub CalculateLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut SmsEncodedLength) -> windows_core::HRESULT,
    pub SendMessageAndGetResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveDeviceStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsDevice2Statics, ISmsDevice2Statics_Vtbl, 0x65c78325_1031_491e_8fb6_ef9991afe363);
impl windows_core::RuntimeType for ISmsDevice2Statics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDevice2Statics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromParentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsDeviceMessageStore, ISmsDeviceMessageStore_Vtbl, 0x9889f253_f188_4427_8d54_ce0c2423c5c1);
impl windows_core::RuntimeType for ISmsDeviceMessageStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDeviceMessageStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeleteMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteMessagesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, SmsMessageFilter, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessagesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, SmsMessageFilter, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsDeviceStatics, ISmsDeviceStatics_Vtbl, 0xf88d07ea_d815_4dd1_a234_4520ce4604a4);
impl windows_core::RuntimeType for ISmsDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsDeviceStatics2, ISmsDeviceStatics2_Vtbl, 0x2ca11c87_0873_4caf_8a7d_bd471e8586d1);
impl windows_core::RuntimeType for ISmsDeviceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDeviceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromNetworkAccountIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsFilterRule, ISmsFilterRule_Vtbl, 0x40e32fae_b049_4fbc_afe9_e2a610eff55c);
impl windows_core::RuntimeType for ISmsFilterRule {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRule_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsMessageType) -> windows_core::HRESULT,
    pub ImsiPrefixes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SenderNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TextMessagePrefixes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PortNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CellularClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CellularClass) -> windows_core::HRESULT,
    pub SetCellularClass: unsafe extern "system" fn(*mut core::ffi::c_void, CellularClass) -> windows_core::HRESULT,
    pub ProtocolIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TeleserviceIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WapApplicationIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WapContentTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BroadcastTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BroadcastChannels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsFilterRuleFactory, ISmsFilterRuleFactory_Vtbl, 0x00c36508_6296_4f29_9aad_8920ceba3ce8);
impl windows_core::RuntimeType for ISmsFilterRuleFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRuleFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFilterRule: unsafe extern "system" fn(*mut core::ffi::c_void, SmsMessageType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsFilterRules, ISmsFilterRules_Vtbl, 0x4e47eafb_79cd_4881_9894_55a4135b23fa);
impl windows_core::RuntimeType for ISmsFilterRules {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRules_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsFilterActionType) -> windows_core::HRESULT,
    pub Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsFilterRulesFactory, ISmsFilterRulesFactory_Vtbl, 0xa09924ed_6e2e_4530_9fde_465d02eed00e);
impl windows_core::RuntimeType for ISmsFilterRulesFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRulesFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFilterRules: unsafe extern "system" fn(*mut core::ffi::c_void, SmsFilterActionType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsMessage, ISmsMessage_Vtbl, 0xed3c5e28_6984_4b07_811d_8d5906ed3cea);
impl windows_core::RuntimeType for ISmsMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISmsMessage, windows_core::IUnknown, windows_core::IInspectable);
impl ISmsMessage {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for ISmsMessage {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsMessage";
}
pub trait ISmsMessage_Impl: windows_core::IUnknownImpl {
    fn Id(&self) -> windows_core::Result<u32>;
    fn MessageClass(&self) -> windows_core::Result<SmsMessageClass>;
}
impl ISmsMessage_Vtbl {
    pub const fn new<Identity: ISmsMessage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: ISmsMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsMessage_Impl::Id(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MessageClass<Identity: ISmsMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SmsMessageClass) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsMessage_Impl::MessageClass(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISmsMessage, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            MessageClass: MessageClass::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISmsMessage as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MessageClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsMessageClass) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsMessageBase, ISmsMessageBase_Vtbl, 0x2cf0fe30_fe50_4fc6_aa88_4ccfe27a29ea);
impl windows_core::RuntimeType for ISmsMessageBase {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISmsMessageBase, windows_core::IUnknown, windows_core::IInspectable);
impl ISmsMessageBase {
    pub fn MessageType(&self) -> windows_core::Result<SmsMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CellularClass(&self) -> windows_core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SimIccId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimIccId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for ISmsMessageBase {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsMessageBase";
}
pub trait ISmsMessageBase_Impl: windows_core::IUnknownImpl {
    fn MessageType(&self) -> windows_core::Result<SmsMessageType>;
    fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn CellularClass(&self) -> windows_core::Result<CellularClass>;
    fn MessageClass(&self) -> windows_core::Result<SmsMessageClass>;
    fn SimIccId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl ISmsMessageBase_Vtbl {
    pub const fn new<Identity: ISmsMessageBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MessageType<Identity: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SmsMessageType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsMessageBase_Impl::MessageType(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceId<Identity: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsMessageBase_Impl::DeviceId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CellularClass<Identity: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CellularClass) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsMessageBase_Impl::CellularClass(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MessageClass<Identity: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SmsMessageClass) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsMessageBase_Impl::MessageClass(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SimIccId<Identity: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsMessageBase_Impl::SimIccId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISmsMessageBase, OFFSET>(),
            MessageType: MessageType::<Identity, OFFSET>,
            DeviceId: DeviceId::<Identity, OFFSET>,
            CellularClass: CellularClass::<Identity, OFFSET>,
            MessageClass: MessageClass::<Identity, OFFSET>,
            SimIccId: SimIccId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISmsMessageBase as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsMessageType) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CellularClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CellularClass) -> windows_core::HRESULT,
    pub MessageClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsMessageClass) -> windows_core::HRESULT,
    pub SimIccId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsMessageReceivedEventArgs, ISmsMessageReceivedEventArgs_Vtbl, 0x08e80a98_b8e5_41c1_a3d8_d3abfae22675);
impl windows_core::RuntimeType for ISmsMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TextMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BinaryMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsMessageReceivedTriggerDetails, ISmsMessageReceivedTriggerDetails_Vtbl, 0x2bcfcbd4_2657_4128_ad5f_e3877132bdb1);
impl windows_core::RuntimeType for ISmsMessageReceivedTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageReceivedTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsMessageType) -> windows_core::HRESULT,
    pub TextMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WapMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BroadcastMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VoicemailMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StatusMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Drop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsMessageRegistration, ISmsMessageRegistration_Vtbl, 0x1720503e_f34f_446b_83b3_0ff19923b409);
impl windows_core::RuntimeType for ISmsMessageRegistration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageRegistration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveMessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsMessageRegistrationStatics, ISmsMessageRegistrationStatics_Vtbl, 0x63a05464_2898_4778_a03c_6f994907d63a);
impl windows_core::RuntimeType for ISmsMessageRegistrationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageRegistrationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllRegistrations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsReceivedEventDetails, ISmsReceivedEventDetails_Vtbl, 0x5bb50f15_e46d_4c82_847d_5a0304c1d53d);
impl windows_core::RuntimeType for ISmsReceivedEventDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsReceivedEventDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MessageIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsReceivedEventDetails2, ISmsReceivedEventDetails2_Vtbl, 0x40e05c86_a7b4_4771_9ae7_0b5ffb12c03a);
impl windows_core::RuntimeType for ISmsReceivedEventDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsReceivedEventDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsMessageClass) -> windows_core::HRESULT,
    pub BinaryMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsSendMessageResult, ISmsSendMessageResult_Vtbl, 0xdb139af2_78c9_4feb_9622_452328088d62);
impl windows_core::RuntimeType for ISmsSendMessageResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsSendMessageResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSuccessful: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MessageReferenceNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CellularClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CellularClass) -> windows_core::HRESULT,
    pub ModemErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsModemErrorCode) -> windows_core::HRESULT,
    pub IsErrorTransient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub NetworkCauseCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TransportFailureCause: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsStatusMessage, ISmsStatusMessage_Vtbl, 0xe6d28342_b70b_4677_9379_c9783fdff8f4);
impl windows_core::RuntimeType for ISmsStatusMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsStatusMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub To: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub From: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MessageReferenceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ServiceCenterTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub DischargeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsTextMessage, ISmsTextMessage_Vtbl, 0xd61c904c_a495_487f_9a6f_971548c5bc9f);
impl windows_core::RuntimeType for ISmsTextMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISmsTextMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ISmsTextMessage, ISmsMessage);
impl ISmsTextMessage {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PartReferenceId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PartReferenceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PartNumber(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PartNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PartCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PartCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn To(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).To)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTo(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn From(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).From)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetFrom(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFrom)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Body(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Body)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetBody(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBody)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Encoding(&self) -> windows_core::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Encoding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEncoding(&self, value: SmsEncoding) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEncoding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ToBinaryMessages(&self, format: SmsDataFormat) -> windows_core::Result<windows_collections::IVectorView<ISmsBinaryMessage>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToBinaryMessages)(windows_core::Interface::as_raw(this), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = &windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for ISmsTextMessage {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsTextMessage";
}
pub trait ISmsTextMessage_Impl: ISmsMessage_Impl {
    fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime>;
    fn PartReferenceId(&self) -> windows_core::Result<u32>;
    fn PartNumber(&self) -> windows_core::Result<u32>;
    fn PartCount(&self) -> windows_core::Result<u32>;
    fn To(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetTo(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn From(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetFrom(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Body(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetBody(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Encoding(&self) -> windows_core::Result<SmsEncoding>;
    fn SetEncoding(&self, value: SmsEncoding) -> windows_core::Result<()>;
    fn ToBinaryMessages(&self, format: SmsDataFormat) -> windows_core::Result<windows_collections::IVectorView<ISmsBinaryMessage>>;
}
impl ISmsTextMessage_Vtbl {
    pub const fn new<Identity: ISmsTextMessage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Timestamp<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsTextMessage_Impl::Timestamp(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PartReferenceId<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsTextMessage_Impl::PartReferenceId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PartNumber<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsTextMessage_Impl::PartNumber(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PartCount<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsTextMessage_Impl::PartCount(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn To<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsTextMessage_Impl::To(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTo<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISmsTextMessage_Impl::SetTo(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn From<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsTextMessage_Impl::From(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFrom<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISmsTextMessage_Impl::SetFrom(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Body<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsTextMessage_Impl::Body(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBody<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISmsTextMessage_Impl::SetBody(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Encoding<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SmsEncoding) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsTextMessage_Impl::Encoding(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEncoding<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SmsEncoding) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISmsTextMessage_Impl::SetEncoding(this, value).into()
            }
        }
        unsafe extern "system" fn ToBinaryMessages<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: SmsDataFormat, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISmsTextMessage_Impl::ToBinaryMessages(this, format) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISmsTextMessage, OFFSET>(),
            Timestamp: Timestamp::<Identity, OFFSET>,
            PartReferenceId: PartReferenceId::<Identity, OFFSET>,
            PartNumber: PartNumber::<Identity, OFFSET>,
            PartCount: PartCount::<Identity, OFFSET>,
            To: To::<Identity, OFFSET>,
            SetTo: SetTo::<Identity, OFFSET>,
            From: From::<Identity, OFFSET>,
            SetFrom: SetFrom::<Identity, OFFSET>,
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            Encoding: Encoding::<Identity, OFFSET>,
            SetEncoding: SetEncoding::<Identity, OFFSET>,
            ToBinaryMessages: ToBinaryMessages::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISmsTextMessage as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsTextMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub PartReferenceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PartNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PartCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub To: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub From: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Encoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsEncoding) -> windows_core::HRESULT,
    pub SetEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, SmsEncoding) -> windows_core::HRESULT,
    pub ToBinaryMessages: unsafe extern "system" fn(*mut core::ffi::c_void, SmsDataFormat, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsTextMessage2, ISmsTextMessage2_Vtbl, 0x22a0d893_4555_4755_b5a1_e7fd84955f8d);
impl windows_core::RuntimeType for ISmsTextMessage2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsTextMessage2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub To: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub From: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Encoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmsEncoding) -> windows_core::HRESULT,
    pub SetEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, SmsEncoding) -> windows_core::HRESULT,
    pub CallbackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCallbackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsDeliveryNotificationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDeliveryNotificationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RetryAttemptCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRetryAttemptCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub TeleserviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProtocolId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsTextMessageStatics, ISmsTextMessageStatics_Vtbl, 0x7f68c5ed_3ccc_47a3_8c55_380d3b010892);
impl windows_core::RuntimeType for ISmsTextMessageStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsTextMessageStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromBinaryMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromBinaryData: unsafe extern "system" fn(*mut core::ffi::c_void, SmsDataFormat, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsVoicemailMessage, ISmsVoicemailMessage_Vtbl, 0x271aa0a6_95b1_44ff_bcb8_b8fdd7e08bc3);
impl windows_core::RuntimeType for ISmsVoicemailMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsVoicemailMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub To: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MessageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmsWapMessage, ISmsWapMessage_Vtbl, 0xcd937743_7a55_4d3b_9021_f22e022d09c5);
impl windows_core::RuntimeType for ISmsWapMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsWapMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub To: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub From: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub BinaryBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BinaryBody: usize,
    pub Headers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub type SendSmsMessageOperation = windows_future::IAsyncAction;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsAppMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsAppMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SmsAppMessage, ISmsMessageBase);
impl SmsAppMessage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmsAppMessage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn To(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).To)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTo(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn From(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).From)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Body(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Body)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetBody(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBody)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CallbackNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallbackNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCallbackNumber(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCallbackNumber)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsDeliveryNotificationEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDeliveryNotificationEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDeliveryNotificationEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDeliveryNotificationEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RetryAttemptCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetryAttemptCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRetryAttemptCount(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRetryAttemptCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Encoding(&self) -> windows_core::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Encoding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEncoding(&self, value: SmsEncoding) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEncoding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PortNumber(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PortNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPortNumber(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPortNumber)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TeleserviceId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TeleserviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTeleserviceId(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTeleserviceId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProtocolId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProtocolId(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProtocolId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn BinaryBody(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BinaryBody)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetBinaryBody<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBinaryBody)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn MessageType(&self) -> windows_core::Result<SmsMessageType> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CellularClass(&self) -> windows_core::Result<CellularClass> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SimIccId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimIccId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for SmsAppMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsAppMessage>();
}
unsafe impl windows_core::Interface for SmsAppMessage {
    type Vtable = <ISmsAppMessage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsAppMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsAppMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsAppMessage";
}
unsafe impl Send for SmsAppMessage {}
unsafe impl Sync for SmsAppMessage {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsBinaryMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsBinaryMessage, windows_core::IUnknown, windows_core::IInspectable, ISmsBinaryMessage);
windows_core::imp::required_hierarchy!(SmsBinaryMessage, ISmsMessage);
impl SmsBinaryMessage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmsBinaryMessage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Format(&self) -> windows_core::Result<SmsDataFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFormat(&self, value: SmsDataFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetData(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetData)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SetData(&self, value: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = &windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SmsBinaryMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsBinaryMessage>();
}
unsafe impl windows_core::Interface for SmsBinaryMessage {
    type Vtable = <ISmsBinaryMessage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsBinaryMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsBinaryMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsBinaryMessage";
}
unsafe impl Send for SmsBinaryMessage {}
unsafe impl Sync for SmsBinaryMessage {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsBroadcastMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsBroadcastMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SmsBroadcastMessage, ISmsMessageBase);
impl SmsBroadcastMessage {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn To(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).To)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Body(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Body)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Channel(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Channel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GeographicalScope(&self) -> windows_core::Result<SmsGeographicalScope> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeographicalScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageCode(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UpdateNumber(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BroadcastType(&self) -> windows_core::Result<SmsBroadcastType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsEmergencyAlert(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEmergencyAlert)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsUserPopupRequested(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUserPopupRequested)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageType(&self) -> windows_core::Result<SmsMessageType> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CellularClass(&self) -> windows_core::Result<CellularClass> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SimIccId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimIccId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for SmsBroadcastMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsBroadcastMessage>();
}
unsafe impl windows_core::Interface for SmsBroadcastMessage {
    type Vtable = <ISmsBroadcastMessage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsBroadcastMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsBroadcastMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsBroadcastMessage";
}
unsafe impl Send for SmsBroadcastMessage {}
unsafe impl Sync for SmsBroadcastMessage {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SmsBroadcastType(pub i32);
impl SmsBroadcastType {
    pub const Other: Self = Self(0i32);
    pub const CmasPresidential: Self = Self(1i32);
    pub const CmasExtreme: Self = Self(2i32);
    pub const CmasSevere: Self = Self(3i32);
    pub const CmasAmber: Self = Self(4i32);
    pub const CmasTest: Self = Self(5i32);
    pub const EUAlert1: Self = Self(6i32);
    pub const EUAlert2: Self = Self(7i32);
    pub const EUAlert3: Self = Self(8i32);
    pub const EUAlertAmber: Self = Self(9i32);
    pub const EUAlertInfo: Self = Self(10i32);
    pub const EtwsEarthquake: Self = Self(11i32);
    pub const EtwsTsunami: Self = Self(12i32);
    pub const EtwsTsunamiAndEarthquake: Self = Self(13i32);
    pub const LatAlertLocal: Self = Self(14i32);
}
impl windows_core::TypeKind for SmsBroadcastType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SmsBroadcastType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsBroadcastType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SmsDataFormat(pub i32);
impl SmsDataFormat {
    pub const Unknown: Self = Self(0i32);
    pub const CdmaSubmit: Self = Self(1i32);
    pub const GsmSubmit: Self = Self(2i32);
    pub const CdmaDeliver: Self = Self(3i32);
    pub const GsmDeliver: Self = Self(4i32);
}
impl windows_core::TypeKind for SmsDataFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SmsDataFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsDataFormat;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsDevice, windows_core::IUnknown, windows_core::IInspectable, ISmsDevice);
impl SmsDevice {
    pub fn SendMessageAsync<P0>(&self, message: P0) -> windows_core::Result<SendSmsMessageOperation>
    where
        P0: windows_core::Param<ISmsMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendMessageAsync)(windows_core::Interface::as_raw(this), message.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CalculateLength<P0>(&self, message: P0) -> windows_core::Result<SmsEncodedLength>
    where
        P0: windows_core::Param<SmsTextMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CalculateLength)(windows_core::Interface::as_raw(this), message.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn AccountPhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountPhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CellularClass(&self) -> windows_core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageStore(&self) -> windows_core::Result<SmsDeviceMessageStore> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageStore)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceStatus(&self) -> windows_core::Result<SmsDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SmsMessageReceived<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<SmsMessageReceivedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmsMessageReceived)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSmsMessageReceived(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSmsMessageReceived)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn SmsDeviceStatusChanged<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<SmsDeviceStatusChangedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmsDeviceStatusChanged)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSmsDeviceStatusChanged(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSmsDeviceStatusChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISmsDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<SmsDevice>> {
        Self::ISmsDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAsync() -> windows_core::Result<windows_future::IAsyncOperation<SmsDevice>> {
        Self::ISmsDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromNetworkAccountIdAsync(networkaccountid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<SmsDevice>> {
        Self::ISmsDeviceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromNetworkAccountIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(networkaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISmsDeviceStatics<R, F: FnOnce(&ISmsDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmsDevice, ISmsDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ISmsDeviceStatics2<R, F: FnOnce(&ISmsDeviceStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmsDevice, ISmsDeviceStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmsDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsDevice>();
}
unsafe impl windows_core::Interface for SmsDevice {
    type Vtable = <ISmsDevice as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsDevice {
    const NAME: &'static str = "Windows.Devices.Sms.SmsDevice";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsDevice2(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsDevice2, windows_core::IUnknown, windows_core::IInspectable);
impl SmsDevice2 {
    pub fn SmscAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmscAddress)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetSmscAddress(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSmscAddress)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ParentDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentDeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn AccountPhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountPhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CellularClass(&self) -> windows_core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceStatus(&self) -> windows_core::Result<SmsDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CalculateLength<P0>(&self, message: P0) -> windows_core::Result<SmsEncodedLength>
    where
        P0: windows_core::Param<ISmsMessageBase>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CalculateLength)(windows_core::Interface::as_raw(this), message.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SendMessageAndGetResultAsync<P0>(&self, message: P0) -> windows_core::Result<windows_future::IAsyncOperation<SmsSendMessageResult>>
    where
        P0: windows_core::Param<ISmsMessageBase>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendMessageAndGetResultAsync)(windows_core::Interface::as_raw(this), message.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceStatusChanged<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SmsDevice2, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceStatusChanged)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDeviceStatusChanged(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDeviceStatusChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromId(deviceid: &windows_core::HSTRING) -> windows_core::Result<SmsDevice2> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefault() -> windows_core::Result<SmsDevice2> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromParentId(parentdeviceid: &windows_core::HSTRING) -> windows_core::Result<SmsDevice2> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromParentId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(parentdeviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISmsDevice2Statics<R, F: FnOnce(&ISmsDevice2Statics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmsDevice2, ISmsDevice2Statics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmsDevice2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsDevice2>();
}
unsafe impl windows_core::Interface for SmsDevice2 {
    type Vtable = <ISmsDevice2 as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsDevice2 as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsDevice2 {
    const NAME: &'static str = "Windows.Devices.Sms.SmsDevice2";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsDeviceMessageStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsDeviceMessageStore, windows_core::IUnknown, windows_core::IInspectable);
impl SmsDeviceMessageStore {
    pub fn DeleteMessageAsync(&self, messageid: u32) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteMessageAsync)(windows_core::Interface::as_raw(this), messageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteMessagesAsync(&self, messagefilter: SmsMessageFilter) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteMessagesAsync)(windows_core::Interface::as_raw(this), messagefilter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMessageAsync(&self, messageid: u32) -> windows_core::Result<windows_future::IAsyncOperation<ISmsMessage>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMessageAsync)(windows_core::Interface::as_raw(this), messageid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMessagesAsync(&self, messagefilter: SmsMessageFilter) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<windows_collections::IVectorView<ISmsMessage>, i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMessagesAsync)(windows_core::Interface::as_raw(this), messagefilter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxMessages(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxMessages)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SmsDeviceMessageStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsDeviceMessageStore>();
}
unsafe impl windows_core::Interface for SmsDeviceMessageStore {
    type Vtable = <ISmsDeviceMessageStore as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsDeviceMessageStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsDeviceMessageStore {
    const NAME: &'static str = "Windows.Devices.Sms.SmsDeviceMessageStore";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SmsDeviceStatus(pub i32);
impl SmsDeviceStatus {
    pub const Off: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const SimNotInserted: Self = Self(2i32);
    pub const BadSim: Self = Self(3i32);
    pub const DeviceFailure: Self = Self(4i32);
    pub const SubscriptionNotActivated: Self = Self(5i32);
    pub const DeviceLocked: Self = Self(6i32);
    pub const DeviceBlocked: Self = Self(7i32);
}
impl windows_core::TypeKind for SmsDeviceStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SmsDeviceStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsDeviceStatus;i4)");
}
windows_core::imp::define_interface!(SmsDeviceStatusChangedEventHandler, SmsDeviceStatusChangedEventHandler_Vtbl, 0x982b1162_3dd7_4618_af89_0c272d5d06d8);
impl windows_core::RuntimeType for SmsDeviceStatusChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl SmsDeviceStatusChangedEventHandler {
    pub fn new<F: Fn(windows_core::Ref<SmsDevice>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = SmsDeviceStatusChangedEventHandlerBox { vtable: &SmsDeviceStatusChangedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SmsDevice>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct SmsDeviceStatusChangedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(C)]
struct SmsDeviceStatusChangedEventHandlerBox<F: Fn(windows_core::Ref<SmsDevice>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const SmsDeviceStatusChangedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: Fn(windows_core::Ref<SmsDevice>) -> windows_core::Result<()> + Send + 'static> SmsDeviceStatusChangedEventHandlerBox<F> {
    const VTABLE: SmsDeviceStatusChangedEventHandler_Vtbl = SmsDeviceStatusChangedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <SmsDeviceStatusChangedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&sender)).into()
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SmsEncodedLength {
    pub SegmentCount: u32,
    pub CharacterCountLastSegment: u32,
    pub CharactersPerSegment: u32,
    pub ByteCountLastSegment: u32,
    pub BytesPerSegment: u32,
}
impl windows_core::TypeKind for SmsEncodedLength {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SmsEncodedLength {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Sms.SmsEncodedLength;u4;u4;u4;u4;u4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SmsEncoding(pub i32);
impl SmsEncoding {
    pub const Unknown: Self = Self(0i32);
    pub const Optimal: Self = Self(1i32);
    pub const SevenBitAscii: Self = Self(2i32);
    pub const Unicode: Self = Self(3i32);
    pub const GsmSevenBit: Self = Self(4i32);
    pub const EightBit: Self = Self(5i32);
    pub const Latin: Self = Self(6i32);
    pub const Korean: Self = Self(7i32);
    pub const IA5: Self = Self(8i32);
    pub const ShiftJis: Self = Self(9i32);
    pub const LatinHebrew: Self = Self(10i32);
}
impl windows_core::TypeKind for SmsEncoding {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SmsEncoding {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsEncoding;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SmsFilterActionType(pub i32);
impl SmsFilterActionType {
    pub const AcceptImmediately: Self = Self(0i32);
    pub const Drop: Self = Self(1i32);
    pub const Peek: Self = Self(2i32);
    pub const Accept: Self = Self(3i32);
}
impl windows_core::TypeKind for SmsFilterActionType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SmsFilterActionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsFilterActionType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsFilterRule(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsFilterRule, windows_core::IUnknown, windows_core::IInspectable);
impl SmsFilterRule {
    pub fn MessageType(&self) -> windows_core::Result<SmsMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ImsiPrefixes(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImsiPrefixes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceIds(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SenderNumbers(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SenderNumbers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TextMessagePrefixes(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextMessagePrefixes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PortNumbers(&self) -> windows_core::Result<windows_collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PortNumbers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CellularClass(&self) -> windows_core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCellularClass(&self, value: CellularClass) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCellularClass)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProtocolIds(&self) -> windows_core::Result<windows_collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TeleserviceIds(&self) -> windows_core::Result<windows_collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TeleserviceIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WapApplicationIds(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WapApplicationIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WapContentTypes(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WapContentTypes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BroadcastTypes(&self) -> windows_core::Result<windows_collections::IVector<SmsBroadcastType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastTypes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BroadcastChannels(&self) -> windows_core::Result<windows_collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastChannels)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFilterRule(messagetype: SmsMessageType) -> windows_core::Result<SmsFilterRule> {
        Self::ISmsFilterRuleFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFilterRule)(windows_core::Interface::as_raw(this), messagetype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISmsFilterRuleFactory<R, F: FnOnce(&ISmsFilterRuleFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmsFilterRule, ISmsFilterRuleFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmsFilterRule {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsFilterRule>();
}
unsafe impl windows_core::Interface for SmsFilterRule {
    type Vtable = <ISmsFilterRule as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsFilterRule as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsFilterRule {
    const NAME: &'static str = "Windows.Devices.Sms.SmsFilterRule";
}
unsafe impl Send for SmsFilterRule {}
unsafe impl Sync for SmsFilterRule {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsFilterRules(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsFilterRules, windows_core::IUnknown, windows_core::IInspectable);
impl SmsFilterRules {
    pub fn ActionType(&self) -> windows_core::Result<SmsFilterActionType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Rules(&self) -> windows_core::Result<windows_collections::IVector<SmsFilterRule>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rules)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFilterRules(actiontype: SmsFilterActionType) -> windows_core::Result<SmsFilterRules> {
        Self::ISmsFilterRulesFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFilterRules)(windows_core::Interface::as_raw(this), actiontype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISmsFilterRulesFactory<R, F: FnOnce(&ISmsFilterRulesFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmsFilterRules, ISmsFilterRulesFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmsFilterRules {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsFilterRules>();
}
unsafe impl windows_core::Interface for SmsFilterRules {
    type Vtable = <ISmsFilterRules as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsFilterRules as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsFilterRules {
    const NAME: &'static str = "Windows.Devices.Sms.SmsFilterRules";
}
unsafe impl Send for SmsFilterRules {}
unsafe impl Sync for SmsFilterRules {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SmsGeographicalScope(pub i32);
impl SmsGeographicalScope {
    pub const None: Self = Self(0i32);
    pub const CellWithImmediateDisplay: Self = Self(1i32);
    pub const LocationArea: Self = Self(2i32);
    pub const Plmn: Self = Self(3i32);
    pub const Cell: Self = Self(4i32);
}
impl windows_core::TypeKind for SmsGeographicalScope {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SmsGeographicalScope {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsGeographicalScope;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SmsMessageClass(pub i32);
impl SmsMessageClass {
    pub const None: Self = Self(0i32);
    pub const Class0: Self = Self(1i32);
    pub const Class1: Self = Self(2i32);
    pub const Class2: Self = Self(3i32);
    pub const Class3: Self = Self(4i32);
}
impl windows_core::TypeKind for SmsMessageClass {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SmsMessageClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsMessageClass;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SmsMessageFilter(pub i32);
impl SmsMessageFilter {
    pub const All: Self = Self(0i32);
    pub const Unread: Self = Self(1i32);
    pub const Read: Self = Self(2i32);
    pub const Sent: Self = Self(3i32);
    pub const Draft: Self = Self(4i32);
}
impl windows_core::TypeKind for SmsMessageFilter {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SmsMessageFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsMessageFilter;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsMessageReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsMessageReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SmsMessageReceivedEventArgs {
    pub fn TextMessage(&self) -> windows_core::Result<SmsTextMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BinaryMessage(&self) -> windows_core::Result<SmsBinaryMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BinaryMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmsMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsMessageReceivedEventArgs>();
}
unsafe impl windows_core::Interface for SmsMessageReceivedEventArgs {
    type Vtable = <ISmsMessageReceivedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsMessageReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sms.SmsMessageReceivedEventArgs";
}
windows_core::imp::define_interface!(SmsMessageReceivedEventHandler, SmsMessageReceivedEventHandler_Vtbl, 0x0b7ad409_ec2d_47ce_a253_732beeebcacd);
impl windows_core::RuntimeType for SmsMessageReceivedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl SmsMessageReceivedEventHandler {
    pub fn new<F: Fn(windows_core::Ref<SmsDevice>, windows_core::Ref<SmsMessageReceivedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = SmsMessageReceivedEventHandlerBox { vtable: &SmsMessageReceivedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SmsDevice>,
        P1: windows_core::Param<SmsMessageReceivedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct SmsMessageReceivedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(C)]
struct SmsMessageReceivedEventHandlerBox<F: Fn(windows_core::Ref<SmsDevice>, windows_core::Ref<SmsMessageReceivedEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const SmsMessageReceivedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: Fn(windows_core::Ref<SmsDevice>, windows_core::Ref<SmsMessageReceivedEventArgs>) -> windows_core::Result<()> + Send + 'static> SmsMessageReceivedEventHandlerBox<F> {
    const VTABLE: SmsMessageReceivedEventHandler_Vtbl = SmsMessageReceivedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <SmsMessageReceivedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsMessageReceivedTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsMessageReceivedTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl SmsMessageReceivedTriggerDetails {
    pub fn MessageType(&self) -> windows_core::Result<SmsMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TextMessage(&self) -> windows_core::Result<SmsTextMessage2> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WapMessage(&self) -> windows_core::Result<SmsWapMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WapMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppMessage(&self) -> windows_core::Result<SmsAppMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BroadcastMessage(&self) -> windows_core::Result<SmsBroadcastMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VoicemailMessage(&self) -> windows_core::Result<SmsVoicemailMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VoicemailMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StatusMessage(&self) -> windows_core::Result<SmsStatusMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Drop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Drop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Accept(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Accept)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for SmsMessageReceivedTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsMessageReceivedTriggerDetails>();
}
unsafe impl windows_core::Interface for SmsMessageReceivedTriggerDetails {
    type Vtable = <ISmsMessageReceivedTriggerDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsMessageReceivedTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsMessageReceivedTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sms.SmsMessageReceivedTriggerDetails";
}
unsafe impl Send for SmsMessageReceivedTriggerDetails {}
unsafe impl Sync for SmsMessageReceivedTriggerDetails {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsMessageRegistration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsMessageRegistration, windows_core::IUnknown, windows_core::IInspectable);
impl SmsMessageRegistration {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Unregister(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Unregister)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MessageReceived<P0>(&self, eventhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SmsMessageRegistration, SmsMessageReceivedTriggerDetails>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageReceived)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMessageReceived(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMessageReceived)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn AllRegistrations() -> windows_core::Result<windows_collections::IVectorView<SmsMessageRegistration>> {
        Self::ISmsMessageRegistrationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllRegistrations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Register<P1>(id: &windows_core::HSTRING, filterrules: P1) -> windows_core::Result<SmsMessageRegistration>
    where
        P1: windows_core::Param<SmsFilterRules>,
    {
        Self::ISmsMessageRegistrationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Register)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), filterrules.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISmsMessageRegistrationStatics<R, F: FnOnce(&ISmsMessageRegistrationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmsMessageRegistration, ISmsMessageRegistrationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmsMessageRegistration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsMessageRegistration>();
}
unsafe impl windows_core::Interface for SmsMessageRegistration {
    type Vtable = <ISmsMessageRegistration as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsMessageRegistration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsMessageRegistration {
    const NAME: &'static str = "Windows.Devices.Sms.SmsMessageRegistration";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SmsMessageType(pub i32);
impl SmsMessageType {
    pub const Binary: Self = Self(0i32);
    pub const Text: Self = Self(1i32);
    pub const Wap: Self = Self(2i32);
    pub const App: Self = Self(3i32);
    pub const Broadcast: Self = Self(4i32);
    pub const Voicemail: Self = Self(5i32);
    pub const Status: Self = Self(6i32);
}
impl windows_core::TypeKind for SmsMessageType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SmsMessageType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsMessageType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SmsModemErrorCode(pub i32);
impl SmsModemErrorCode {
    pub const Other: Self = Self(0i32);
    pub const MessagingNetworkError: Self = Self(1i32);
    pub const SmsOperationNotSupportedByDevice: Self = Self(2i32);
    pub const SmsServiceNotSupportedByNetwork: Self = Self(3i32);
    pub const DeviceFailure: Self = Self(4i32);
    pub const MessageNotEncodedProperly: Self = Self(5i32);
    pub const MessageTooLarge: Self = Self(6i32);
    pub const DeviceNotReady: Self = Self(7i32);
    pub const NetworkNotReady: Self = Self(8i32);
    pub const InvalidSmscAddress: Self = Self(9i32);
    pub const NetworkFailure: Self = Self(10i32);
    pub const FixedDialingNumberRestricted: Self = Self(11i32);
}
impl windows_core::TypeKind for SmsModemErrorCode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SmsModemErrorCode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsModemErrorCode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsReceivedEventDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsReceivedEventDetails, windows_core::IUnknown, windows_core::IInspectable);
impl SmsReceivedEventDetails {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MessageIndex(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = &windows_core::Interface::cast::<ISmsReceivedEventDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BinaryMessage(&self) -> windows_core::Result<SmsBinaryMessage> {
        let this = &windows_core::Interface::cast::<ISmsReceivedEventDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BinaryMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmsReceivedEventDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsReceivedEventDetails>();
}
unsafe impl windows_core::Interface for SmsReceivedEventDetails {
    type Vtable = <ISmsReceivedEventDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsReceivedEventDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsReceivedEventDetails {
    const NAME: &'static str = "Windows.Devices.Sms.SmsReceivedEventDetails";
}
unsafe impl Send for SmsReceivedEventDetails {}
unsafe impl Sync for SmsReceivedEventDetails {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsSendMessageResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsSendMessageResult, windows_core::IUnknown, windows_core::IInspectable);
impl SmsSendMessageResult {
    pub fn IsSuccessful(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSuccessful)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageReferenceNumbers(&self) -> windows_core::Result<windows_collections::IVectorView<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageReferenceNumbers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CellularClass(&self) -> windows_core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ModemErrorCode(&self) -> windows_core::Result<SmsModemErrorCode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModemErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsErrorTransient(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsErrorTransient)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NetworkCauseCode(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkCauseCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TransportFailureCause(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportFailureCause)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SmsSendMessageResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsSendMessageResult>();
}
unsafe impl windows_core::Interface for SmsSendMessageResult {
    type Vtable = <ISmsSendMessageResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsSendMessageResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsSendMessageResult {
    const NAME: &'static str = "Windows.Devices.Sms.SmsSendMessageResult";
}
unsafe impl Send for SmsSendMessageResult {}
unsafe impl Sync for SmsSendMessageResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsStatusMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsStatusMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SmsStatusMessage, ISmsMessageBase);
impl SmsStatusMessage {
    pub fn MessageType(&self) -> windows_core::Result<SmsMessageType> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CellularClass(&self) -> windows_core::Result<CellularClass> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SimIccId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimIccId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn To(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).To)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn From(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).From)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Body(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Body)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageReferenceNumber(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageReferenceNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ServiceCenterTimestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceCenterTimestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DischargeTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DischargeTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SmsStatusMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsStatusMessage>();
}
unsafe impl windows_core::Interface for SmsStatusMessage {
    type Vtable = <ISmsStatusMessage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsStatusMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsStatusMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsStatusMessage";
}
unsafe impl Send for SmsStatusMessage {}
unsafe impl Sync for SmsStatusMessage {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsTextMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsTextMessage, windows_core::IUnknown, windows_core::IInspectable, ISmsTextMessage);
windows_core::imp::required_hierarchy!(SmsTextMessage, ISmsMessage);
impl SmsTextMessage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmsTextMessage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = &windows_core::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PartReferenceId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PartReferenceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PartNumber(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PartNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PartCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PartCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn To(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).To)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTo(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn From(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).From)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetFrom(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFrom)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Body(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Body)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetBody(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBody)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Encoding(&self) -> windows_core::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Encoding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEncoding(&self, value: SmsEncoding) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEncoding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ToBinaryMessages(&self, format: SmsDataFormat) -> windows_core::Result<windows_collections::IVectorView<ISmsBinaryMessage>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToBinaryMessages)(windows_core::Interface::as_raw(this), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromBinaryMessage<P0>(binarymessage: P0) -> windows_core::Result<SmsTextMessage>
    where
        P0: windows_core::Param<SmsBinaryMessage>,
    {
        Self::ISmsTextMessageStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromBinaryMessage)(windows_core::Interface::as_raw(this), binarymessage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromBinaryData(format: SmsDataFormat, value: &[u8]) -> windows_core::Result<SmsTextMessage> {
        Self::ISmsTextMessageStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromBinaryData)(windows_core::Interface::as_raw(this), format, value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISmsTextMessageStatics<R, F: FnOnce(&ISmsTextMessageStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmsTextMessage, ISmsTextMessageStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmsTextMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsTextMessage>();
}
unsafe impl windows_core::Interface for SmsTextMessage {
    type Vtable = <ISmsTextMessage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsTextMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsTextMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsTextMessage";
}
unsafe impl Send for SmsTextMessage {}
unsafe impl Sync for SmsTextMessage {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsTextMessage2(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsTextMessage2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SmsTextMessage2, ISmsMessageBase);
impl SmsTextMessage2 {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmsTextMessage2, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MessageType(&self) -> windows_core::Result<SmsMessageType> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CellularClass(&self) -> windows_core::Result<CellularClass> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SimIccId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimIccId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn To(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).To)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTo(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn From(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).From)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Body(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Body)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetBody(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBody)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Encoding(&self) -> windows_core::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Encoding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEncoding(&self, value: SmsEncoding) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEncoding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CallbackNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallbackNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCallbackNumber(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCallbackNumber)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsDeliveryNotificationEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDeliveryNotificationEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDeliveryNotificationEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDeliveryNotificationEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RetryAttemptCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetryAttemptCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRetryAttemptCount(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRetryAttemptCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TeleserviceId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TeleserviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtocolId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtocolId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SmsTextMessage2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsTextMessage2>();
}
unsafe impl windows_core::Interface for SmsTextMessage2 {
    type Vtable = <ISmsTextMessage2 as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsTextMessage2 as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsTextMessage2 {
    const NAME: &'static str = "Windows.Devices.Sms.SmsTextMessage2";
}
unsafe impl Send for SmsTextMessage2 {}
unsafe impl Sync for SmsTextMessage2 {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsVoicemailMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsVoicemailMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SmsVoicemailMessage, ISmsMessageBase);
impl SmsVoicemailMessage {
    pub fn MessageType(&self) -> windows_core::Result<SmsMessageType> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CellularClass(&self) -> windows_core::Result<CellularClass> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SimIccId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimIccId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn To(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).To)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Body(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Body)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn MessageCount(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmsVoicemailMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsVoicemailMessage>();
}
unsafe impl windows_core::Interface for SmsVoicemailMessage {
    type Vtable = <ISmsVoicemailMessage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsVoicemailMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsVoicemailMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsVoicemailMessage";
}
unsafe impl Send for SmsVoicemailMessage {}
unsafe impl Sync for SmsVoicemailMessage {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SmsWapMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsWapMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SmsWapMessage, ISmsMessageBase);
impl SmsWapMessage {
    pub fn MessageType(&self) -> windows_core::Result<SmsMessageType> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CellularClass(&self) -> windows_core::Result<CellularClass> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageClass(&self) -> windows_core::Result<SmsMessageClass> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SimIccId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimIccId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn To(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).To)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn From(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).From)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ApplicationId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplicationId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn BinaryBody(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BinaryBody)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Headers(&self) -> windows_core::Result<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Headers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmsWapMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmsWapMessage>();
}
unsafe impl windows_core::Interface for SmsWapMessage {
    type Vtable = <ISmsWapMessage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISmsWapMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsWapMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsWapMessage";
}
unsafe impl Send for SmsWapMessage {}
unsafe impl Sync for SmsWapMessage {}
