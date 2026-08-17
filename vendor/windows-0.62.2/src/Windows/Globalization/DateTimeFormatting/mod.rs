#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateTimeFormatter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DateTimeFormatter, windows_core::IUnknown, windows_core::IInspectable);
impl DateTimeFormatter {
    pub fn Languages(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Languages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Calendar(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Calendar)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Clock(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clock)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn NumeralSystem(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumeralSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetNumeralSystem(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNumeralSystem)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Patterns(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Patterns)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Template(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Template)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Format(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IncludeYear(&self) -> windows_core::Result<YearFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeYear)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IncludeMonth(&self) -> windows_core::Result<MonthFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeMonth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IncludeDayOfWeek(&self) -> windows_core::Result<DayOfWeekFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeDayOfWeek)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IncludeDay(&self) -> windows_core::Result<DayFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeDay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IncludeHour(&self) -> windows_core::Result<HourFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeHour)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IncludeMinute(&self) -> windows_core::Result<MinuteFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeMinute)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IncludeSecond(&self) -> windows_core::Result<SecondFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedGeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatUsingTimeZone(&self, datetime: super::super::Foundation::DateTime, timezoneid: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IDateTimeFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUsingTimeZone)(windows_core::Interface::as_raw(this), datetime, core::mem::transmute_copy(timezoneid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateDateTimeFormatter(formattemplate: &windows_core::HSTRING) -> windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDateTimeFormatter)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(formattemplate), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDateTimeFormatterLanguages<P1>(formattemplate: &windows_core::HSTRING, languages: P1) -> windows_core::Result<DateTimeFormatter>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDateTimeFormatterLanguages)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(formattemplate), languages.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDateTimeFormatterContext<P1>(formattemplate: &windows_core::HSTRING, languages: P1, geographicregion: &windows_core::HSTRING, calendar: &windows_core::HSTRING, clock: &windows_core::HSTRING) -> windows_core::Result<DateTimeFormatter>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDateTimeFormatterContext)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(formattemplate), languages.param().abi(), core::mem::transmute_copy(geographicregion), core::mem::transmute_copy(calendar), core::mem::transmute_copy(clock), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDateTimeFormatterDate(yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat) -> windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDateTimeFormatterDate)(windows_core::Interface::as_raw(this), yearformat, monthformat, dayformat, dayofweekformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDateTimeFormatterTime(hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat) -> windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDateTimeFormatterTime)(windows_core::Interface::as_raw(this), hourformat, minuteformat, secondformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDateTimeFormatterDateTimeLanguages<P7>(yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: P7) -> windows_core::Result<DateTimeFormatter>
    where
        P7: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDateTimeFormatterDateTimeLanguages)(windows_core::Interface::as_raw(this), yearformat, monthformat, dayformat, dayofweekformat, hourformat, minuteformat, secondformat, languages.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDateTimeFormatterDateTimeContext<P7>(yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: P7, geographicregion: &windows_core::HSTRING, calendar: &windows_core::HSTRING, clock: &windows_core::HSTRING) -> windows_core::Result<DateTimeFormatter>
    where
        P7: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDateTimeFormatterDateTimeContext)(windows_core::Interface::as_raw(this), yearformat, monthformat, dayformat, dayofweekformat, hourformat, minuteformat, secondformat, languages.param().abi(), core::mem::transmute_copy(geographicregion), core::mem::transmute_copy(calendar), core::mem::transmute_copy(clock), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LongDate() -> windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LongDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LongTime() -> windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LongTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShortDate() -> windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShortDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShortTime() -> windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShortTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IDateTimeFormatterFactory<R, F: FnOnce(&IDateTimeFormatterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DateTimeFormatter, IDateTimeFormatterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IDateTimeFormatterStatics<R, F: FnOnce(&IDateTimeFormatterStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DateTimeFormatter, IDateTimeFormatterStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DateTimeFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDateTimeFormatter>();
}
unsafe impl windows_core::Interface for DateTimeFormatter {
    type Vtable = <IDateTimeFormatter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDateTimeFormatter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DateTimeFormatter {
    const NAME: &'static str = "Windows.Globalization.DateTimeFormatting.DateTimeFormatter";
}
unsafe impl Send for DateTimeFormatter {}
unsafe impl Sync for DateTimeFormatter {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DayFormat(pub i32);
impl DayFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl windows_core::TypeKind for DayFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DayFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.DayFormat;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DayOfWeekFormat(pub i32);
impl DayOfWeekFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Abbreviated: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl windows_core::TypeKind for DayOfWeekFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DayOfWeekFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.DayOfWeekFormat;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HourFormat(pub i32);
impl HourFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl windows_core::TypeKind for HourFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HourFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.HourFormat;i4)");
}
windows_core::imp::define_interface!(IDateTimeFormatter, IDateTimeFormatter_Vtbl, 0x95eeca10_73e0_4e4b_a183_3d6ad0ba35ec);
impl windows_core::RuntimeType for IDateTimeFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GeographicRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Calendar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Patterns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Template: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IncludeYear: unsafe extern "system" fn(*mut core::ffi::c_void, *mut YearFormat) -> windows_core::HRESULT,
    pub IncludeMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MonthFormat) -> windows_core::HRESULT,
    pub IncludeDayOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DayOfWeekFormat) -> windows_core::HRESULT,
    pub IncludeDay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DayFormat) -> windows_core::HRESULT,
    pub IncludeHour: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HourFormat) -> windows_core::HRESULT,
    pub IncludeMinute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MinuteFormat) -> windows_core::HRESULT,
    pub IncludeSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SecondFormat) -> windows_core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResolvedGeographicRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDateTimeFormatter2, IDateTimeFormatter2_Vtbl, 0x27c91a86_bdaa_4fd0_9e36_671d5aa5ee03);
impl windows_core::RuntimeType for IDateTimeFormatter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FormatUsingTimeZone: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDateTimeFormatterFactory, IDateTimeFormatterFactory_Vtbl, 0xec8d8a53_1a2e_412d_8815_3b745fb1a2a0);
impl windows_core::RuntimeType for IDateTimeFormatterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateDateTimeFormatter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDateTimeFormatterLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDateTimeFormatterContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDateTimeFormatterDate: unsafe extern "system" fn(*mut core::ffi::c_void, YearFormat, MonthFormat, DayFormat, DayOfWeekFormat, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDateTimeFormatterTime: unsafe extern "system" fn(*mut core::ffi::c_void, HourFormat, MinuteFormat, SecondFormat, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDateTimeFormatterDateTimeLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, YearFormat, MonthFormat, DayFormat, DayOfWeekFormat, HourFormat, MinuteFormat, SecondFormat, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDateTimeFormatterDateTimeContext: unsafe extern "system" fn(*mut core::ffi::c_void, YearFormat, MonthFormat, DayFormat, DayOfWeekFormat, HourFormat, MinuteFormat, SecondFormat, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDateTimeFormatterStatics, IDateTimeFormatterStatics_Vtbl, 0xbfcde7c0_df4c_4a2e_9012_f47daf3f1212);
impl windows_core::RuntimeType for IDateTimeFormatterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LongDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LongTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShortDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShortTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MinuteFormat(pub i32);
impl MinuteFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl windows_core::TypeKind for MinuteFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for MinuteFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.MinuteFormat;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MonthFormat(pub i32);
impl MonthFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Abbreviated: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
    pub const Numeric: Self = Self(4i32);
}
impl windows_core::TypeKind for MonthFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for MonthFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.MonthFormat;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SecondFormat(pub i32);
impl SecondFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl windows_core::TypeKind for SecondFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SecondFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.SecondFormat;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct YearFormat(pub i32);
impl YearFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Abbreviated: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl windows_core::TypeKind for YearFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for YearFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.YearFormat;i4)");
}
