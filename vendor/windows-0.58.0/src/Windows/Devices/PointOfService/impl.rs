pub trait ICashDrawerEventSourceEventArgs_Impl: Sized {
    fn CashDrawer(&self) -> windows_core::Result<CashDrawer>;
}
impl windows_core::RuntimeName for ICashDrawerEventSourceEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICashDrawerEventSourceEventArgs";
}
impl ICashDrawerEventSourceEventArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICashDrawerEventSourceEventArgs_Vtbl
    where
        Identity: ICashDrawerEventSourceEventArgs_Impl,
    {
        unsafe extern "system" fn CashDrawer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICashDrawerEventSourceEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICashDrawerEventSourceEventArgs_Impl::CashDrawer(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ICashDrawerEventSourceEventArgs, OFFSET>(), CashDrawer: CashDrawer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICashDrawerEventSourceEventArgs as windows_core::Interface>::IID
    }
}
pub trait ICommonClaimedPosPrinterStation_Impl: Sized {
    fn SetCharactersPerLine(&self, value: u32) -> windows_core::Result<()>;
    fn CharactersPerLine(&self) -> windows_core::Result<u32>;
    fn SetLineHeight(&self, value: u32) -> windows_core::Result<()>;
    fn LineHeight(&self) -> windows_core::Result<u32>;
    fn SetLineSpacing(&self, value: u32) -> windows_core::Result<()>;
    fn LineSpacing(&self) -> windows_core::Result<u32>;
    fn LineWidth(&self) -> windows_core::Result<u32>;
    fn SetIsLetterQuality(&self, value: bool) -> windows_core::Result<()>;
    fn IsLetterQuality(&self) -> windows_core::Result<bool>;
    fn IsPaperNearEnd(&self) -> windows_core::Result<bool>;
    fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> windows_core::Result<()>;
    fn ColorCartridge(&self) -> windows_core::Result<PosPrinterColorCartridge>;
    fn IsCoverOpen(&self) -> windows_core::Result<bool>;
    fn IsCartridgeRemoved(&self) -> windows_core::Result<bool>;
    fn IsCartridgeEmpty(&self) -> windows_core::Result<bool>;
    fn IsHeadCleaning(&self) -> windows_core::Result<bool>;
    fn IsPaperEmpty(&self) -> windows_core::Result<bool>;
    fn IsReadyToPrint(&self) -> windows_core::Result<bool>;
    fn ValidateData(&self, data: &windows_core::HSTRING) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for ICommonClaimedPosPrinterStation {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonClaimedPosPrinterStation";
}
impl ICommonClaimedPosPrinterStation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICommonClaimedPosPrinterStation_Vtbl
    where
        Identity: ICommonClaimedPosPrinterStation_Impl,
    {
        unsafe extern "system" fn SetCharactersPerLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommonClaimedPosPrinterStation_Impl::SetCharactersPerLine(this, value).into()
        }
        unsafe extern "system" fn CharactersPerLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::CharactersPerLine(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommonClaimedPosPrinterStation_Impl::SetLineHeight(this, value).into()
        }
        unsafe extern "system" fn LineHeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::LineHeight(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommonClaimedPosPrinterStation_Impl::SetLineSpacing(this, value).into()
        }
        unsafe extern "system" fn LineSpacing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::LineSpacing(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::LineWidth(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLetterQuality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommonClaimedPosPrinterStation_Impl::SetIsLetterQuality(this, value).into()
        }
        unsafe extern "system" fn IsLetterQuality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsLetterQuality(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperNearEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsPaperNearEnd(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorCartridge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PosPrinterColorCartridge) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommonClaimedPosPrinterStation_Impl::SetColorCartridge(this, value).into()
        }
        unsafe extern "system" fn ColorCartridge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PosPrinterColorCartridge) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::ColorCartridge(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCoverOpen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsCoverOpen(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCartridgeRemoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsCartridgeRemoved(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCartridgeEmpty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsCartridgeEmpty(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHeadCleaning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsHeadCleaning(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperEmpty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsPaperEmpty(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadyToPrint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::IsReadyToPrint(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonClaimedPosPrinterStation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonClaimedPosPrinterStation_Impl::ValidateData(this, core::mem::transmute(&data)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICommonClaimedPosPrinterStation, OFFSET>(),
            SetCharactersPerLine: SetCharactersPerLine::<Identity, OFFSET>,
            CharactersPerLine: CharactersPerLine::<Identity, OFFSET>,
            SetLineHeight: SetLineHeight::<Identity, OFFSET>,
            LineHeight: LineHeight::<Identity, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
            LineSpacing: LineSpacing::<Identity, OFFSET>,
            LineWidth: LineWidth::<Identity, OFFSET>,
            SetIsLetterQuality: SetIsLetterQuality::<Identity, OFFSET>,
            IsLetterQuality: IsLetterQuality::<Identity, OFFSET>,
            IsPaperNearEnd: IsPaperNearEnd::<Identity, OFFSET>,
            SetColorCartridge: SetColorCartridge::<Identity, OFFSET>,
            ColorCartridge: ColorCartridge::<Identity, OFFSET>,
            IsCoverOpen: IsCoverOpen::<Identity, OFFSET>,
            IsCartridgeRemoved: IsCartridgeRemoved::<Identity, OFFSET>,
            IsCartridgeEmpty: IsCartridgeEmpty::<Identity, OFFSET>,
            IsHeadCleaning: IsHeadCleaning::<Identity, OFFSET>,
            IsPaperEmpty: IsPaperEmpty::<Identity, OFFSET>,
            IsReadyToPrint: IsReadyToPrint::<Identity, OFFSET>,
            ValidateData: ValidateData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommonClaimedPosPrinterStation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICommonPosPrintStationCapabilities_Impl: Sized {
    fn IsPrinterPresent(&self) -> windows_core::Result<bool>;
    fn IsDualColorSupported(&self) -> windows_core::Result<bool>;
    fn ColorCartridgeCapabilities(&self) -> windows_core::Result<PosPrinterColorCapabilities>;
    fn CartridgeSensors(&self) -> windows_core::Result<PosPrinterCartridgeSensors>;
    fn IsBoldSupported(&self) -> windows_core::Result<bool>;
    fn IsItalicSupported(&self) -> windows_core::Result<bool>;
    fn IsUnderlineSupported(&self) -> windows_core::Result<bool>;
    fn IsDoubleHighPrintSupported(&self) -> windows_core::Result<bool>;
    fn IsDoubleWidePrintSupported(&self) -> windows_core::Result<bool>;
    fn IsDoubleHighDoubleWidePrintSupported(&self) -> windows_core::Result<bool>;
    fn IsPaperEmptySensorSupported(&self) -> windows_core::Result<bool>;
    fn IsPaperNearEndSensorSupported(&self) -> windows_core::Result<bool>;
    fn SupportedCharactersPerLine(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ICommonPosPrintStationCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonPosPrintStationCapabilities";
}
#[cfg(feature = "Foundation_Collections")]
impl ICommonPosPrintStationCapabilities_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICommonPosPrintStationCapabilities_Vtbl
    where
        Identity: ICommonPosPrintStationCapabilities_Impl,
    {
        unsafe extern "system" fn IsPrinterPresent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsPrinterPresent(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDualColorSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsDualColorSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorCartridgeCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PosPrinterColorCapabilities) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::ColorCartridgeCapabilities(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CartridgeSensors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PosPrinterCartridgeSensors) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::CartridgeSensors(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBoldSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsBoldSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsItalicSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsItalicSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUnderlineSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsUnderlineSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleHighPrintSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsDoubleHighPrintSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleWidePrintSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsDoubleWidePrintSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleHighDoubleWidePrintSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsDoubleHighDoubleWidePrintSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperEmptySensorSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsPaperEmptySensorSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPaperNearEndSensorSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::IsPaperNearEndSensorSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCharactersPerLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICommonPosPrintStationCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonPosPrintStationCapabilities_Impl::SupportedCharactersPerLine(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICommonPosPrintStationCapabilities, OFFSET>(),
            IsPrinterPresent: IsPrinterPresent::<Identity, OFFSET>,
            IsDualColorSupported: IsDualColorSupported::<Identity, OFFSET>,
            ColorCartridgeCapabilities: ColorCartridgeCapabilities::<Identity, OFFSET>,
            CartridgeSensors: CartridgeSensors::<Identity, OFFSET>,
            IsBoldSupported: IsBoldSupported::<Identity, OFFSET>,
            IsItalicSupported: IsItalicSupported::<Identity, OFFSET>,
            IsUnderlineSupported: IsUnderlineSupported::<Identity, OFFSET>,
            IsDoubleHighPrintSupported: IsDoubleHighPrintSupported::<Identity, OFFSET>,
            IsDoubleWidePrintSupported: IsDoubleWidePrintSupported::<Identity, OFFSET>,
            IsDoubleHighDoubleWidePrintSupported: IsDoubleHighDoubleWidePrintSupported::<Identity, OFFSET>,
            IsPaperEmptySensorSupported: IsPaperEmptySensorSupported::<Identity, OFFSET>,
            IsPaperNearEndSensorSupported: IsPaperNearEndSensorSupported::<Identity, OFFSET>,
            SupportedCharactersPerLine: SupportedCharactersPerLine::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommonPosPrintStationCapabilities as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICommonReceiptSlipCapabilities_Impl: Sized + ICommonPosPrintStationCapabilities_Impl {
    fn IsBarcodeSupported(&self) -> windows_core::Result<bool>;
    fn IsBitmapSupported(&self) -> windows_core::Result<bool>;
    fn IsLeft90RotationSupported(&self) -> windows_core::Result<bool>;
    fn IsRight90RotationSupported(&self) -> windows_core::Result<bool>;
    fn Is180RotationSupported(&self) -> windows_core::Result<bool>;
    fn IsPrintAreaSupported(&self) -> windows_core::Result<bool>;
    fn RuledLineCapabilities(&self) -> windows_core::Result<PosPrinterRuledLineCapabilities>;
    fn SupportedBarcodeRotations(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
    fn SupportedBitmapRotations(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for ICommonReceiptSlipCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ICommonReceiptSlipCapabilities";
}
#[cfg(feature = "Foundation_Collections")]
impl ICommonReceiptSlipCapabilities_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICommonReceiptSlipCapabilities_Vtbl
    where
        Identity: ICommonReceiptSlipCapabilities_Impl,
    {
        unsafe extern "system" fn IsBarcodeSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonReceiptSlipCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::IsBarcodeSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBitmapSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonReceiptSlipCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::IsBitmapSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLeft90RotationSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonReceiptSlipCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::IsLeft90RotationSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRight90RotationSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonReceiptSlipCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::IsRight90RotationSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is180RotationSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonReceiptSlipCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::Is180RotationSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPrintAreaSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICommonReceiptSlipCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::IsPrintAreaSupported(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RuledLineCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PosPrinterRuledLineCapabilities) -> windows_core::HRESULT
        where
            Identity: ICommonReceiptSlipCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::RuledLineCapabilities(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedBarcodeRotations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICommonReceiptSlipCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::SupportedBarcodeRotations(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedBitmapRotations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICommonReceiptSlipCapabilities_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommonReceiptSlipCapabilities_Impl::SupportedBitmapRotations(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICommonReceiptSlipCapabilities, OFFSET>(),
            IsBarcodeSupported: IsBarcodeSupported::<Identity, OFFSET>,
            IsBitmapSupported: IsBitmapSupported::<Identity, OFFSET>,
            IsLeft90RotationSupported: IsLeft90RotationSupported::<Identity, OFFSET>,
            IsRight90RotationSupported: IsRight90RotationSupported::<Identity, OFFSET>,
            Is180RotationSupported: Is180RotationSupported::<Identity, OFFSET>,
            IsPrintAreaSupported: IsPrintAreaSupported::<Identity, OFFSET>,
            RuledLineCapabilities: RuledLineCapabilities::<Identity, OFFSET>,
            SupportedBarcodeRotations: SupportedBarcodeRotations::<Identity, OFFSET>,
            SupportedBitmapRotations: SupportedBitmapRotations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommonReceiptSlipCapabilities as windows_core::Interface>::IID
    }
}
pub trait IPosPrinterJob_Impl: Sized {
    fn Print(&self, data: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn PrintLine(&self, data: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn PrintNewline(&self) -> windows_core::Result<()>;
    fn ExecuteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
impl windows_core::RuntimeName for IPosPrinterJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IPosPrinterJob";
}
impl IPosPrinterJob_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPosPrinterJob_Vtbl
    where
        Identity: IPosPrinterJob_Impl,
    {
        unsafe extern "system" fn Print<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IPosPrinterJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPosPrinterJob_Impl::Print(this, core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn PrintLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IPosPrinterJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPosPrinterJob_Impl::PrintLine(this, core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn PrintNewline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPosPrinterJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPosPrinterJob_Impl::PrintNewline(this).into()
        }
        unsafe extern "system" fn ExecuteAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPosPrinterJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPosPrinterJob_Impl::ExecuteAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPosPrinterJob, OFFSET>(),
            Print: Print::<Identity, OFFSET>,
            PrintLine: PrintLine::<Identity, OFFSET>,
            PrintNewline: PrintNewline::<Identity, OFFSET>,
            ExecuteAsync: ExecuteAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPosPrinterJob as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Graphics_Imaging")]
pub trait IReceiptOrSlipJob_Impl: Sized + IPosPrinterJob_Impl {
    fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> windows_core::Result<()>;
    fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> windows_core::Result<()>;
    fn SetPrintArea(&self, value: &super::super::Foundation::Rect) -> windows_core::Result<()>;
    fn SetBitmap(&self, bitmapnumber: u32, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> windows_core::Result<()>;
    fn SetBitmapCustomWidthStandardAlign(&self, bitmapnumber: u32, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> windows_core::Result<()>;
    fn SetCustomAlignedBitmap(&self, bitmapnumber: u32, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> windows_core::Result<()>;
    fn SetBitmapCustomWidthCustomAlign(&self, bitmapnumber: u32, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> windows_core::Result<()>;
    fn PrintSavedBitmap(&self, bitmapnumber: u32) -> windows_core::Result<()>;
    fn DrawRuledLine(&self, positionlist: &windows_core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> windows_core::Result<()>;
    fn PrintBarcode(&self, data: &windows_core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> windows_core::Result<()>;
    fn PrintBarcodeCustomAlign(&self, data: &windows_core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> windows_core::Result<()>;
    fn PrintBitmap(&self, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> windows_core::Result<()>;
    fn PrintBitmapCustomWidthStandardAlign(&self, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> windows_core::Result<()>;
    fn PrintCustomAlignedBitmap(&self, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> windows_core::Result<()>;
    fn PrintBitmapCustomWidthCustomAlign(&self, bitmap: Option<&super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Graphics_Imaging")]
impl windows_core::RuntimeName for IReceiptOrSlipJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.IReceiptOrSlipJob";
}
#[cfg(feature = "Graphics_Imaging")]
impl IReceiptOrSlipJob_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IReceiptOrSlipJob_Vtbl
    where
        Identity: IReceiptOrSlipJob_Impl,
    {
        unsafe extern "system" fn SetBarcodeRotation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PosPrinterRotation) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetBarcodeRotation(this, value).into()
        }
        unsafe extern "system" fn SetPrintRotation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PosPrinterRotation, includebitmaps: bool) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetPrintRotation(this, value, includebitmaps).into()
        }
        unsafe extern "system" fn SetPrintArea<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::Rect) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetPrintArea(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SetBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapnumber: u32, bitmap: *mut core::ffi::c_void, alignment: PosPrinterAlignment) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetBitmap(this, bitmapnumber, windows_core::from_raw_borrowed(&bitmap), alignment).into()
        }
        unsafe extern "system" fn SetBitmapCustomWidthStandardAlign<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapnumber: u32, bitmap: *mut core::ffi::c_void, alignment: PosPrinterAlignment, width: u32) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetBitmapCustomWidthStandardAlign(this, bitmapnumber, windows_core::from_raw_borrowed(&bitmap), alignment, width).into()
        }
        unsafe extern "system" fn SetCustomAlignedBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapnumber: u32, bitmap: *mut core::ffi::c_void, alignmentdistance: u32) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetCustomAlignedBitmap(this, bitmapnumber, windows_core::from_raw_borrowed(&bitmap), alignmentdistance).into()
        }
        unsafe extern "system" fn SetBitmapCustomWidthCustomAlign<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapnumber: u32, bitmap: *mut core::ffi::c_void, alignmentdistance: u32, width: u32) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::SetBitmapCustomWidthCustomAlign(this, bitmapnumber, windows_core::from_raw_borrowed(&bitmap), alignmentdistance, width).into()
        }
        unsafe extern "system" fn PrintSavedBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapnumber: u32) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintSavedBitmap(this, bitmapnumber).into()
        }
        unsafe extern "system" fn DrawRuledLine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, positionlist: core::mem::MaybeUninit<windows_core::HSTRING>, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::DrawRuledLine(this, core::mem::transmute(&positionlist), linedirection, linewidth, linestyle, linecolor).into()
        }
        unsafe extern "system" fn PrintBarcode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintBarcode(this, core::mem::transmute(&data), symbology, height, width, textposition, alignment).into()
        }
        unsafe extern "system" fn PrintBarcodeCustomAlign<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintBarcodeCustomAlign(this, core::mem::transmute(&data), symbology, height, width, textposition, alignmentdistance).into()
        }
        unsafe extern "system" fn PrintBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, alignment: PosPrinterAlignment) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintBitmap(this, windows_core::from_raw_borrowed(&bitmap), alignment).into()
        }
        unsafe extern "system" fn PrintBitmapCustomWidthStandardAlign<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, alignment: PosPrinterAlignment, width: u32) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintBitmapCustomWidthStandardAlign(this, windows_core::from_raw_borrowed(&bitmap), alignment, width).into()
        }
        unsafe extern "system" fn PrintCustomAlignedBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, alignmentdistance: u32) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintCustomAlignedBitmap(this, windows_core::from_raw_borrowed(&bitmap), alignmentdistance).into()
        }
        unsafe extern "system" fn PrintBitmapCustomWidthCustomAlign<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, alignmentdistance: u32, width: u32) -> windows_core::HRESULT
        where
            Identity: IReceiptOrSlipJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReceiptOrSlipJob_Impl::PrintBitmapCustomWidthCustomAlign(this, windows_core::from_raw_borrowed(&bitmap), alignmentdistance, width).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IReceiptOrSlipJob, OFFSET>(),
            SetBarcodeRotation: SetBarcodeRotation::<Identity, OFFSET>,
            SetPrintRotation: SetPrintRotation::<Identity, OFFSET>,
            SetPrintArea: SetPrintArea::<Identity, OFFSET>,
            SetBitmap: SetBitmap::<Identity, OFFSET>,
            SetBitmapCustomWidthStandardAlign: SetBitmapCustomWidthStandardAlign::<Identity, OFFSET>,
            SetCustomAlignedBitmap: SetCustomAlignedBitmap::<Identity, OFFSET>,
            SetBitmapCustomWidthCustomAlign: SetBitmapCustomWidthCustomAlign::<Identity, OFFSET>,
            PrintSavedBitmap: PrintSavedBitmap::<Identity, OFFSET>,
            DrawRuledLine: DrawRuledLine::<Identity, OFFSET>,
            PrintBarcode: PrintBarcode::<Identity, OFFSET>,
            PrintBarcodeCustomAlign: PrintBarcodeCustomAlign::<Identity, OFFSET>,
            PrintBitmap: PrintBitmap::<Identity, OFFSET>,
            PrintBitmapCustomWidthStandardAlign: PrintBitmapCustomWidthStandardAlign::<Identity, OFFSET>,
            PrintCustomAlignedBitmap: PrintCustomAlignedBitmap::<Identity, OFFSET>,
            PrintBitmapCustomWidthCustomAlign: PrintBitmapCustomWidthCustomAlign::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReceiptOrSlipJob as windows_core::Interface>::IID
    }
}
