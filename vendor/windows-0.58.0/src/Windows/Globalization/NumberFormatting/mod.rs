windows_core::imp::define_interface!(ICurrencyFormatter, ICurrencyFormatter_Vtbl, 0x11730ca5_4b00_41b2_b332_73b12a497d54);
impl windows_core::RuntimeType for ICurrencyFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICurrencyFormatter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Currency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub SetCurrency: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetCurrency: usize,
}
windows_core::imp::define_interface!(ICurrencyFormatter2, ICurrencyFormatter2_Vtbl, 0x072c2f1d_e7ba_4197_920e_247c92f7dea6);
impl windows_core::RuntimeType for ICurrencyFormatter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
pub struct ICurrencyFormatterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateCurrencyFormatterCode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCurrencyFormatterCodeContext: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCurrencyFormatterCodeContext: usize,
}
windows_core::imp::define_interface!(IDecimalFormatterFactory, IDecimalFormatterFactory_Vtbl, 0x0d018c9a_e393_46b8_b830_7a69c8f89fbb);
impl windows_core::RuntimeType for IDecimalFormatterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDecimalFormatterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDecimalFormatter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDecimalFormatter: usize,
}
windows_core::imp::define_interface!(IIncrementNumberRounder, IIncrementNumberRounder_Vtbl, 0x70a64ff8_66ab_4155_9da1_739e46764543);
impl windows_core::RuntimeType for IIncrementNumberRounder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIncrementNumberRounder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RoundingAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RoundingAlgorithm) -> windows_core::HRESULT,
    pub SetRoundingAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, RoundingAlgorithm) -> windows_core::HRESULT,
    pub Increment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetIncrement: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumberFormatter, INumberFormatter_Vtbl, 0xa5007c49_7676_4db7_8631_1b6ff265caa9);
impl core::ops::Deref for INumberFormatter {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INumberFormatter, windows_core::IUnknown, windows_core::IInspectable);
impl INumberFormatter {
    pub fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for INumberFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INumberFormatter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FormatInt: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumberFormatter2, INumberFormatter2_Vtbl, 0xd4a8c1f0_80d0_4b0d_a89e_882c1e8f8310);
impl core::ops::Deref for INumberFormatter2 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INumberFormatter2, windows_core::IUnknown, windows_core::IInspectable);
impl INumberFormatter2 {
    pub fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for INumberFormatter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INumberFormatter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FormatInt: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumberFormatterOptions, INumberFormatterOptions_Vtbl, 0x80332d21_aee1_4a39_baa2_07ed8c96daf6);
impl core::ops::Deref for INumberFormatterOptions {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INumberFormatterOptions, windows_core::IUnknown, windows_core::IInspectable);
impl INumberFormatterOptions {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
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
            (windows_core::Interface::vtable(this).GeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).NumeralSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedGeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for INumberFormatterOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INumberFormatterOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub GeographicRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IntegerDigits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIntegerDigits: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FractionDigits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFractionDigits: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsGrouped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsGrouped: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsDecimalPointAlwaysDisplayed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDecimalPointAlwaysDisplayed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ResolvedGeographicRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumberParser, INumberParser_Vtbl, 0xe6659412_4a13_4a53_83a1_392fbe4cff9f);
impl core::ops::Deref for INumberParser {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
impl windows_core::RuntimeType for INumberParser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INumberParser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseInt: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParseUInt: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParseDouble: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumberRounder, INumberRounder_Vtbl, 0x5473c375_38ed_4631_b80c_ef34fc48b7f5);
impl core::ops::Deref for INumberRounder {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
impl windows_core::RuntimeType for INumberRounder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
impl core::ops::Deref for INumberRounderOption {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
impl windows_core::RuntimeType for INumberRounderOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
pub struct INumeralSystemTranslator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub ResolvedLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TranslateNumerals: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INumeralSystemTranslatorFactory, INumeralSystemTranslatorFactory_Vtbl, 0x9630c8da_36ef_4d88_a85c_6f0d98d620a6);
impl windows_core::RuntimeType for INumeralSystemTranslatorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INumeralSystemTranslatorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
}
windows_core::imp::define_interface!(IPercentFormatterFactory, IPercentFormatterFactory_Vtbl, 0xb7828aef_fed4_4018_a6e2_e09961e03765);
impl windows_core::RuntimeType for IPercentFormatterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPercentFormatterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePercentFormatter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePercentFormatter: usize,
}
windows_core::imp::define_interface!(IPermilleFormatterFactory, IPermilleFormatterFactory_Vtbl, 0x2b37b4ac_e638_4ed5_a998_62f6b06a49ae);
impl windows_core::RuntimeType for IPermilleFormatterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPermilleFormatterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePermilleFormatter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePermilleFormatter: usize,
}
windows_core::imp::define_interface!(ISignedZeroOption, ISignedZeroOption_Vtbl, 0xfd1cdd31_0a3c_49c4_a642_96a1564f4f30);
impl core::ops::Deref for ISignedZeroOption {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
impl windows_core::RuntimeType for ISignedZeroOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
pub struct ISignificantDigitsNumberRounder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RoundingAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RoundingAlgorithm) -> windows_core::HRESULT,
    pub SetRoundingAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, RoundingAlgorithm) -> windows_core::HRESULT,
    pub SignificantDigits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSignificantDigits: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISignificantDigitsOption, ISignificantDigitsOption_Vtbl, 0x1d4dfcdd_2d43_4ee8_bbf1_c1b26a711a58);
impl core::ops::Deref for ISignificantDigitsOption {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
impl windows_core::RuntimeType for ISignificantDigitsOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISignificantDigitsOption_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SignificantDigits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSignificantDigits: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CurrencyFormatter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CurrencyFormatter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CurrencyFormatter, INumberFormatter, INumberFormatter2, INumberFormatterOptions, INumberParser, INumberRounderOption, ISignedZeroOption, ISignificantDigitsOption);
impl CurrencyFormatter {
    pub fn Currency(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Currency)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
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
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCurrencyFormatterCodeContext<P0>(currencycode: &windows_core::HSTRING, languages: P0, geographicregion: &windows_core::HSTRING) -> windows_core::Result<CurrencyFormatter>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
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
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatInt2(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
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
            (windows_core::Interface::vtable(this).GeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).NumeralSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedGeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    #[doc(hidden)]
    pub fn ICurrencyFormatterFactory<R, F: FnOnce(&ICurrencyFormatterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CurrencyFormatter, ICurrencyFormatterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CurrencyFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICurrencyFormatter>();
}
unsafe impl windows_core::Interface for CurrencyFormatter {
    type Vtable = ICurrencyFormatter_Vtbl;
    const IID: windows_core::GUID = <ICurrencyFormatter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CurrencyFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.CurrencyFormatter";
}
unsafe impl Send for CurrencyFormatter {}
unsafe impl Sync for CurrencyFormatter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DecimalFormatter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DecimalFormatter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DecimalFormatter, INumberFormatter, INumberFormatter2, INumberFormatterOptions, INumberParser, INumberRounderOption, ISignedZeroOption, ISignificantDigitsOption);
impl DecimalFormatter {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DecimalFormatter, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDecimalFormatter<P0>(languages: P0, geographicregion: &windows_core::HSTRING) -> windows_core::Result<DecimalFormatter>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
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
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatInt2(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
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
            (windows_core::Interface::vtable(this).GeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).NumeralSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedGeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    #[doc(hidden)]
    pub fn IDecimalFormatterFactory<R, F: FnOnce(&IDecimalFormatterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DecimalFormatter, IDecimalFormatterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DecimalFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INumberFormatter>();
}
unsafe impl windows_core::Interface for DecimalFormatter {
    type Vtable = INumberFormatter_Vtbl;
    const IID: windows_core::GUID = <INumberFormatter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DecimalFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.DecimalFormatter";
}
unsafe impl Send for DecimalFormatter {}
unsafe impl Sync for DecimalFormatter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IncrementNumberRounder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IncrementNumberRounder, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IncrementNumberRounder, INumberRounder);
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
    type Vtable = INumberRounder_Vtbl;
    const IID: windows_core::GUID = <INumberRounder as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IncrementNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IncrementNumberRounder";
}
unsafe impl Send for IncrementNumberRounder {}
unsafe impl Sync for IncrementNumberRounder {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
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
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
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
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NumeralSystem(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumeralSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).TranslateNumerals)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0>(languages: P0) -> windows_core::Result<NumeralSystemTranslator>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::INumeralSystemTranslatorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), languages.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn INumeralSystemTranslatorFactory<R, F: FnOnce(&INumeralSystemTranslatorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NumeralSystemTranslator, INumeralSystemTranslatorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NumeralSystemTranslator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INumeralSystemTranslator>();
}
unsafe impl windows_core::Interface for NumeralSystemTranslator {
    type Vtable = INumeralSystemTranslator_Vtbl;
    const IID: windows_core::GUID = <INumeralSystemTranslator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NumeralSystemTranslator {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.NumeralSystemTranslator";
}
unsafe impl Send for NumeralSystemTranslator {}
unsafe impl Sync for NumeralSystemTranslator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PercentFormatter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PercentFormatter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PercentFormatter, INumberFormatter, INumberFormatter2, INumberFormatterOptions, INumberParser, INumberRounderOption, ISignedZeroOption, ISignificantDigitsOption);
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
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatInt2(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
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
            (windows_core::Interface::vtable(this).GeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).NumeralSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedGeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePercentFormatter<P0>(languages: P0, geographicregion: &windows_core::HSTRING) -> windows_core::Result<PercentFormatter>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
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
    #[doc(hidden)]
    pub fn IPercentFormatterFactory<R, F: FnOnce(&IPercentFormatterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PercentFormatter, IPercentFormatterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PercentFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INumberFormatter>();
}
unsafe impl windows_core::Interface for PercentFormatter {
    type Vtable = INumberFormatter_Vtbl;
    const IID: windows_core::GUID = <INumberFormatter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PercentFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PercentFormatter";
}
unsafe impl Send for PercentFormatter {}
unsafe impl Sync for PercentFormatter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PermilleFormatter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PermilleFormatter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PermilleFormatter, INumberFormatter, INumberFormatter2, INumberFormatterOptions, INumberParser, INumberRounderOption, ISignedZeroOption, ISignificantDigitsOption);
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
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatInt2(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUInt)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatDouble)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
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
            (windows_core::Interface::vtable(this).GeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).NumeralSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).ResolvedLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedGeographicRegion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePermilleFormatter<P0>(languages: P0, geographicregion: &windows_core::HSTRING) -> windows_core::Result<PermilleFormatter>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
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
    #[doc(hidden)]
    pub fn IPermilleFormatterFactory<R, F: FnOnce(&IPermilleFormatterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PermilleFormatter, IPermilleFormatterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PermilleFormatter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INumberFormatter>();
}
unsafe impl windows_core::Interface for PermilleFormatter {
    type Vtable = INumberFormatter_Vtbl;
    const IID: windows_core::GUID = <INumberFormatter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PermilleFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PermilleFormatter";
}
unsafe impl Send for PermilleFormatter {}
unsafe impl Sync for PermilleFormatter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SignificantDigitsNumberRounder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SignificantDigitsNumberRounder, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SignificantDigitsNumberRounder, INumberRounder);
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
    type Vtable = INumberRounder_Vtbl;
    const IID: windows_core::GUID = <INumberRounder as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SignificantDigitsNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder";
}
unsafe impl Send for SignificantDigitsNumberRounder {}
unsafe impl Sync for SignificantDigitsNumberRounder {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CurrencyFormatterMode(pub i32);
impl CurrencyFormatterMode {
    pub const UseSymbol: Self = Self(0i32);
    pub const UseCurrencyCode: Self = Self(1i32);
}
impl windows_core::TypeKind for CurrencyFormatterMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CurrencyFormatterMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CurrencyFormatterMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CurrencyFormatterMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.CurrencyFormatterMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
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
impl core::fmt::Debug for RoundingAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RoundingAlgorithm").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RoundingAlgorithm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.RoundingAlgorithm;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
