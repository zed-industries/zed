pub trait IErrorReportingSettings_Impl: Sized {
    fn SetErrorOptions(&self, value: ErrorOptions) -> windows_core::Result<()>;
    fn GetErrorOptions(&self) -> windows_core::Result<ErrorOptions>;
}
impl windows_core::RuntimeName for IErrorReportingSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IErrorReportingSettings";
}
impl IErrorReportingSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IErrorReportingSettings_Vtbl
    where
        Identity: IErrorReportingSettings_Impl,
    {
        unsafe extern "system" fn SetErrorOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ErrorOptions) -> windows_core::HRESULT
        where
            Identity: IErrorReportingSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IErrorReportingSettings_Impl::SetErrorOptions(this, value).into()
        }
        unsafe extern "system" fn GetErrorOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ErrorOptions) -> windows_core::HRESULT
        where
            Identity: IErrorReportingSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IErrorReportingSettings_Impl::GetErrorOptions(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IErrorReportingSettings, OFFSET>(),
            SetErrorOptions: SetErrorOptions::<Identity, OFFSET>,
            GetErrorOptions: GetErrorOptions::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFileLoggingSession_Vtbl
    where
        Identity: IFileLoggingSession_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IFileLoggingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileLoggingSession_Impl::Name(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingchannel: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFileLoggingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileLoggingSession_Impl::AddLoggingChannel(this, windows_core::from_raw_borrowed(&loggingchannel)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingchannel: *mut core::ffi::c_void, maxlevel: LoggingLevel) -> windows_core::HRESULT
        where
            Identity: IFileLoggingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileLoggingSession_Impl::AddLoggingChannelWithLevel(this, windows_core::from_raw_borrowed(&loggingchannel), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingchannel: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFileLoggingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileLoggingSession_Impl::RemoveLoggingChannel(this, windows_core::from_raw_borrowed(&loggingchannel)).into()
        }
        unsafe extern "system" fn CloseAndSaveToFileAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFileLoggingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileLoggingSession_Impl::CloseAndSaveToFileAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogFileGenerated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IFileLoggingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileLoggingSession_Impl::LogFileGenerated(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLogFileGenerated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IFileLoggingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileLoggingSession_Impl::RemoveLogFileGenerated(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFileLoggingSession, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            AddLoggingChannel: AddLoggingChannel::<Identity, OFFSET>,
            AddLoggingChannelWithLevel: AddLoggingChannelWithLevel::<Identity, OFFSET>,
            RemoveLoggingChannel: RemoveLoggingChannel::<Identity, OFFSET>,
            CloseAndSaveToFileAsync: CloseAndSaveToFileAsync::<Identity, OFFSET>,
            LogFileGenerated: LogFileGenerated::<Identity, OFFSET>,
            RemoveLogFileGenerated: RemoveLogFileGenerated::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILoggingChannel_Vtbl
    where
        Identity: ILoggingChannel_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ILoggingChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingChannel_Impl::Name(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ILoggingChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingChannel_Impl::Enabled(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Level<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut LoggingLevel) -> windows_core::HRESULT
        where
            Identity: ILoggingChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingChannel_Impl::Level(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventstring: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ILoggingChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoggingChannel_Impl::LogMessage(this, core::mem::transmute(&eventstring)).into()
        }
        unsafe extern "system" fn LogMessageWithLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventstring: core::mem::MaybeUninit<windows_core::HSTRING>, level: LoggingLevel) -> windows_core::HRESULT
        where
            Identity: ILoggingChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoggingChannel_Impl::LogMessageWithLevel(this, core::mem::transmute(&eventstring), level).into()
        }
        unsafe extern "system" fn LogValuePair<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value1: core::mem::MaybeUninit<windows_core::HSTRING>, value2: i32) -> windows_core::HRESULT
        where
            Identity: ILoggingChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoggingChannel_Impl::LogValuePair(this, core::mem::transmute(&value1), value2).into()
        }
        unsafe extern "system" fn LogValuePairWithLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value1: core::mem::MaybeUninit<windows_core::HSTRING>, value2: i32, level: LoggingLevel) -> windows_core::HRESULT
        where
            Identity: ILoggingChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoggingChannel_Impl::LogValuePairWithLevel(this, core::mem::transmute(&value1), value2, level).into()
        }
        unsafe extern "system" fn LoggingEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ILoggingChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingChannel_Impl::LoggingEnabled(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoggingEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ILoggingChannel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoggingChannel_Impl::RemoveLoggingEnabled(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILoggingChannel, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            Level: Level::<Identity, OFFSET>,
            LogMessage: LogMessage::<Identity, OFFSET>,
            LogMessageWithLevel: LogMessageWithLevel::<Identity, OFFSET>,
            LogValuePair: LogValuePair::<Identity, OFFSET>,
            LogValuePairWithLevel: LogValuePairWithLevel::<Identity, OFFSET>,
            LoggingEnabled: LoggingEnabled::<Identity, OFFSET>,
            RemoveLoggingEnabled: RemoveLoggingEnabled::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILoggingSession_Vtbl
    where
        Identity: ILoggingSession_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ILoggingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingSession_Impl::Name(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToFileAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut core::ffi::c_void, filename: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoggingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingSession_Impl::SaveToFileAsync(this, windows_core::from_raw_borrowed(&folder), core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingchannel: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoggingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoggingSession_Impl::AddLoggingChannel(this, windows_core::from_raw_borrowed(&loggingchannel)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingchannel: *mut core::ffi::c_void, maxlevel: LoggingLevel) -> windows_core::HRESULT
        where
            Identity: ILoggingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoggingSession_Impl::AddLoggingChannelWithLevel(this, windows_core::from_raw_borrowed(&loggingchannel), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingchannel: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoggingSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoggingSession_Impl::RemoveLoggingChannel(this, windows_core::from_raw_borrowed(&loggingchannel)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILoggingSession, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SaveToFileAsync: SaveToFileAsync::<Identity, OFFSET>,
            AddLoggingChannel: AddLoggingChannel::<Identity, OFFSET>,
            AddLoggingChannelWithLevel: AddLoggingChannelWithLevel::<Identity, OFFSET>,
            RemoveLoggingChannel: RemoveLoggingChannel::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILoggingTarget_Vtbl
    where
        Identity: ILoggingTarget_Impl,
    {
        unsafe extern "system" fn IsEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ILoggingTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingTarget_Impl::IsEnabled(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledWithLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: LoggingLevel, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ILoggingTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingTarget_Impl::IsEnabledWithLevel(this, level) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledWithLevelAndKeywords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: LoggingLevel, keywords: i64, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ILoggingTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingTarget_Impl::IsEnabledWithLevelAndKeywords(this, level, keywords) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventname: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: ILoggingTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoggingTarget_Impl::LogEvent(this, core::mem::transmute(&eventname)).into()
        }
        unsafe extern "system" fn LogEventWithFields<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventname: core::mem::MaybeUninit<windows_core::HSTRING>, fields: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoggingTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoggingTarget_Impl::LogEventWithFields(this, core::mem::transmute(&eventname), windows_core::from_raw_borrowed(&fields)).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventname: core::mem::MaybeUninit<windows_core::HSTRING>, fields: *mut core::ffi::c_void, level: LoggingLevel) -> windows_core::HRESULT
        where
            Identity: ILoggingTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoggingTarget_Impl::LogEventWithFieldsAndLevel(this, core::mem::transmute(&eventname), windows_core::from_raw_borrowed(&fields), level).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventname: core::mem::MaybeUninit<windows_core::HSTRING>, fields: *mut core::ffi::c_void, level: LoggingLevel, options: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoggingTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoggingTarget_Impl::LogEventWithFieldsAndOptions(this, core::mem::transmute(&eventname), windows_core::from_raw_borrowed(&fields), level, windows_core::from_raw_borrowed(&options)).into()
        }
        unsafe extern "system" fn StartActivity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starteventname: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoggingTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingTarget_Impl::StartActivity(this, core::mem::transmute(&starteventname)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFields<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starteventname: core::mem::MaybeUninit<windows_core::HSTRING>, fields: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoggingTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingTarget_Impl::StartActivityWithFields(this, core::mem::transmute(&starteventname), windows_core::from_raw_borrowed(&fields)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFieldsAndLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starteventname: core::mem::MaybeUninit<windows_core::HSTRING>, fields: *mut core::ffi::c_void, level: LoggingLevel, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoggingTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingTarget_Impl::StartActivityWithFieldsAndLevel(this, core::mem::transmute(&starteventname), windows_core::from_raw_borrowed(&fields), level) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFieldsAndOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, starteventname: core::mem::MaybeUninit<windows_core::HSTRING>, fields: *mut core::ffi::c_void, level: LoggingLevel, options: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoggingTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoggingTarget_Impl::StartActivityWithFieldsAndOptions(this, core::mem::transmute(&starteventname), windows_core::from_raw_borrowed(&fields), level, windows_core::from_raw_borrowed(&options)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILoggingTarget, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            IsEnabledWithLevel: IsEnabledWithLevel::<Identity, OFFSET>,
            IsEnabledWithLevelAndKeywords: IsEnabledWithLevelAndKeywords::<Identity, OFFSET>,
            LogEvent: LogEvent::<Identity, OFFSET>,
            LogEventWithFields: LogEventWithFields::<Identity, OFFSET>,
            LogEventWithFieldsAndLevel: LogEventWithFieldsAndLevel::<Identity, OFFSET>,
            LogEventWithFieldsAndOptions: LogEventWithFieldsAndOptions::<Identity, OFFSET>,
            StartActivity: StartActivity::<Identity, OFFSET>,
            StartActivityWithFields: StartActivityWithFields::<Identity, OFFSET>,
            StartActivityWithFieldsAndLevel: StartActivityWithFieldsAndLevel::<Identity, OFFSET>,
            StartActivityWithFieldsAndOptions: StartActivityWithFieldsAndOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILoggingTarget as windows_core::Interface>::IID
    }
}
