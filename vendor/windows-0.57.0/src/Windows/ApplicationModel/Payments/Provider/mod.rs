windows_core::imp::define_interface!(IPaymentAppCanMakePaymentTriggerDetails, IPaymentAppCanMakePaymentTriggerDetails_Vtbl, 0x0ce201f0_8b93_4eb6_8c46_2e4a6c6a26f6);
impl windows_core::RuntimeType for IPaymentAppCanMakePaymentTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentAppCanMakePaymentTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportCanMakePaymentResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentAppManager, IPaymentAppManager_Vtbl, 0x0e47aa53_8521_4969_a957_df2538a3a98f);
impl windows_core::RuntimeType for IPaymentAppManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentAppManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAsync: usize,
    pub UnregisterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentAppManagerStatics, IPaymentAppManagerStatics_Vtbl, 0xa341ac28_fc89_4406_b4d9_34e7fe79dfb6);
impl windows_core::RuntimeType for IPaymentAppManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentAppManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentTransaction, IPaymentTransaction_Vtbl, 0x62581da0_26a5_4e9b_a6eb_66606cf001d3);
impl windows_core::RuntimeType for IPaymentTransaction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentTransaction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PaymentRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PayerEmail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPayerEmail: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PayerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPayerName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PayerPhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPayerPhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UpdateShippingAddressAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateSelectedShippingOptionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentTransactionAcceptResult, IPaymentTransactionAcceptResult_Vtbl, 0x060e3276_d30c_4817_95a2_df7ae9273b56);
impl windows_core::RuntimeType for IPaymentTransactionAcceptResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentTransactionAcceptResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::PaymentRequestCompletionStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentTransactionStatics, IPaymentTransactionStatics_Vtbl, 0x8d639750_ee0a_4df5_9b1e_1c0f9ec59881);
impl windows_core::RuntimeType for IPaymentTransactionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentTransactionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PaymentAppCanMakePaymentTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentAppCanMakePaymentTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentAppCanMakePaymentTriggerDetails {
    pub fn Request(&self) -> windows_core::Result<super::PaymentRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCanMakePaymentResult<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::PaymentCanMakePaymentResult>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportCanMakePaymentResult)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for PaymentAppCanMakePaymentTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentAppCanMakePaymentTriggerDetails>();
}
unsafe impl windows_core::Interface for PaymentAppCanMakePaymentTriggerDetails {
    type Vtable = IPaymentAppCanMakePaymentTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IPaymentAppCanMakePaymentTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentAppCanMakePaymentTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails";
}
unsafe impl Send for PaymentAppCanMakePaymentTriggerDetails {}
unsafe impl Sync for PaymentAppCanMakePaymentTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PaymentAppManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentAppManager, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentAppManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterAsync<P0>(&self, supportedpaymentmethodids: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterAsync)(windows_core::Interface::as_raw(this), supportedpaymentmethodids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UnregisterAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnregisterAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Current() -> windows_core::Result<PaymentAppManager> {
        Self::IPaymentAppManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentAppManagerStatics<R, F: FnOnce(&IPaymentAppManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentAppManager, IPaymentAppManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentAppManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentAppManager>();
}
unsafe impl windows_core::Interface for PaymentAppManager {
    type Vtable = IPaymentAppManager_Vtbl;
    const IID: windows_core::GUID = <IPaymentAppManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentAppManager";
}
unsafe impl Send for PaymentAppManager {}
unsafe impl Sync for PaymentAppManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PaymentTransaction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentTransaction, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentTransaction {
    pub fn PaymentRequest(&self) -> windows_core::Result<super::PaymentRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaymentRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PayerEmail(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PayerEmail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPayerEmail(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPayerEmail)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PayerName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PayerName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPayerName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPayerName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PayerPhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PayerPhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPayerPhoneNumber(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPayerPhoneNumber)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn UpdateShippingAddressAsync<P0>(&self, shippingaddress: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>
    where
        P0: windows_core::Param<super::PaymentAddress>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateShippingAddressAsync)(windows_core::Interface::as_raw(this), shippingaddress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateSelectedShippingOptionAsync<P0>(&self, selectedshippingoption: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>
    where
        P0: windows_core::Param<super::PaymentShippingOption>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateSelectedShippingOptionAsync)(windows_core::Interface::as_raw(this), selectedshippingoption.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AcceptAsync<P0>(&self, paymenttoken: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransactionAcceptResult>>
    where
        P0: windows_core::Param<super::PaymentToken>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceptAsync)(windows_core::Interface::as_raw(this), paymenttoken.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reject(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Reject)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FromIdAsync(id: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransaction>> {
        Self::IPaymentTransactionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentTransactionStatics<R, F: FnOnce(&IPaymentTransactionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentTransaction, IPaymentTransactionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentTransaction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentTransaction>();
}
unsafe impl windows_core::Interface for PaymentTransaction {
    type Vtable = IPaymentTransaction_Vtbl;
    const IID: windows_core::GUID = <IPaymentTransaction as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentTransaction {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentTransaction";
}
unsafe impl Send for PaymentTransaction {}
unsafe impl Sync for PaymentTransaction {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PaymentTransactionAcceptResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentTransactionAcceptResult, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentTransactionAcceptResult {
    pub fn Status(&self) -> windows_core::Result<super::PaymentRequestCompletionStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PaymentTransactionAcceptResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentTransactionAcceptResult>();
}
unsafe impl windows_core::Interface for PaymentTransactionAcceptResult {
    type Vtable = IPaymentTransactionAcceptResult_Vtbl;
    const IID: windows_core::GUID = <IPaymentTransactionAcceptResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentTransactionAcceptResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult";
}
unsafe impl Send for PaymentTransactionAcceptResult {}
unsafe impl Sync for PaymentTransactionAcceptResult {}
