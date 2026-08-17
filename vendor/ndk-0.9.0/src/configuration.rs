//! Bindings for [`AConfiguration`]
//!
//! See also the [NDK docs](https://developer.android.com/ndk/reference/group/configuration) for
//! [`AConfiguration`], as well as the [docs for providing
//! resources](https://developer.android.com/guide/topics/resources/providing-resources.html),
//! which explain many of the configuration values.  The [`android.content.res.Configuration`
//! javadoc](https://developer.android.com/reference/android/content/res/Configuration.html) may
//! also have useful information.
//!
//! [`AConfiguration`]: https://developer.android.com/ndk/reference/group/configuration#aconfiguration

use crate::asset::AssetManager;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::fmt;
use std::ptr::NonNull;

/// A native [`AConfiguration *`]
///
/// [`Configuration`] is an opaque type used to get and set various subsystem configurations.
///
/// [`AConfiguration *`]: https://developer.android.com/ndk/reference/group/configuration#aconfiguration
pub struct Configuration {
    ptr: NonNull<ffi::AConfiguration>,
}

unsafe impl Send for Configuration {}
unsafe impl Sync for Configuration {}

impl Drop for Configuration {
    fn drop(&mut self) {
        unsafe { ffi::AConfiguration_delete(self.ptr.as_ptr()) }
    }
}

impl Clone for Configuration {
    fn clone(&self) -> Self {
        let mut new = Self::new();
        new.copy(self);
        new
    }
}

impl PartialEq for Configuration {
    fn eq(&self, other: &Self) -> bool {
        self.diff(other).0 == 0
    }
}
impl Eq for Configuration {}

impl fmt::Debug for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Configuration")
            .field("mcc", &self.mcc())
            .field("mnc", &self.mnc())
            .field("lang", &self.language())
            .field("country", &self.country())
            .field("orientation", &self.orientation())
            .field("touchscreen", &self.touchscreen())
            .field("density", &self.density())
            .field("keyboard", &self.keyboard())
            .field("navigation", &self.navigation())
            .field("keys_hidden", &self.keys_hidden())
            .field("nav_hidden", &self.nav_hidden())
            .field("sdk_version", &self.sdk_version())
            .field("screen_size", &self.screen_size())
            .field("screen_long", &self.screen_long())
            .field("ui_mode_type", &self.ui_mode_type())
            .field("ui_mode_night", &self.ui_mode_night())
            .finish()
    }
}

impl Configuration {
    /// Construct a `Configuration` from a pointer.
    ///
    /// # Safety
    /// By calling this function, you assert that it is a valid pointer to a native
    /// `AConfiguration`, and give ownership of it to the `Configuration` instance.
    pub unsafe fn from_ptr(ptr: NonNull<ffi::AConfiguration>) -> Self {
        Self { ptr }
    }

    /// Create a new `Configuration`, with the same contents as the `AConfiguration` referenced by
    /// the pointer.
    ///
    /// This is useful if you have a pointer, but not ownership of it.
    ///
    /// # Safety
    /// By calling this function, you assert that it is a valid pointer to a native
    /// `AConfiguration`.
    pub unsafe fn clone_from_ptr(ptr: NonNull<ffi::AConfiguration>) -> Self {
        let conf = Self::new();
        ffi::AConfiguration_copy(conf.ptr.as_ptr(), ptr.as_ptr());
        conf
    }

    /// The pointer to the native `AConfiguration`.  Keep in mind that the `Configuration` object
    /// still has ownership, and will free it when dropped.
    pub fn ptr(&self) -> NonNull<ffi::AConfiguration> {
        self.ptr
    }

    pub fn from_asset_manager(am: &AssetManager) -> Self {
        let config = Self::new();
        unsafe {
            ffi::AConfiguration_fromAssetManager(config.ptr().as_mut(), am.ptr().as_mut());
        }
        config
    }

    /// Create a new `Configuration`, with none of the values set.
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: NonNull::new(ffi::AConfiguration_new()).unwrap(),
            }
        }
    }

    /// `dest.copy(&src)` copies the contents of `src` to `dest`
    pub fn copy(&mut self, other: &Self) {
        unsafe { ffi::AConfiguration_copy(self.ptr.as_ptr(), other.ptr.as_ptr()) }
    }

    /// Information about what fields differ between the two configurations
    pub fn diff(&self, other: &Self) -> DiffResult {
        unsafe {
            DiffResult(ffi::AConfiguration_diff(self.ptr.as_ptr(), other.ptr.as_ptr()) as u32)
        }
    }

    /// Returns false if anything in `self` conflicts with `requested`
    pub fn matches(&self, requested: &Self) -> bool {
        unsafe { ffi::AConfiguration_match(self.ptr.as_ptr(), requested.ptr.as_ptr()) != 0 }
    }

    /// Returns the country code, as a [`String`] of two characters, if set
    pub fn country(&self) -> Option<String> {
        let mut chars = [0u8; 2];
        unsafe {
            ffi::AConfiguration_getCountry(self.ptr.as_ptr(), chars.as_mut_ptr().cast());
        }
        if chars[0] == 0 {
            None
        } else {
            Some(std::str::from_utf8(chars.as_slice()).unwrap().to_owned())
        }
    }

    /// Returns the screen density in dpi.
    ///
    /// On some devices it can return values outside of the density enum.
    pub fn density(&self) -> Option<u32> {
        let density = unsafe { ffi::AConfiguration_getDensity(self.ptr.as_ptr()) as u32 };
        match density {
            ffi::ACONFIGURATION_DENSITY_DEFAULT => Some(160),
            ffi::ACONFIGURATION_DENSITY_ANY => None,
            ffi::ACONFIGURATION_DENSITY_NONE => None,
            density => Some(density),
        }
    }

    /// Returns the keyboard type.
    pub fn keyboard(&self) -> Keyboard {
        unsafe { ffi::AConfiguration_getKeyboard(self.ptr.as_ptr()).into() }
    }

    /// Returns keyboard visibility/availability.
    pub fn keys_hidden(&self) -> KeysHidden {
        unsafe { ffi::AConfiguration_getKeysHidden(self.ptr.as_ptr()).into() }
    }

    /// Returns the language, as a [`String`] of two characters, if set
    pub fn language(&self) -> Option<String> {
        let mut chars = [0u8; 2];
        unsafe {
            ffi::AConfiguration_getLanguage(self.ptr.as_ptr(), chars.as_mut_ptr().cast());
        }
        if chars[0] == 0 {
            None
        } else {
            Some(std::str::from_utf8(chars.as_slice()).unwrap().to_owned())
        }
    }

    /// Returns the layout direction
    pub fn layout_direction(&self) -> LayoutDir {
        unsafe { ffi::AConfiguration_getLayoutDirection(self.ptr.as_ptr()).into() }
    }

    /// Returns the mobile country code.
    pub fn mcc(&self) -> i32 {
        unsafe { ffi::AConfiguration_getMcc(self.ptr.as_ptr()) }
    }

    /// Returns the mobile network code, if one is defined
    pub fn mnc(&self) -> Option<i32> {
        unsafe {
            match ffi::AConfiguration_getMnc(self.ptr.as_ptr()) {
                0 => None,
                x if x == ffi::ACONFIGURATION_MNC_ZERO as i32 => Some(0),
                x => Some(x),
            }
        }
    }

    pub fn nav_hidden(&self) -> NavHidden {
        unsafe { ffi::AConfiguration_getNavHidden(self.ptr.as_ptr()).into() }
    }

    pub fn navigation(&self) -> Navigation {
        unsafe { ffi::AConfiguration_getNavigation(self.ptr.as_ptr()).into() }
    }

    pub fn orientation(&self) -> Orientation {
        unsafe { ffi::AConfiguration_getOrientation(self.ptr.as_ptr()).into() }
    }

    pub fn screen_height_dp(&self) -> Option<i32> {
        unsafe {
            let height = ffi::AConfiguration_getScreenHeightDp(self.ptr.as_ptr());
            if height == ffi::ACONFIGURATION_SCREEN_HEIGHT_DP_ANY as i32 {
                None
            } else {
                Some(height)
            }
        }
    }

    pub fn screen_width_dp(&self) -> Option<i32> {
        unsafe {
            let width = ffi::AConfiguration_getScreenWidthDp(self.ptr.as_ptr());
            if width == ffi::ACONFIGURATION_SCREEN_WIDTH_DP_ANY as i32 {
                None
            } else {
                Some(width)
            }
        }
    }

    pub fn screen_long(&self) -> ScreenLong {
        unsafe { ffi::AConfiguration_getScreenLong(self.ptr.as_ptr()).into() }
    }

    #[cfg(feature = "api-level-30")]
    pub fn screen_round(&self) -> ScreenRound {
        unsafe { ffi::AConfiguration_getScreenRound(self.ptr.as_ptr()).into() }
    }

    pub fn screen_size(&self) -> ScreenSize {
        unsafe { ffi::AConfiguration_getScreenSize(self.ptr.as_ptr()).into() }
    }

    pub fn sdk_version(&self) -> i32 {
        unsafe { ffi::AConfiguration_getSdkVersion(self.ptr.as_ptr()) }
    }

    pub fn smallest_screen_width_dp(&self) -> Option<i32> {
        unsafe {
            let width = ffi::AConfiguration_getSmallestScreenWidthDp(self.ptr.as_ptr());
            if width == ffi::ACONFIGURATION_SMALLEST_SCREEN_WIDTH_DP_ANY as i32 {
                None
            } else {
                Some(width)
            }
        }
    }

    pub fn touchscreen(&self) -> Touchscreen {
        unsafe { ffi::AConfiguration_getTouchscreen(self.ptr.as_ptr()).into() }
    }

    pub fn ui_mode_night(&self) -> UiModeNight {
        unsafe { ffi::AConfiguration_getUiModeNight(self.ptr.as_ptr()).into() }
    }

    pub fn ui_mode_type(&self) -> UiModeType {
        unsafe { ffi::AConfiguration_getUiModeType(self.ptr.as_ptr()).into() }
    }
}

/// A bitfield representing the differences between two [`Configuration`]s
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DiffResult(pub u32);

impl DiffResult {
    pub fn mcc(self) -> bool {
        self.0 & ffi::ACONFIGURATION_MCC != 0
    }
    pub fn mnc(self) -> bool {
        self.0 & ffi::ACONFIGURATION_MNC != 0
    }
    pub fn locale(self) -> bool {
        self.0 & ffi::ACONFIGURATION_LOCALE != 0
    }
    pub fn touchscreen(self) -> bool {
        self.0 & ffi::ACONFIGURATION_TOUCHSCREEN != 0
    }
    pub fn keyboard(self) -> bool {
        self.0 & ffi::ACONFIGURATION_KEYBOARD != 0
    }
    pub fn keyboard_hidden(self) -> bool {
        self.0 & ffi::ACONFIGURATION_KEYBOARD_HIDDEN != 0
    }
    pub fn navigation(self) -> bool {
        self.0 & ffi::ACONFIGURATION_NAVIGATION != 0
    }
    pub fn orientation(self) -> bool {
        self.0 & ffi::ACONFIGURATION_ORIENTATION != 0
    }
    pub fn density(self) -> bool {
        self.0 & ffi::ACONFIGURATION_DENSITY != 0
    }
    pub fn screen_size(self) -> bool {
        self.0 & ffi::ACONFIGURATION_SCREEN_SIZE != 0
    }
    pub fn version(self) -> bool {
        self.0 & ffi::ACONFIGURATION_VERSION != 0
    }
    pub fn screen_layout(self) -> bool {
        self.0 & ffi::ACONFIGURATION_SCREEN_LAYOUT != 0
    }
    pub fn ui_mode(self) -> bool {
        self.0 & ffi::ACONFIGURATION_UI_MODE != 0
    }
    pub fn smallest_screen_size(self) -> bool {
        self.0 & ffi::ACONFIGURATION_SMALLEST_SCREEN_SIZE != 0
    }
    pub fn layout_dir(self) -> bool {
        self.0 & ffi::ACONFIGURATION_LAYOUTDIR != 0
    }
    pub fn screen_round(self) -> bool {
        self.0 & ffi::ACONFIGURATION_SCREEN_ROUND != 0
    }
    pub fn color_mode(self) -> bool {
        self.0 & ffi::ACONFIGURATION_COLOR_MODE != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum Orientation {
    Any = ffi::ACONFIGURATION_ORIENTATION_ANY as i32,
    Port = ffi::ACONFIGURATION_ORIENTATION_PORT as i32,
    Land = ffi::ACONFIGURATION_ORIENTATION_LAND as i32,
    Square = ffi::ACONFIGURATION_ORIENTATION_SQUARE as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum Touchscreen {
    Any = ffi::ACONFIGURATION_TOUCHSCREEN_ANY as i32,
    NoTouch = ffi::ACONFIGURATION_TOUCHSCREEN_NOTOUCH as i32,
    Stylus = ffi::ACONFIGURATION_TOUCHSCREEN_STYLUS as i32,
    Finger = ffi::ACONFIGURATION_TOUCHSCREEN_FINGER as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum Density {
    Default = ffi::ACONFIGURATION_DENSITY_DEFAULT as i32,
    Low = ffi::ACONFIGURATION_DENSITY_LOW as i32,
    Medium = ffi::ACONFIGURATION_DENSITY_MEDIUM as i32,
    TV = ffi::ACONFIGURATION_DENSITY_TV as i32,
    High = ffi::ACONFIGURATION_DENSITY_HIGH as i32,
    XHigh = ffi::ACONFIGURATION_DENSITY_XHIGH as i32,
    XXHigh = ffi::ACONFIGURATION_DENSITY_XXHIGH as i32,
    XXXHigh = ffi::ACONFIGURATION_DENSITY_XXXHIGH as i32,
    Any = ffi::ACONFIGURATION_DENSITY_ANY as i32,
    None = ffi::ACONFIGURATION_DENSITY_NONE as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

impl Density {
    /// The DPI associated with the density class.
    /// See [the Android screen density
    /// docs](https://developer.android.com/training/multiscreen/screendensities#TaskProvideAltBmp)
    ///
    /// There are some [`Density`] values that have no associated DPI; these values return [`None`].
    pub fn dpi(self) -> Option<u32> {
        match self {
            Self::Default => Some(160), // Or should it be None?
            Self::Low => Some(120),
            Self::Medium => Some(160),
            Self::High => Some(240),
            Self::XHigh => Some(320),
            Self::XXHigh => Some(480),
            Self::XXXHigh => Some(640),
            Self::TV => Some(213),
            Self::Any => None,
            Self::None => None,
            // TODO
            Self::__Unknown(v) => Some(v as u32),
        }
    }

    /// The Hi-DPI factor associated with the density class.  This is the factor by which an
    /// image/resource should be scaled to match its size across devices.  The baseline is a 160dpi
    /// screen (i.e., Hi-DPI factor = DPI / 160).
    /// See [the Android screen density
    /// docs](https://developer.android.com/training/multiscreen/screendensities#TaskProvideAltBmp)
    ///
    /// There are some [`Density`] values that have no associated DPI; these values return [`None`].
    pub fn approx_hidpi_factor(self) -> Option<f64> {
        match self {
            Self::Default => Some(1.), // Or should it be None?
            Self::Low => Some(0.75),
            Self::Medium => Some(1.),
            Self::High => Some(1.5),
            Self::XHigh => Some(2.),
            Self::XXHigh => Some(3.),
            Self::XXXHigh => Some(4.),
            Self::TV => Some(4. / 3.),
            Self::Any => None,
            Self::None => None,
            Self::__Unknown(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum Keyboard {
    Any = ffi::ACONFIGURATION_KEYBOARD_ANY as i32,
    NoKeys = ffi::ACONFIGURATION_KEYBOARD_NOKEYS as i32,
    Qwerty = ffi::ACONFIGURATION_KEYBOARD_QWERTY as i32,
    TwelveKey = ffi::ACONFIGURATION_KEYBOARD_12KEY as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum Navigation {
    Any = ffi::ACONFIGURATION_NAVIGATION_ANY as i32,
    NoNav = ffi::ACONFIGURATION_NAVIGATION_NONAV as i32,
    DPad = ffi::ACONFIGURATION_NAVIGATION_DPAD as i32,
    Trackball = ffi::ACONFIGURATION_NAVIGATION_TRACKBALL as i32,
    Wheel = ffi::ACONFIGURATION_NAVIGATION_WHEEL as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum KeysHidden {
    Any = ffi::ACONFIGURATION_KEYSHIDDEN_ANY as i32,
    No = ffi::ACONFIGURATION_KEYSHIDDEN_NO as i32,
    Yes = ffi::ACONFIGURATION_KEYSHIDDEN_YES as i32,
    Soft = ffi::ACONFIGURATION_KEYSHIDDEN_SOFT as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum NavHidden {
    Any = ffi::ACONFIGURATION_NAVHIDDEN_ANY as i32,
    No = ffi::ACONFIGURATION_NAVHIDDEN_NO as i32,
    Yes = ffi::ACONFIGURATION_NAVHIDDEN_YES as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum ScreenSize {
    Any = ffi::ACONFIGURATION_SCREENSIZE_ANY as i32,
    Small = ffi::ACONFIGURATION_SCREENSIZE_SMALL as i32,
    Normal = ffi::ACONFIGURATION_SCREENSIZE_NORMAL as i32,
    Large = ffi::ACONFIGURATION_SCREENSIZE_LARGE as i32,
    XLarge = ffi::ACONFIGURATION_SCREENSIZE_XLARGE as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum ScreenLong {
    Any = ffi::ACONFIGURATION_SCREENLONG_ANY as i32,
    No = ffi::ACONFIGURATION_SCREENLONG_NO as i32,
    Yes = ffi::ACONFIGURATION_SCREENLONG_YES as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum ScreenRound {
    Any = ffi::ACONFIGURATION_SCREENROUND_ANY as i32,
    No = ffi::ACONFIGURATION_SCREENROUND_NO as i32,
    Yes = ffi::ACONFIGURATION_SCREENROUND_YES as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum WideColorGamut {
    Any = ffi::ACONFIGURATION_WIDE_COLOR_GAMUT_ANY as i32,
    No = ffi::ACONFIGURATION_WIDE_COLOR_GAMUT_NO as i32,
    Yes = ffi::ACONFIGURATION_WIDE_COLOR_GAMUT_YES as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum HDR {
    Any = ffi::ACONFIGURATION_HDR_ANY as i32,
    No = ffi::ACONFIGURATION_HDR_NO as i32,
    Yes = ffi::ACONFIGURATION_HDR_YES as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum LayoutDir {
    Any = ffi::ACONFIGURATION_LAYOUTDIR_ANY as i32,
    Ltr = ffi::ACONFIGURATION_LAYOUTDIR_LTR as i32,
    Rtl = ffi::ACONFIGURATION_LAYOUTDIR_RTL as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum UiModeType {
    Any = ffi::ACONFIGURATION_UI_MODE_TYPE_ANY as i32,
    Normal = ffi::ACONFIGURATION_UI_MODE_TYPE_NORMAL as i32,
    Desk = ffi::ACONFIGURATION_UI_MODE_TYPE_DESK as i32,
    Car = ffi::ACONFIGURATION_UI_MODE_TYPE_CAR as i32,
    Television = ffi::ACONFIGURATION_UI_MODE_TYPE_TELEVISION as i32,
    Applicance = ffi::ACONFIGURATION_UI_MODE_TYPE_APPLIANCE as i32,
    Watch = ffi::ACONFIGURATION_UI_MODE_TYPE_WATCH as i32,
    VrHeadset = ffi::ACONFIGURATION_UI_MODE_TYPE_VR_HEADSET as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
#[non_exhaustive]
pub enum UiModeNight {
    Any = ffi::ACONFIGURATION_UI_MODE_NIGHT_ANY as i32,
    No = ffi::ACONFIGURATION_UI_MODE_NIGHT_NO as i32,
    Yes = ffi::ACONFIGURATION_UI_MODE_NIGHT_YES as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}
