#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyFormatter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CurrencyFormatter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CurrencyFormatter, INumberFormatter, INumberFormatter2, INumberFormatterOptions, INumberParser, INumberRounderOption, ISignedZeroOption, ISignificantDigitsOption);
impl CurrencyFormatter {
    pub fn Currency(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Currency)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCurrency(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCurrency)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Mode(&self) -> windows_core::Result<CurrencyFormatterMode> {
        let this = &windows_core::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: CurrencyFormatterMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ApplyRoundingForCurrency(&self, roundingalgorithm: RoundingAlgorithm) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ApplyRoundingForCurrency)(windows_core::Interface::as_raw(this), roundingalgorithm).ok() }
    }
    pub fn CreateCurrencyFormatterCode(currencycode: &windows_core::HSTRING) -> windows_core::Result<CurrencyFormatter> {
        Self::ICurrencyFormatterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCurrencyFormatterCode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(currencycode), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateCurrencyFormatterCodeContext<P1>(currencycode: &windows_core::HSTRING, languages: P1, geographicregion: &windows_core::HSTRING) -> windows_core::Result<CurrencyFormatter>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ICurrencyFormatterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCurrencyFormatterCodeContext)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(currencycode), languages.param().abi(), core::mem::transmute_copy(geographicregion), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatInt2(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Languages(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Languages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IntegerDigits(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IntegerDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIntegerDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FractionDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFractionDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGrouped)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsGrouped)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumeralSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetNumeralSystem(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNumeralSystem)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedGeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ParseInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<i64>> {
        let this = &windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseInt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParseUInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = &windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseUInt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParseDouble(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseDouble)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NumberRounder(&self) -> windows_core::Result<INumberRounder> {
        let this = &windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberRounder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNumberRounder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INumberRounder>,
    {
        let this = &windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNumberRounder)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsZeroSigned(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsZeroSigned)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsZeroSigned)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignificantDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSignificantDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    fn ICurrencyFormatterFactory<R, F: FnOnce(&ICurrencyFormatterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CurrencyFormatter, ICurrencyFormatterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CurrencyFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICurrencyFormatter>();
}
unsafe impl windows_core::Interface for CurrencyFormatter {
    type Vtable = <ICurrencyFormatter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICurrencyFormatter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CurrencyFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.CurrencyFormatter";
}
unsafe impl Send for CurrencyFormatter {}
unsafe impl Sync for CurrencyFormatter {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CurrencyFormatterMode(pub i32);
impl CurrencyFormatterMode {
    pub const UseSymbol: Self = Self(0i32);
    pub const UseCurrencyCode: Self = Self(1i32);
}
impl windows_core::TypeKind for CurrencyFormatterMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CurrencyFormatterMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.CurrencyFormatterMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecimalFormatter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DecimalFormatter, windows_core::IUnknown, windows_core::IInspectable, INumberFormatter);
windows_core::imp::required_hierarchy!(DecimalFormatter, INumberFormatter2, INumberFormatterOptions, INumberParser, INumberRounderOption, ISignedZeroOption, ISignificantDigitsOption);
impl DecimalFormatter {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DecimalFormatter, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CreateDecimalFormatter<P0>(languages: P0, geographicregion: &windows_core::HSTRING) -> windows_core::Result<DecimalFormatter>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IDecimalFormatterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDecimalFormatter)(windows_core::Interface::as_raw(this), languages.param().abi(), core::mem::transmute_copy(geographicregion), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatInt2(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Languages(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Languages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IntegerDigits(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IntegerDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIntegerDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FractionDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFractionDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGrouped)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsGrouped)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumeralSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetNumeralSystem(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNumeralSystem)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedGeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ParseInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<i64>> {
        let this = &windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseInt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParseUInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = &windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseUInt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParseDouble(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseDouble)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NumberRounder(&self) -> windows_core::Result<INumberRounder> {
        let this = &windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberRounder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNumberRounder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INumberRounder>,
    {
        let this = &windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNumberRounder)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsZeroSigned(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsZeroSigned)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsZeroSigned)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignificantDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSignificantDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    fn IDecimalFormatterFactory<R, F: FnOnce(&IDecimalFormatterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DecimalFormatter, IDecimalFormatterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DecimalFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INumberFormatter>();
}
unsafe impl windows_core::Interface for DecimalFormatter {
    type Vtable = <INumberFormatter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INumberFormatter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DecimalFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.DecimalFormatter";
}
unsafe impl Send for DecimalFormatter {}
unsafe impl Sync for DecimalFormatter {}
windows_core::imp::define_interface!(ICurrencyFormatter, ICurrencyFormatter_Vtbl, 0x11730ca5_4b00_41b2_b332_73b12a497d54);
impl windows_core::RuntimeType for ICurrencyFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Currency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCurrency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICurrencyFormatter2, ICurrencyFormatter2_Vtbl, 0x072c2f1d_e7ba_4197_920e_247c92f7dea6);
impl windows_core::RuntimeType for ICurrencyFormatter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CurrencyFormatterMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, CurrencyFormatterMode) -> windows_core::HRESULT,
    pub ApplyRoundingForCurrency: unsafe extern "system" fn(*mut core::ffi::c_void, RoundingAlgorithm) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICurrencyFormatterFactory, ICurrencyFormatterFactory_Vtbl, 0x86c7537e_b938_4aa2_84b0_2c33dc5b1450);
impl windows_core::RuntimeType for ICurrencyFormatterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateCurrencyFormatterCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCurrencyFormatterCodeContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDecimalFormatterFactory, IDecimalFormatterFactory_Vtbl, 0x0d018c9a_e393_46b8_b830_7a69c8f89fbb);
impl windows_core::RuntimeType for IDecimalFormatterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecimalFormatterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateDecimalFormatter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIncrementNumberRounder, IIncrementNumberRounder_Vtbl, 0x70a64ff8_66ab_4155_9da1_739e46764543);
impl windows_core::RuntimeType for IIncrementNumberRounder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IIncrementNumberRounder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RoundingAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RoundingAlgorithm) -> windows_core::HRESULT,
    pub SetRoundingAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, RoundingAlgorithm) -> windows_core::HRESULT,
    pub Increment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetIncrement: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumberFormatter, INumberFormatter_Vtbl, 0xa5007c49_7676_4db7_8631_1b6ff265caa9);
impl windows_core::RuntimeType for INumberFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INumberFormatter, windows_core::IUnknown, windows_core::IInspectable);
impl INumberFormatter {
    pub fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for INumberFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatter";
}
pub trait INumberFormatter_Impl: windows_core::IUnknownImpl {
    fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING>;
    fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING>;
    fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING>;
}
impl INumberFormatter_Vtbl {
    pub const fn new<Identity: INumberFormatter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FormatInt<Identity: INumberFormatter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatter_Impl::FormatInt(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FormatUInt<Identity: INumberFormatter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatter_Impl::FormatUInt(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FormatDouble<Identity: INumberFormatter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatter_Impl::FormatDouble(this, value) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberFormatter, OFFSET>(),
            FormatInt: FormatInt::<Identity, OFFSET>,
            FormatUInt: FormatUInt::<Identity, OFFSET>,
            FormatDouble: FormatDouble::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberFormatter as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FormatInt: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumberFormatter2, INumberFormatter2_Vtbl, 0xd4a8c1f0_80d0_4b0d_a89e_882c1e8f8310);
impl windows_core::RuntimeType for INumberFormatter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INumberFormatter2, windows_core::IUnknown, windows_core::IInspectable);
impl INumberFormatter2 {
    pub fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for INumberFormatter2 {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatter2";
}
pub trait INumberFormatter2_Impl: windows_core::IUnknownImpl {
    fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING>;
    fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING>;
    fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING>;
}
impl INumberFormatter2_Vtbl {
    pub const fn new<Identity: INumberFormatter2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FormatInt<Identity: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatter2_Impl::FormatInt(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FormatUInt<Identity: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatter2_Impl::FormatUInt(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FormatDouble<Identity: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatter2_Impl::FormatDouble(this, value) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberFormatter2, OFFSET>(),
            FormatInt: FormatInt::<Identity, OFFSET>,
            FormatUInt: FormatUInt::<Identity, OFFSET>,
            FormatDouble: FormatDouble::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberFormatter2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FormatInt: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumberFormatterOptions, INumberFormatterOptions_Vtbl, 0x80332d21_aee1_4a39_baa2_07ed8c96daf6);
impl windows_core::RuntimeType for INumberFormatterOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INumberFormatterOptions, windows_core::IUnknown, windows_core::IInspectable);
impl INumberFormatterOptions {
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
    pub fn IntegerDigits(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IntegerDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIntegerDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FractionDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFractionDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGrouped)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsGrouped)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(windows_core::Interface::as_raw(this), value).ok() }
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
}
impl windows_core::RuntimeName for INumberFormatterOptions {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatterOptions";
}
pub trait INumberFormatterOptions_Impl: windows_core::IUnknownImpl {
    fn Languages(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>>;
    fn GeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn IntegerDigits(&self) -> windows_core::Result<i32>;
    fn SetIntegerDigits(&self, value: i32) -> windows_core::Result<()>;
    fn FractionDigits(&self) -> windows_core::Result<i32>;
    fn SetFractionDigits(&self, value: i32) -> windows_core::Result<()>;
    fn IsGrouped(&self) -> windows_core::Result<bool>;
    fn SetIsGrouped(&self, value: bool) -> windows_core::Result<()>;
    fn IsDecimalPointAlwaysDisplayed(&self) -> windows_core::Result<bool>;
    fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> windows_core::Result<()>;
    fn NumeralSystem(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetNumeralSystem(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ResolvedGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl INumberFormatterOptions_Vtbl {
    pub const fn new<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Languages<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatterOptions_Impl::Languages(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GeographicRegion<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatterOptions_Impl::GeographicRegion(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IntegerDigits<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatterOptions_Impl::IntegerDigits(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIntegerDigits<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INumberFormatterOptions_Impl::SetIntegerDigits(this, value).into()
            }
        }
        unsafe extern "system" fn FractionDigits<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatterOptions_Impl::FractionDigits(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFractionDigits<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INumberFormatterOptions_Impl::SetFractionDigits(this, value).into()
            }
        }
        unsafe extern "system" fn IsGrouped<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatterOptions_Impl::IsGrouped(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsGrouped<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INumberFormatterOptions_Impl::SetIsGrouped(this, value).into()
            }
        }
        unsafe extern "system" fn IsDecimalPointAlwaysDisplayed<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatterOptions_Impl::IsDecimalPointAlwaysDisplayed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsDecimalPointAlwaysDisplayed<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INumberFormatterOptions_Impl::SetIsDecimalPointAlwaysDisplayed(this, value).into()
            }
        }
        unsafe extern "system" fn NumeralSystem<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatterOptions_Impl::NumeralSystem(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNumeralSystem<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INumberFormatterOptions_Impl::SetNumeralSystem(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ResolvedLanguage<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatterOptions_Impl::ResolvedLanguage(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResolvedGeographicRegion<Identity: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatterOptions_Impl::ResolvedGeographicRegion(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberFormatterOptions, OFFSET>(),
            Languages: Languages::<Identity, OFFSET>,
            GeographicRegion: GeographicRegion::<Identity, OFFSET>,
            IntegerDigits: IntegerDigits::<Identity, OFFSET>,
            SetIntegerDigits: SetIntegerDigits::<Identity, OFFSET>,
            FractionDigits: FractionDigits::<Identity, OFFSET>,
            SetFractionDigits: SetFractionDigits::<Identity, OFFSET>,
            IsGrouped: IsGrouped::<Identity, OFFSET>,
            SetIsGrouped: SetIsGrouped::<Identity, OFFSET>,
            IsDecimalPointAlwaysDisplayed: IsDecimalPointAlwaysDisplayed::<Identity, OFFSET>,
            SetIsDecimalPointAlwaysDisplayed: SetIsDecimalPointAlwaysDisplayed::<Identity, OFFSET>,
            NumeralSystem: NumeralSystem::<Identity, OFFSET>,
            SetNumeralSystem: SetNumeralSystem::<Identity, OFFSET>,
            ResolvedLanguage: ResolvedLanguage::<Identity, OFFSET>,
            ResolvedGeographicRegion: ResolvedGeographicRegion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberFormatterOptions as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatterOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GeographicRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IntegerDigits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIntegerDigits: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FractionDigits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFractionDigits: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsGrouped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsGrouped: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsDecimalPointAlwaysDisplayed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDecimalPointAlwaysDisplayed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResolvedGeographicRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumberParser, INumberParser_Vtbl, 0xe6659412_4a13_4a53_83a1_392fbe4cff9f);
impl windows_core::RuntimeType for INumberParser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INumberParser, windows_core::IUnknown, windows_core::IInspectable);
impl INumberParser {
    pub fn ParseInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<i64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseInt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParseUInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseUInt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParseDouble(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseDouble)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for INumberParser {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberParser";
}
pub trait INumberParser_Impl: windows_core::IUnknownImpl {
    fn ParseInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<i64>>;
    fn ParseUInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<u64>>;
    fn ParseDouble(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<f64>>;
}
impl INumberParser_Vtbl {
    pub const fn new<Identity: INumberParser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ParseInt<Identity: INumberParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberParser_Impl::ParseInt(this, core::mem::transmute(&text)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ParseUInt<Identity: INumberParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberParser_Impl::ParseUInt(this, core::mem::transmute(&text)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ParseDouble<Identity: INumberParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberParser_Impl::ParseDouble(this, core::mem::transmute(&text)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberParser, OFFSET>(),
            ParseInt: ParseInt::<Identity, OFFSET>,
            ParseUInt: ParseUInt::<Identity, OFFSET>,
            ParseDouble: ParseDouble::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberParser as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberParser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseInt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParseUInt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParseDouble: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumberRounder, INumberRounder_Vtbl, 0x5473c375_38ed_4631_b80c_ef34fc48b7f5);
impl windows_core::RuntimeType for INumberRounder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INumberRounder, windows_core::IUnknown, windows_core::IInspectable);
impl INumberRounder {
    pub fn RoundInt32(&self, value: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundInt32)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundUInt32(&self, value: u32) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundUInt32)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundInt64(&self, value: i64) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundInt64)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundUInt64(&self, value: u64) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundUInt64)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundSingle(&self, value: f32) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundSingle)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundDouble(&self, value: f64) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for INumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberRounder";
}
pub trait INumberRounder_Impl: windows_core::IUnknownImpl {
    fn RoundInt32(&self, value: i32) -> windows_core::Result<i32>;
    fn RoundUInt32(&self, value: u32) -> windows_core::Result<u32>;
    fn RoundInt64(&self, value: i64) -> windows_core::Result<i64>;
    fn RoundUInt64(&self, value: u64) -> windows_core::Result<u64>;
    fn RoundSingle(&self, value: f32) -> windows_core::Result<f32>;
    fn RoundDouble(&self, value: f64) -> windows_core::Result<f64>;
}
impl INumberRounder_Vtbl {
    pub const fn new<Identity: INumberRounder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RoundInt32<Identity: INumberRounder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberRounder_Impl::RoundInt32(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RoundUInt32<Identity: INumberRounder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberRounder_Impl::RoundUInt32(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RoundInt64<Identity: INumberRounder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i64, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberRounder_Impl::RoundInt64(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RoundUInt64<Identity: INumberRounder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64, result__: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberRounder_Impl::RoundUInt64(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RoundSingle<Identity: INumberRounder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberRounder_Impl::RoundSingle(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RoundDouble<Identity: INumberRounder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberRounder_Impl::RoundDouble(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberRounder, OFFSET>(),
            RoundInt32: RoundInt32::<Identity, OFFSET>,
            RoundUInt32: RoundUInt32::<Identity, OFFSET>,
            RoundInt64: RoundInt64::<Identity, OFFSET>,
            RoundUInt64: RoundUInt64::<Identity, OFFSET>,
            RoundSingle: RoundSingle::<Identity, OFFSET>,
            RoundDouble: RoundDouble::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberRounder as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberRounder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RoundInt32: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub RoundUInt32: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub RoundInt64: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut i64) -> windows_core::HRESULT,
    pub RoundUInt64: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut u64) -> windows_core::HRESULT,
    pub RoundSingle: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *mut f32) -> windows_core::HRESULT,
    pub RoundDouble: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumberRounderOption, INumberRounderOption_Vtbl, 0x3b088433_646f_4efe_8d48_66eb2e49e736);
impl windows_core::RuntimeType for INumberRounderOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INumberRounderOption, windows_core::IUnknown, windows_core::IInspectable);
impl INumberRounderOption {
    pub fn NumberRounder(&self) -> windows_core::Result<INumberRounder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberRounder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNumberRounder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INumberRounder>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNumberRounder)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeName for INumberRounderOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberRounderOption";
}
pub trait INumberRounderOption_Impl: windows_core::IUnknownImpl {
    fn NumberRounder(&self) -> windows_core::Result<INumberRounder>;
    fn SetNumberRounder(&self, value: windows_core::Ref<INumberRounder>) -> windows_core::Result<()>;
}
impl INumberRounderOption_Vtbl {
    pub const fn new<Identity: INumberRounderOption_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NumberRounder<Identity: INumberRounderOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberRounderOption_Impl::NumberRounder(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNumberRounder<Identity: INumberRounderOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INumberRounderOption_Impl::SetNumberRounder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberRounderOption, OFFSET>(),
            NumberRounder: NumberRounder::<Identity, OFFSET>,
            SetNumberRounder: SetNumberRounder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberRounderOption as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberRounderOption_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NumberRounder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNumberRounder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumeralSystemTranslator, INumeralSystemTranslator_Vtbl, 0x28f5bc2c_8c23_4234_ad2e_fa5a3a426e9b);
impl windows_core::RuntimeType for INumeralSystemTranslator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemTranslator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TranslateNumerals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumeralSystemTranslatorFactory, INumeralSystemTranslatorFactory_Vtbl, 0x9630c8da_36ef_4d88_a85c_6f0d98d620a6);
impl windows_core::RuntimeType for INumeralSystemTranslatorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemTranslatorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPercentFormatterFactory, IPercentFormatterFactory_Vtbl, 0xb7828aef_fed4_4018_a6e2_e09961e03765);
impl windows_core::RuntimeType for IPercentFormatterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPercentFormatterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreatePercentFormatter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPermilleFormatterFactory, IPermilleFormatterFactory_Vtbl, 0x2b37b4ac_e638_4ed5_a998_62f6b06a49ae);
impl windows_core::RuntimeType for IPermilleFormatterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPermilleFormatterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreatePermilleFormatter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISignedZeroOption, ISignedZeroOption_Vtbl, 0xfd1cdd31_0a3c_49c4_a642_96a1564f4f30);
impl windows_core::RuntimeType for ISignedZeroOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISignedZeroOption, windows_core::IUnknown, windows_core::IInspectable);
impl ISignedZeroOption {
    pub fn IsZeroSigned(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsZeroSigned)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsZeroSigned)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeName for ISignedZeroOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ISignedZeroOption";
}
pub trait ISignedZeroOption_Impl: windows_core::IUnknownImpl {
    fn IsZeroSigned(&self) -> windows_core::Result<bool>;
    fn SetIsZeroSigned(&self, value: bool) -> windows_core::Result<()>;
}
impl ISignedZeroOption_Vtbl {
    pub const fn new<Identity: ISignedZeroOption_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsZeroSigned<Identity: ISignedZeroOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISignedZeroOption_Impl::IsZeroSigned(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsZeroSigned<Identity: ISignedZeroOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISignedZeroOption_Impl::SetIsZeroSigned(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISignedZeroOption, OFFSET>(),
            IsZeroSigned: IsZeroSigned::<Identity, OFFSET>,
            SetIsZeroSigned: SetIsZeroSigned::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISignedZeroOption as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignedZeroOption_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsZeroSigned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsZeroSigned: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISignificantDigitsNumberRounder, ISignificantDigitsNumberRounder_Vtbl, 0xf5941bca_6646_4913_8c76_1b191ff94dfd);
impl windows_core::RuntimeType for ISignificantDigitsNumberRounder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignificantDigitsNumberRounder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RoundingAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RoundingAlgorithm) -> windows_core::HRESULT,
    pub SetRoundingAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, RoundingAlgorithm) -> windows_core::HRESULT,
    pub SignificantDigits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSignificantDigits: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISignificantDigitsOption, ISignificantDigitsOption_Vtbl, 0x1d4dfcdd_2d43_4ee8_bbf1_c1b26a711a58);
impl windows_core::RuntimeType for ISignificantDigitsOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISignificantDigitsOption, windows_core::IUnknown, windows_core::IInspectable);
impl ISignificantDigitsOption {
    pub fn SignificantDigits(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignificantDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSignificantDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeName for ISignificantDigitsOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ISignificantDigitsOption";
}
pub trait ISignificantDigitsOption_Impl: windows_core::IUnknownImpl {
    fn SignificantDigits(&self) -> windows_core::Result<i32>;
    fn SetSignificantDigits(&self, value: i32) -> windows_core::Result<()>;
}
impl ISignificantDigitsOption_Vtbl {
    pub const fn new<Identity: ISignificantDigitsOption_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SignificantDigits<Identity: ISignificantDigitsOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISignificantDigitsOption_Impl::SignificantDigits(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignificantDigits<Identity: ISignificantDigitsOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISignificantDigitsOption_Impl::SetSignificantDigits(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISignificantDigitsOption, OFFSET>(),
            SignificantDigits: SignificantDigits::<Identity, OFFSET>,
            SetSignificantDigits: SetSignificantDigits::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISignificantDigitsOption as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignificantDigitsOption_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SignificantDigits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSignificantDigits: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncrementNumberRounder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IncrementNumberRounder, windows_core::IUnknown, windows_core::IInspectable, INumberRounder);
impl IncrementNumberRounder {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IncrementNumberRounder, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RoundingAlgorithm(&self) -> windows_core::Result<RoundingAlgorithm> {
        let this = &windows_core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundingAlgorithm)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRoundingAlgorithm)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Increment(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Increment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncrement(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIncrement)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoundInt32(&self, value: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundInt32)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundUInt32(&self, value: u32) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundUInt32)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundInt64(&self, value: i64) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundInt64)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundUInt64(&self, value: u64) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundUInt64)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundSingle(&self, value: f32) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundSingle)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundDouble(&self, value: f64) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IncrementNumberRounder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INumberRounder>();
}
unsafe impl windows_core::Interface for IncrementNumberRounder {
    type Vtable = <INumberRounder as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INumberRounder as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IncrementNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IncrementNumberRounder";
}
unsafe impl Send for IncrementNumberRounder {}
unsafe impl Sync for IncrementNumberRounder {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumeralSystemTranslator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NumeralSystemTranslator, windows_core::IUnknown, windows_core::IInspectable);
impl NumeralSystemTranslator {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NumeralSystemTranslator, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Languages(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Languages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
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
    pub fn TranslateNumerals(&self, value: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TranslateNumerals)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Create<P0>(languages: P0) -> windows_core::Result<NumeralSystemTranslator>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::INumeralSystemTranslatorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), languages.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn INumeralSystemTranslatorFactory<R, F: FnOnce(&INumeralSystemTranslatorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NumeralSystemTranslator, INumeralSystemTranslatorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NumeralSystemTranslator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INumeralSystemTranslator>();
}
unsafe impl windows_core::Interface for NumeralSystemTranslator {
    type Vtable = <INumeralSystemTranslator as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INumeralSystemTranslator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NumeralSystemTranslator {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.NumeralSystemTranslator";
}
unsafe impl Send for NumeralSystemTranslator {}
unsafe impl Sync for NumeralSystemTranslator {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PercentFormatter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PercentFormatter, windows_core::IUnknown, windows_core::IInspectable, INumberFormatter);
windows_core::imp::required_hierarchy!(PercentFormatter, INumberFormatter2, INumberFormatterOptions, INumberParser, INumberRounderOption, ISignedZeroOption, ISignificantDigitsOption);
impl PercentFormatter {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PercentFormatter, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatInt2(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Languages(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Languages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IntegerDigits(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IntegerDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIntegerDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FractionDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFractionDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGrouped)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsGrouped)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumeralSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetNumeralSystem(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNumeralSystem)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedGeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ParseInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<i64>> {
        let this = &windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseInt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParseUInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = &windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseUInt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParseDouble(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseDouble)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NumberRounder(&self) -> windows_core::Result<INumberRounder> {
        let this = &windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberRounder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNumberRounder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INumberRounder>,
    {
        let this = &windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNumberRounder)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CreatePercentFormatter<P0>(languages: P0, geographicregion: &windows_core::HSTRING) -> windows_core::Result<PercentFormatter>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IPercentFormatterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePercentFormatter)(windows_core::Interface::as_raw(this), languages.param().abi(), core::mem::transmute_copy(geographicregion), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsZeroSigned(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsZeroSigned)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsZeroSigned)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignificantDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSignificantDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    fn IPercentFormatterFactory<R, F: FnOnce(&IPercentFormatterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PercentFormatter, IPercentFormatterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PercentFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INumberFormatter>();
}
unsafe impl windows_core::Interface for PercentFormatter {
    type Vtable = <INumberFormatter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INumberFormatter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PercentFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PercentFormatter";
}
unsafe impl Send for PercentFormatter {}
unsafe impl Sync for PercentFormatter {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PermilleFormatter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PermilleFormatter, windows_core::IUnknown, windows_core::IInspectable, INumberFormatter);
windows_core::imp::required_hierarchy!(PermilleFormatter, INumberFormatter2, INumberFormatterOptions, INumberParser, INumberRounderOption, ISignedZeroOption, ISignificantDigitsOption);
impl PermilleFormatter {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PermilleFormatter, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatInt2(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Languages(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Languages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IntegerDigits(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IntegerDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIntegerDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FractionDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFractionDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGrouped)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsGrouped)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumeralSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetNumeralSystem(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNumeralSystem)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedGeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ParseInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<i64>> {
        let this = &windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseInt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParseUInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = &windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseUInt)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParseDouble(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParseDouble)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NumberRounder(&self) -> windows_core::Result<INumberRounder> {
        let this = &windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberRounder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNumberRounder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INumberRounder>,
    {
        let this = &windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNumberRounder)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CreatePermilleFormatter<P0>(languages: P0, geographicregion: &windows_core::HSTRING) -> windows_core::Result<PermilleFormatter>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IPermilleFormatterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePermilleFormatter)(windows_core::Interface::as_raw(this), languages.param().abi(), core::mem::transmute_copy(geographicregion), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsZeroSigned(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsZeroSigned)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsZeroSigned)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignificantDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSignificantDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    fn IPermilleFormatterFactory<R, F: FnOnce(&IPermilleFormatterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PermilleFormatter, IPermilleFormatterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PermilleFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INumberFormatter>();
}
unsafe impl windows_core::Interface for PermilleFormatter {
    type Vtable = <INumberFormatter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INumberFormatter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PermilleFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PermilleFormatter";
}
unsafe impl Send for PermilleFormatter {}
unsafe impl Sync for PermilleFormatter {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RoundingAlgorithm(pub i32);
impl RoundingAlgorithm {
    pub const None: Self = Self(0i32);
    pub const RoundDown: Self = Self(1i32);
    pub const RoundUp: Self = Self(2i32);
    pub const RoundTowardsZero: Self = Self(3i32);
    pub const RoundAwayFromZero: Self = Self(4i32);
    pub const RoundHalfDown: Self = Self(5i32);
    pub const RoundHalfUp: Self = Self(6i32);
    pub const RoundHalfTowardsZero: Self = Self(7i32);
    pub const RoundHalfAwayFromZero: Self = Self(8i32);
    pub const RoundHalfToEven: Self = Self(9i32);
    pub const RoundHalfToOdd: Self = Self(10i32);
}
impl windows_core::TypeKind for RoundingAlgorithm {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for RoundingAlgorithm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.RoundingAlgorithm;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignificantDigitsNumberRounder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SignificantDigitsNumberRounder, windows_core::IUnknown, windows_core::IInspectable, INumberRounder);
impl SignificantDigitsNumberRounder {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SignificantDigitsNumberRounder, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RoundInt32(&self, value: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundInt32)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundUInt32(&self, value: u32) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundUInt32)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundInt64(&self, value: i64) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundInt64)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundUInt64(&self, value: u64) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundUInt64)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundSingle(&self, value: f32) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundSingle)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundDouble(&self, value: f64) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundDouble)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn RoundingAlgorithm(&self) -> windows_core::Result<RoundingAlgorithm> {
        let this = &windows_core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundingAlgorithm)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRoundingAlgorithm)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignificantDigits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSignificantDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SignificantDigitsNumberRounder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INumberRounder>();
}
unsafe impl windows_core::Interface for SignificantDigitsNumberRounder {
    type Vtable = <INumberRounder as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INumberRounder as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SignificantDigitsNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder";
}
unsafe impl Send for SignificantDigitsNumberRounder {}
unsafe impl Sync for SignificantDigitsNumberRounder {}
