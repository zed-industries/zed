windows_core::imp::define_interface!(IHtmlUtilities, IHtmlUtilities_Vtbl, 0xfec00add_2399_4fac_b5a7_05e9acd7181d);
impl windows_core::RuntimeType for IHtmlUtilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHtmlUtilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConvertToText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
pub struct HtmlUtilities;
impl HtmlUtilities {
    pub fn ConvertToText(html: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        Self::IHtmlUtilities(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConvertToText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(html), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IHtmlUtilities<R, F: FnOnce(&IHtmlUtilities) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HtmlUtilities, IHtmlUtilities> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for HtmlUtilities {
    const NAME: &'static str = "Windows.Data.Html.HtmlUtilities";
}
