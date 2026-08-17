pub trait IErrorReportingSettings_Impl: Sized {
    fn SetErrorOptions(&self, value: ErrorOptions) -> windows_core::Result<()>;
    fn GetErrorOptions(&self) -> windows_core::Result<ErrorOptions>;
}
impl windows_core::RuntimeName for IErrorReportingSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IErrorReportingSettings";
}
impl IErrorReportingSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IErrorReportingSettings_Impl, const OFFSET: isize>() -> IErrorReportingSettings_Vtbl {
        unsafe extern "system" fn SetErrorOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IErrorReportingSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ErrorOptions) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IErrorReportingSettings_Impl::SetErrorOptions(this, value).into()
        }
        unsafe extern "system" fn GetErrorOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IErrorReportingSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ErrorOptions) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IErrorReportingSettings_Impl::GetErrorOptions(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IErrorReportingSettings, OFFSET>(),
            SetErrorOptions: SetErrorOptions::<Identity, Impl, OFFSET>,
            GetErrorOptions: GetErrorOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IErrorReportingSettings as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Storage")]
pub trait IFileLoggingSession_Impl: Sized + super::IClosable_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn AddLoggingChannel(&self, loggingchannel: Option<&ILoggingChannel>) -> windows_core::Result<()>;
    fn AddLoggingChannelWithLevel(&self, loggingchannel: Option<&ILoggingChannel>, maxlevel: LoggingLevel) -> windows_core::Result<()>;
    fn RemoveLoggingChannel(&self, loggingchannel: Option<&ILoggingChannel>) -> windows_core::Result<()>;
    fn CloseAndSaveToFileAsync(&self) -> windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn LogFileGenerated(&self, handler: Option<&super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>) -> windows_core::Result<super::EventRegistrationToken>;
    fn RemoveLogFileGenerated(&self, token: &super::EventRegistrationToken) -> windows_core::Result<()>;
}
#[cfg(feature = "Storage")]
impl windows_core::RuntimeName for IFileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IFileLoggingSession";
}
#[cfg(feature = "Storage")]
impl IFileLoggingSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>() -> IFileLoggingSession_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFileLoggingSession_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingChannel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingchannel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFileLoggingSession_Impl::AddLoggingChannel(this, windows_core::from_raw_borrowed(&loggingchannel)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingchannel: *mut core::ffi::c_void, maxlevel: LoggingLevel) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFileLoggingSession_Impl::AddLoggingChannelWithLevel(this, windows_core::from_raw_borrowed(&loggingchannel), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingchannel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFileLoggingSession_Impl::RemoveLoggingChannel(this, windows_core::from_raw_borrowed(&loggingchannel)).into()
        }
        unsafe extern "system" fn CloseAndSaveToFileAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFileLoggingSession_Impl::CloseAndSaveToFileAsync(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogFileGenerated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFileLoggingSession_Impl::LogFileGenerated(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLogFileGenerated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFileLoggingSession_Impl::RemoveLogFileGenerated(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileLoggingSession, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            AddLoggingChannel: AddLoggingChannel::<Identity, Impl, OFFSET>,
            AddLoggingChannelWithLevel: AddLoggingChannelWithLevel::<Identity, Impl, OFFSET>,
            RemoveLoggingChannel: RemoveLoggingChannel::<Identity, Impl, OFFSET>,
            CloseAndSaveToFileAsync: CloseAndSaveToFileAsync::<Identity, Impl, OFFSET>,
            LogFileGenerated: LogFileGenerated::<Identity, Impl, OFFSET>,
            RemoveLogFileGenerated: RemoveLogFileGenerated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileLoggingSession as windows_core::Interface>::IID
    }
}
pub trait ILoggingChannel_Impl: Sized + super::IClosable_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Enabled(&self) -> windows_core::Result<bool>;
    fn Level(&self) -> windows_core::Result<LoggingLevel>;
    fn LogMessage(&self, eventstring: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn LogMessageWithLevel(&self, eventstring: &windows_core::HSTRING, level: LoggingLevel) -> windows_core::Result<()>;
    fn LogValuePair(&self, value1: &windows_core::HSTRING, value2: i32) -> windows_core::Result<()>;
    fn LogValuePairWithLevel(&self, value1: &windows_core::HSTRING, value2: i32, level: LoggingLevel) -> windows_core::Result<()>;
    fn LoggingEnabled(&self, handler: Option<&super::TypedEventHandler<ILoggingChannel, windows_core::IInspectable>>) -> windows_core::Result<super::EventRegistrationToken>;
    fn RemoveLoggingEnabled(&self, token: &super::EventRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ILoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannel";
}
impl ILoggingChannel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>() -> ILoggingChannel_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingChannel_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingChannel_Impl::Enabled(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Level<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut LoggingLevel) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingChannel_Impl::Level(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventstring: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoggingChannel_Impl::LogMessage(this, core::mem::transmute(&eventstring)).into()
        }
        unsafe extern "system" fn LogMessageWithLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventstring: core::mem::MaybeUninit<windows_core::HSTRING>, level: LoggingLevel) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoggingChannel_Impl::LogMessageWithLevel(this, core::mem::transmute(&eventstring), level).into()
        }
        unsafe extern "system" fn LogValuePair<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value1: core::mem::MaybeUninit<windows_core::HSTRING>, value2: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoggingChannel_Impl::LogValuePair(this, core::mem::transmute(&value1), value2).into()
        }
        unsafe extern "system" fn LogValuePairWithLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value1: core::mem::MaybeUninit<windows_core::HSTRING>, value2: i32, level: LoggingLevel) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoggingChannel_Impl::LogValuePairWithLevel(this, core::mem::transmute(&value1), value2, level).into()
        }
        unsafe extern "system" fn LoggingEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingChannel_Impl::LoggingEnabled(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoggingEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoggingChannel_Impl::RemoveLoggingEnabled(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILoggingChannel, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            Level: Level::<Identity, Impl, OFFSET>,
            LogMessage: LogMessage::<Identity, Impl, OFFSET>,
            LogMessageWithLevel: LogMessageWithLevel::<Identity, Impl, OFFSET>,
            LogValuePair: LogValuePair::<Identity, Impl, OFFSET>,
            LogValuePairWithLevel: LogValuePairWithLevel::<Identity, Impl, OFFSET>,
            LoggingEnabled: LoggingEnabled::<Identity, Impl, OFFSET>,
            RemoveLoggingEnabled: RemoveLoggingEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILoggingChannel as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Storage")]
pub trait ILoggingSession_Impl: Sized + super::IClosable_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SaveToFileAsync(&self, folder: Option<&super::super::Storage::IStorageFolder>, filename: &windows_core::HSTRING) -> windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn AddLoggingChannel(&self, loggingchannel: Option<&ILoggingChannel>) -> windows_core::Result<()>;
    fn AddLoggingChannelWithLevel(&self, loggingchannel: Option<&ILoggingChannel>, maxlevel: LoggingLevel) -> windows_core::Result<()>;
    fn RemoveLoggingChannel(&self, loggingchannel: Option<&ILoggingChannel>) -> windows_core::Result<()>;
}
#[cfg(feature = "Storage")]
impl windows_core::RuntimeName for ILoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingSession";
}
#[cfg(feature = "Storage")]
impl ILoggingSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>() -> ILoggingSession_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingSession_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToFileAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut core::ffi::c_void, filename: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingSession_Impl::SaveToFileAsync(this, windows_core::from_raw_borrowed(&folder), core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingChannel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingchannel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoggingSession_Impl::AddLoggingChannel(this, windows_core::from_raw_borrowed(&loggingchannel)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingchannel: *mut core::ffi::c_void, maxlevel: LoggingLevel) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoggingSession_Impl::AddLoggingChannelWithLevel(this, windows_core::from_raw_borrowed(&loggingchannel), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingchannel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoggingSession_Impl::RemoveLoggingChannel(this, windows_core::from_raw_borrowed(&loggingchannel)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILoggingSession, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SaveToFileAsync: SaveToFileAsync::<Identity, Impl, OFFSET>,
            AddLoggingChannel: AddLoggingChannel::<Identity, Impl, OFFSET>,
            AddLoggingChannelWithLevel: AddLoggingChannelWithLevel::<Identity, Impl, OFFSET>,
            RemoveLoggingChannel: RemoveLoggingChannel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILoggingSession as windows_core::Interface>::IID
    }
}
pub trait ILoggingTarget_Impl: Sized {
    fn IsEnabled(&self) -> windows_core::Result<bool>;
    fn IsEnabledWithLevel(&self, level: LoggingLevel) -> windows_core::Result<bool>;
    fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> windows_core::Result<bool>;
    fn LogEvent(&self, eventname: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn LogEventWithFields(&self, eventname: &windows_core::HSTRING, fields: Option<&LoggingFields>) -> windows_core::Result<()>;
    fn LogEventWithFieldsAndLevel(&self, eventname: &windows_core::HSTRING, fields: Option<&LoggingFields>, level: LoggingLevel) -> windows_core::Result<()>;
    fn LogEventWithFieldsAndOptions(&self, eventname: &windows_core::HSTRING, fields: Option<&LoggingFields>, level: LoggingLevel, options: Option<&LoggingOptions>) -> windows_core::Result<()>;
    fn StartActivity(&self, starteventname: &windows_core::HSTRING) -> windows_core::Result<LoggingActivity>;
    fn StartActivityWithFields(&self, starteventname: &windows_core::HSTRING, fields: Option<&LoggingFields>) -> windows_core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndLevel(&self, starteventname: &windows_core::HSTRING, fields: Option<&LoggingFields>, level: LoggingLevel) -> windows_core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndOptions(&self, starteventname: &windows_core::HSTRING, fields: Option<&LoggingFields>, level: LoggingLevel, options: Option<&LoggingOptions>) -> windows_core::Result<LoggingActivity>;
}
impl windows_core::RuntimeName for ILoggingTarget {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingTarget";
}
impl ILoggingTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>() -> ILoggingTarget_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingTarget_Impl::IsEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledWithLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: LoggingLevel, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingTarget_Impl::IsEnabledWithLevel(this, level) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledWithLevelAndKeywords<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: LoggingLevel, keywords: i64, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingTarget_Impl::IsEnabledWithLevelAndKeywords(this, level, keywords) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventname: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoggingTarget_Impl::LogEvent(this, core::mem::transmute(&eventname)).into()
        }
        unsafe extern "system" fn LogEventWithFields<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventname: core::mem::MaybeUninit<windows_core::HSTRING>, fields: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoggingTarget_Impl::LogEventWithFields(this, core::mem::transmute(&eventname), windows_core::from_raw_borrowed(&fields)).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventname: core::mem::MaybeUninit<windows_core::HSTRING>, fields: *mut core::ffi::c_void, level: LoggingLevel) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoggingTarget_Impl::LogEventWithFieldsAndLevel(this, core::mem::transmute(&eventname), windows_core::from_raw_borrowed(&fields), level).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventname: core::mem::MaybeUninit<windows_core::HSTRING>, fields: *mut core::ffi::c_void, level: LoggingLevel, options: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoggingTarget_Impl::LogEventWithFieldsAndOptions(this, core::mem::transmute(&eventname), windows_core::from_raw_borrowed(&fields), level, windows_core::from_raw_borrowed(&options)).into()
        }
        unsafe extern "system" fn StartActivity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starteventname: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingTarget_Impl::StartActivity(this, core::mem::transmute(&starteventname)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFields<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starteventname: core::mem::MaybeUninit<windows_core::HSTRING>, fields: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingTarget_Impl::StartActivityWithFields(this, core::mem::transmute(&starteventname), windows_core::from_raw_borrowed(&fields)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFieldsAndLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starteventname: core::mem::MaybeUninit<windows_core::HSTRING>, fields: *mut core::ffi::c_void, level: LoggingLevel, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingTarget_Impl::StartActivityWithFieldsAndLevel(this, core::mem::transmute(&starteventname), windows_core::from_raw_borrowed(&fields), level) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFieldsAndOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starteventname: core::mem::MaybeUninit<windows_core::HSTRING>, fields: *mut core::ffi::c_void, level: LoggingLevel, options: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoggingTarget_Impl::StartActivityWithFieldsAndOptions(this, core::mem::transmute(&starteventname), windows_core::from_raw_borrowed(&fields), level, windows_core::from_raw_borrowed(&options)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILoggingTarget, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            IsEnabledWithLevel: IsEnabledWithLevel::<Identity, Impl, OFFSET>,
            IsEnabledWithLevelAndKeywords: IsEnabledWithLevelAndKeywords::<Identity, Impl, OFFSET>,
            LogEvent: LogEvent::<Identity, Impl, OFFSET>,
            LogEventWithFields: LogEventWithFields::<Identity, Impl, OFFSET>,
            LogEventWithFieldsAndLevel: LogEventWithFieldsAndLevel::<Identity, Impl, OFFSET>,
            LogEventWithFieldsAndOptions: LogEventWithFieldsAndOptions::<Identity, Impl, OFFSET>,
            StartActivity: StartActivity::<Identity, Impl, OFFSET>,
            StartActivityWithFields: StartActivityWithFields::<Identity, Impl, OFFSET>,
            StartActivityWithFieldsAndLevel: StartActivityWithFieldsAndLevel::<Identity, Impl, OFFSET>,
            StartActivityWithFieldsAndOptions: StartActivityWithFieldsAndOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILoggingTarget as windows_core::Interface>::IID
    }
}
