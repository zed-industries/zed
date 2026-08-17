#[cfg(feature = "UI_Accessibility")]
pub mod Accessibility;
#[cfg(feature = "UI_ApplicationSettings")]
pub mod ApplicationSettings;
#[cfg(feature = "UI_Composition")]
pub mod Composition;
#[cfg(feature = "UI_Core")]
pub mod Core;
#[cfg(feature = "UI_Input")]
pub mod Input;
#[cfg(feature = "UI_Notifications")]
pub mod Notifications;
#[cfg(feature = "UI_Popups")]
pub mod Popups;
#[cfg(feature = "UI_Shell")]
pub mod Shell;
#[cfg(feature = "UI_StartScreen")]
pub mod StartScreen;
#[cfg(feature = "UI_Text")]
pub mod Text;
#[cfg(feature = "UI_UIAutomation")]
pub mod UIAutomation;
#[cfg(feature = "UI_ViewManagement")]
pub mod ViewManagement;
#[cfg(feature = "UI_WebUI")]
pub mod WebUI;
#[cfg(feature = "UI_WindowManagement")]
pub mod WindowManagement;
windows_core::imp::define_interface!(IColorHelper, IColorHelper_Vtbl, 0x193cfbe7_65c7_4540_ad08_6283ba76879a);
impl windows_core::RuntimeType for IColorHelper {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IColorHelper_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IColorHelperStatics, IColorHelperStatics_Vtbl, 0x8504dbea_fb6a_4144_a6c2_33499c9284f5);
impl windows_core::RuntimeType for IColorHelperStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IColorHelperStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromArgb: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u8, u8, u8, *mut Color) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IColorHelperStatics2, IColorHelperStatics2_Vtbl, 0x24d9af02_6eb0_4b94_855c_fcf0818d9a16);
impl windows_core::RuntimeType for IColorHelperStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IColorHelperStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ToDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, Color, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IColors, IColors_Vtbl, 0x9b8c9326_4ca6_4ce5_8994_9eff65cabdcc);
impl windows_core::RuntimeType for IColors {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IColors_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IColorsStatics, IColorsStatics_Vtbl, 0xcff52e04_cca6_4614_a17e_754910c84a99);
impl windows_core::RuntimeType for IColorsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IColorsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AliceBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub AntiqueWhite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Aqua: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Aquamarine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Azure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Beige: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Bisque: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Black: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub BlanchedAlmond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Blue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub BlueViolet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Brown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub BurlyWood: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub CadetBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Chartreuse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Chocolate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Coral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub CornflowerBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Cornsilk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Crimson: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Cyan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkCyan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkGoldenrod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkGray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkKhaki: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkMagenta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkOliveGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkOrange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkOrchid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkRed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkSalmon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkSeaGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkSlateBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkSlateGray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkTurquoise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DarkViolet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DeepPink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DeepSkyBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DimGray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub DodgerBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Firebrick: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub FloralWhite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub ForestGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Fuchsia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Gainsboro: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub GhostWhite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Gold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Goldenrod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Gray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Green: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub GreenYellow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Honeydew: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub HotPink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub IndianRed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Indigo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Ivory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Khaki: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Lavender: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LavenderBlush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LawnGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LemonChiffon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightCoral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightCyan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightGoldenrodYellow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightGray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightPink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightSalmon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightSeaGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightSkyBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightSlateGray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightSteelBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LightYellow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Lime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub LimeGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Linen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Magenta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Maroon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MediumAquamarine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MediumBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MediumOrchid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MediumPurple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MediumSeaGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MediumSlateBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MediumSpringGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MediumTurquoise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MediumVioletRed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MidnightBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MintCream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub MistyRose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Moccasin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub NavajoWhite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Navy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub OldLace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Olive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub OliveDrab: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Orange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub OrangeRed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Orchid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub PaleGoldenrod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub PaleGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub PaleTurquoise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub PaleVioletRed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub PapayaWhip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub PeachPuff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Peru: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Pink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Plum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub PowderBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Purple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Red: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub RosyBrown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub RoyalBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub SaddleBrown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Salmon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub SandyBrown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub SeaGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub SeaShell: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Sienna: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Silver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub SkyBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub SlateBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub SlateGray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Snow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub SpringGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub SteelBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Tan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Teal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Thistle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Tomato: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Transparent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Turquoise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Violet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Wheat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub White: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub WhiteSmoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub Yellow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
    pub YellowGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Color) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIContentRoot, IUIContentRoot_Vtbl, 0x1dfcbac6_b36b_5cb9_9bc5_2b7a0eddc378);
impl windows_core::RuntimeType for IUIContentRoot {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUIContentRoot_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UIContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIContext, IUIContext_Vtbl, 0xbb5cfacd_5bd8_59d0_a59e_1c17a4d6d243);
impl windows_core::RuntimeType for IUIContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUIContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ColorHelper(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ColorHelper, windows_core::IUnknown, windows_core::IInspectable);
impl ColorHelper {
    pub fn FromArgb(a: u8, r: u8, g: u8, b: u8) -> windows_core::Result<Color> {
        Self::IColorHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromArgb)(windows_core::Interface::as_raw(this), a, r, g, b, &mut result__).map(|| result__)
        })
    }
    pub fn ToDisplayName(color: Color) -> windows_core::Result<windows_core::HSTRING> {
        Self::IColorHelperStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToDisplayName)(windows_core::Interface::as_raw(this), color, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IColorHelperStatics<R, F: FnOnce(&IColorHelperStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ColorHelper, IColorHelperStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IColorHelperStatics2<R, F: FnOnce(&IColorHelperStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ColorHelper, IColorHelperStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ColorHelper {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IColorHelper>();
}
unsafe impl windows_core::Interface for ColorHelper {
    type Vtable = IColorHelper_Vtbl;
    const IID: windows_core::GUID = <IColorHelper as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ColorHelper {
    const NAME: &'static str = "Windows.UI.ColorHelper";
}
unsafe impl Send for ColorHelper {}
unsafe impl Sync for ColorHelper {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Colors(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Colors, windows_core::IUnknown, windows_core::IInspectable);
impl Colors {
    pub fn AliceBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AliceBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn AntiqueWhite() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AntiqueWhite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Aqua() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Aqua)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Aquamarine() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Aquamarine)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Azure() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Azure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Beige() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Beige)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Bisque() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bisque)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Black() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Black)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BlanchedAlmond() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BlanchedAlmond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Blue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Blue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BlueViolet() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BlueViolet)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Brown() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Brown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BurlyWood() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BurlyWood)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CadetBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CadetBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Chartreuse() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Chartreuse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Chocolate() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Chocolate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Coral() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Coral)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CornflowerBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CornflowerBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Cornsilk() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cornsilk)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Crimson() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Crimson)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Cyan() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cyan)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkCyan() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkCyan)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkGoldenrod() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkGoldenrod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkGray() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkGray)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkKhaki() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkKhaki)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkMagenta() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkMagenta)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkOliveGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkOliveGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkOrange() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkOrange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkOrchid() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkOrchid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkRed() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkRed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkSalmon() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkSalmon)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkSeaGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkSeaGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkSlateBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkSlateBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkSlateGray() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkSlateGray)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkTurquoise() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkTurquoise)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DarkViolet() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DarkViolet)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DeepPink() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeepPink)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DeepSkyBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeepSkyBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DimGray() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DimGray)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DodgerBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DodgerBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Firebrick() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Firebrick)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn FloralWhite() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FloralWhite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ForestGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForestGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Fuchsia() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Fuchsia)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Gainsboro() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gainsboro)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GhostWhite() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GhostWhite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Gold() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gold)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Goldenrod() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Goldenrod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Gray() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gray)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Green() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Green)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GreenYellow() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GreenYellow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Honeydew() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Honeydew)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HotPink() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HotPink)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IndianRed() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndianRed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Indigo() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Indigo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Ivory() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ivory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Khaki() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Khaki)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Lavender() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lavender)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LavenderBlush() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LavenderBlush)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LawnGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LawnGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LemonChiffon() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LemonChiffon)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightCoral() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightCoral)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightCyan() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightCyan)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightGoldenrodYellow() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightGoldenrodYellow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightGray() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightGray)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightPink() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightPink)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightSalmon() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightSalmon)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightSeaGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightSeaGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightSkyBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightSkyBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightSlateGray() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightSlateGray)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightSteelBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightSteelBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LightYellow() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LightYellow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Lime() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LimeGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LimeGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Linen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Linen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Magenta() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Magenta)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Maroon() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Maroon)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MediumAquamarine() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediumAquamarine)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MediumBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediumBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MediumOrchid() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediumOrchid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MediumPurple() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediumPurple)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MediumSeaGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediumSeaGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MediumSlateBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediumSlateBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MediumSpringGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediumSpringGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MediumTurquoise() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediumTurquoise)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MediumVioletRed() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediumVioletRed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MidnightBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MidnightBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MintCream() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MintCream)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MistyRose() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MistyRose)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Moccasin() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Moccasin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn NavajoWhite() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavajoWhite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Navy() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Navy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn OldLace() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OldLace)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Olive() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Olive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn OliveDrab() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OliveDrab)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Orange() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn OrangeRed() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrangeRed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Orchid() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orchid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PaleGoldenrod() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaleGoldenrod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PaleGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaleGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PaleTurquoise() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaleTurquoise)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PaleVioletRed() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaleVioletRed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PapayaWhip() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PapayaWhip)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PeachPuff() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PeachPuff)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Peru() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Peru)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Pink() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pink)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Plum() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Plum)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PowderBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowderBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Purple() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Purple)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Red() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Red)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RosyBrown() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RosyBrown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RoyalBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoyalBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SaddleBrown() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaddleBrown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Salmon() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Salmon)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SandyBrown() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SandyBrown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SeaGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SeaGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SeaShell() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SeaShell)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Sienna() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Sienna)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Silver() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Silver)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SkyBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SkyBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SlateBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SlateBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SlateGray() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SlateGray)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Snow() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Snow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SpringGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpringGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SteelBlue() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SteelBlue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Tan() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tan)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Teal() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Teal)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Thistle() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thistle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Tomato() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tomato)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Transparent() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transparent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Turquoise() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Turquoise)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Violet() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Violet)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Wheat() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Wheat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn White() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).White)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn WhiteSmoke() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WhiteSmoke)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Yellow() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Yellow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn YellowGreen() -> windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YellowGreen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorsStatics<R, F: FnOnce(&IColorsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Colors, IColorsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Colors {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IColors>();
}
unsafe impl windows_core::Interface for Colors {
    type Vtable = IColors_Vtbl;
    const IID: windows_core::GUID = <IColors as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Colors {
    const NAME: &'static str = "Windows.UI.Colors";
}
unsafe impl Send for Colors {}
unsafe impl Sync for Colors {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UIContentRoot(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UIContentRoot, windows_core::IUnknown, windows_core::IInspectable);
impl UIContentRoot {
    pub fn UIContext(&self) -> windows_core::Result<UIContext> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UIContentRoot {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUIContentRoot>();
}
unsafe impl windows_core::Interface for UIContentRoot {
    type Vtable = IUIContentRoot_Vtbl;
    const IID: windows_core::GUID = <IUIContentRoot as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UIContentRoot {
    const NAME: &'static str = "Windows.UI.UIContentRoot";
}
unsafe impl Send for UIContentRoot {}
unsafe impl Sync for UIContentRoot {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UIContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UIContext, windows_core::IUnknown, windows_core::IInspectable);
impl UIContext {}
impl windows_core::RuntimeType for UIContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUIContext>();
}
unsafe impl windows_core::Interface for UIContext {
    type Vtable = IUIContext_Vtbl;
    const IID: windows_core::GUID = <IUIContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UIContext {
    const NAME: &'static str = "Windows.UI.UIContext";
}
unsafe impl Send for UIContext {}
unsafe impl Sync for UIContext {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
    pub A: u8,
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
impl windows_core::TypeKind for Color {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Color {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
}
impl Default for Color {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowId {
    pub Value: u64,
}
impl windows_core::TypeKind for WindowId {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WindowId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.WindowId;u8)");
}
impl Default for WindowId {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
