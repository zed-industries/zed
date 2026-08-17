#[inline]
pub unsafe fn AddPointerInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerid: u32) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn AddPointerInteractionContext(interactioncontext : HINTERACTIONCONTEXT, pointerid : u32) -> windows_core::HRESULT);
    unsafe { AddPointerInteractionContext(interactioncontext, pointerid).ok() }
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn BufferPointerPacketsInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerinfo: &[super::Input::Pointer::POINTER_INFO]) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn BufferPointerPacketsInteractionContext(interactioncontext : HINTERACTIONCONTEXT, entriescount : u32, pointerinfo : *const super::Input::Pointer:: POINTER_INFO) -> windows_core::HRESULT);
    unsafe { BufferPointerPacketsInteractionContext(interactioncontext, pointerinfo.len().try_into().unwrap(), core::mem::transmute(pointerinfo.as_ptr())).ok() }
}
#[inline]
pub unsafe fn CreateInteractionContext() -> windows_core::Result<HINTERACTIONCONTEXT> {
    windows_core::link!("ninput.dll" "system" fn CreateInteractionContext(interactioncontext : *mut HINTERACTIONCONTEXT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateInteractionContext(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DestroyInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn DestroyInteractionContext(interactioncontext : HINTERACTIONCONTEXT) -> windows_core::HRESULT);
    unsafe { DestroyInteractionContext(interactioncontext).ok() }
}
#[inline]
pub unsafe fn GetCrossSlideParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, threshold: CROSS_SLIDE_THRESHOLD) -> windows_core::Result<f32> {
    windows_core::link!("ninput.dll" "system" fn GetCrossSlideParameterInteractionContext(interactioncontext : HINTERACTIONCONTEXT, threshold : CROSS_SLIDE_THRESHOLD, distance : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetCrossSlideParameterInteractionContext(interactioncontext, threshold, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetHoldParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: HOLD_PARAMETER) -> windows_core::Result<f32> {
    windows_core::link!("ninput.dll" "system" fn GetHoldParameterInteractionContext(interactioncontext : HINTERACTIONCONTEXT, parameter : HOLD_PARAMETER, value : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetHoldParameterInteractionContext(interactioncontext, parameter, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetInertiaParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, inertiaparameter: INERTIA_PARAMETER) -> windows_core::Result<f32> {
    windows_core::link!("ninput.dll" "system" fn GetInertiaParameterInteractionContext(interactioncontext : HINTERACTIONCONTEXT, inertiaparameter : INERTIA_PARAMETER, value : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetInertiaParameterInteractionContext(interactioncontext, inertiaparameter, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetInteractionConfigurationInteractionContext(interactioncontext: HINTERACTIONCONTEXT, configuration: &mut [INTERACTION_CONTEXT_CONFIGURATION]) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn GetInteractionConfigurationInteractionContext(interactioncontext : HINTERACTIONCONTEXT, configurationcount : u32, configuration : *mut INTERACTION_CONTEXT_CONFIGURATION) -> windows_core::HRESULT);
    unsafe { GetInteractionConfigurationInteractionContext(interactioncontext, configuration.len().try_into().unwrap(), core::mem::transmute(configuration.as_ptr())).ok() }
}
#[inline]
pub unsafe fn GetMouseWheelParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: MOUSE_WHEEL_PARAMETER) -> windows_core::Result<f32> {
    windows_core::link!("ninput.dll" "system" fn GetMouseWheelParameterInteractionContext(interactioncontext : HINTERACTIONCONTEXT, parameter : MOUSE_WHEEL_PARAMETER, value : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetMouseWheelParameterInteractionContext(interactioncontext, parameter, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetPropertyInteractionContext(interactioncontext: HINTERACTIONCONTEXT, contextproperty: INTERACTION_CONTEXT_PROPERTY) -> windows_core::Result<u32> {
    windows_core::link!("ninput.dll" "system" fn GetPropertyInteractionContext(interactioncontext : HINTERACTIONCONTEXT, contextproperty : INTERACTION_CONTEXT_PROPERTY, value : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetPropertyInteractionContext(interactioncontext, contextproperty, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetStateInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerinfo: Option<*const super::Input::Pointer::POINTER_INFO>) -> windows_core::Result<INTERACTION_STATE> {
    windows_core::link!("ninput.dll" "system" fn GetStateInteractionContext(interactioncontext : HINTERACTIONCONTEXT, pointerinfo : *const super::Input::Pointer:: POINTER_INFO, state : *mut INTERACTION_STATE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetStateInteractionContext(interactioncontext, pointerinfo.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetTapParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TAP_PARAMETER) -> windows_core::Result<f32> {
    windows_core::link!("ninput.dll" "system" fn GetTapParameterInteractionContext(interactioncontext : HINTERACTIONCONTEXT, parameter : TAP_PARAMETER, value : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetTapParameterInteractionContext(interactioncontext, parameter, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetTranslationParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TRANSLATION_PARAMETER) -> windows_core::Result<f32> {
    windows_core::link!("ninput.dll" "system" fn GetTranslationParameterInteractionContext(interactioncontext : HINTERACTIONCONTEXT, parameter : TRANSLATION_PARAMETER, value : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetTranslationParameterInteractionContext(interactioncontext, parameter, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn ProcessBufferedPacketsInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn ProcessBufferedPacketsInteractionContext(interactioncontext : HINTERACTIONCONTEXT) -> windows_core::HRESULT);
    unsafe { ProcessBufferedPacketsInteractionContext(interactioncontext).ok() }
}
#[inline]
pub unsafe fn ProcessInertiaInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn ProcessInertiaInteractionContext(interactioncontext : HINTERACTIONCONTEXT) -> windows_core::HRESULT);
    unsafe { ProcessInertiaInteractionContext(interactioncontext).ok() }
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn ProcessPointerFramesInteractionContext(interactioncontext: HINTERACTIONCONTEXT, entriescount: u32, pointercount: u32, pointerinfo: *const super::Input::Pointer::POINTER_INFO) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn ProcessPointerFramesInteractionContext(interactioncontext : HINTERACTIONCONTEXT, entriescount : u32, pointercount : u32, pointerinfo : *const super::Input::Pointer:: POINTER_INFO) -> windows_core::HRESULT);
    unsafe { ProcessPointerFramesInteractionContext(interactioncontext, entriescount, pointercount, pointerinfo).ok() }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn RegisterOutputCallbackInteractionContext(interactioncontext: HINTERACTIONCONTEXT, outputcallback: INTERACTION_CONTEXT_OUTPUT_CALLBACK, clientdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn RegisterOutputCallbackInteractionContext(interactioncontext : HINTERACTIONCONTEXT, outputcallback : INTERACTION_CONTEXT_OUTPUT_CALLBACK, clientdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RegisterOutputCallbackInteractionContext(interactioncontext, outputcallback, clientdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn RegisterOutputCallbackInteractionContext2(interactioncontext: HINTERACTIONCONTEXT, outputcallback: INTERACTION_CONTEXT_OUTPUT_CALLBACK2, clientdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn RegisterOutputCallbackInteractionContext2(interactioncontext : HINTERACTIONCONTEXT, outputcallback : INTERACTION_CONTEXT_OUTPUT_CALLBACK2, clientdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RegisterOutputCallbackInteractionContext2(interactioncontext, outputcallback, clientdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn RemovePointerInteractionContext(interactioncontext: HINTERACTIONCONTEXT, pointerid: u32) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn RemovePointerInteractionContext(interactioncontext : HINTERACTIONCONTEXT, pointerid : u32) -> windows_core::HRESULT);
    unsafe { RemovePointerInteractionContext(interactioncontext, pointerid).ok() }
}
#[inline]
pub unsafe fn ResetInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn ResetInteractionContext(interactioncontext : HINTERACTIONCONTEXT) -> windows_core::HRESULT);
    unsafe { ResetInteractionContext(interactioncontext).ok() }
}
#[inline]
pub unsafe fn SetCrossSlideParametersInteractionContext(interactioncontext: HINTERACTIONCONTEXT, crossslideparameters: &[CROSS_SLIDE_PARAMETER]) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn SetCrossSlideParametersInteractionContext(interactioncontext : HINTERACTIONCONTEXT, parametercount : u32, crossslideparameters : *const CROSS_SLIDE_PARAMETER) -> windows_core::HRESULT);
    unsafe { SetCrossSlideParametersInteractionContext(interactioncontext, crossslideparameters.len().try_into().unwrap(), core::mem::transmute(crossslideparameters.as_ptr())).ok() }
}
#[inline]
pub unsafe fn SetHoldParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: HOLD_PARAMETER, value: f32) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn SetHoldParameterInteractionContext(interactioncontext : HINTERACTIONCONTEXT, parameter : HOLD_PARAMETER, value : f32) -> windows_core::HRESULT);
    unsafe { SetHoldParameterInteractionContext(interactioncontext, parameter, value).ok() }
}
#[inline]
pub unsafe fn SetInertiaParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, inertiaparameter: INERTIA_PARAMETER, value: f32) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn SetInertiaParameterInteractionContext(interactioncontext : HINTERACTIONCONTEXT, inertiaparameter : INERTIA_PARAMETER, value : f32) -> windows_core::HRESULT);
    unsafe { SetInertiaParameterInteractionContext(interactioncontext, inertiaparameter, value).ok() }
}
#[inline]
pub unsafe fn SetInteractionConfigurationInteractionContext(interactioncontext: HINTERACTIONCONTEXT, configuration: &[INTERACTION_CONTEXT_CONFIGURATION]) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn SetInteractionConfigurationInteractionContext(interactioncontext : HINTERACTIONCONTEXT, configurationcount : u32, configuration : *const INTERACTION_CONTEXT_CONFIGURATION) -> windows_core::HRESULT);
    unsafe { SetInteractionConfigurationInteractionContext(interactioncontext, configuration.len().try_into().unwrap(), core::mem::transmute(configuration.as_ptr())).ok() }
}
#[inline]
pub unsafe fn SetMouseWheelParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: MOUSE_WHEEL_PARAMETER, value: f32) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn SetMouseWheelParameterInteractionContext(interactioncontext : HINTERACTIONCONTEXT, parameter : MOUSE_WHEEL_PARAMETER, value : f32) -> windows_core::HRESULT);
    unsafe { SetMouseWheelParameterInteractionContext(interactioncontext, parameter, value).ok() }
}
#[inline]
pub unsafe fn SetPivotInteractionContext(interactioncontext: HINTERACTIONCONTEXT, x: f32, y: f32, radius: f32) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn SetPivotInteractionContext(interactioncontext : HINTERACTIONCONTEXT, x : f32, y : f32, radius : f32) -> windows_core::HRESULT);
    unsafe { SetPivotInteractionContext(interactioncontext, x, y, radius).ok() }
}
#[inline]
pub unsafe fn SetPropertyInteractionContext(interactioncontext: HINTERACTIONCONTEXT, contextproperty: INTERACTION_CONTEXT_PROPERTY, value: u32) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn SetPropertyInteractionContext(interactioncontext : HINTERACTIONCONTEXT, contextproperty : INTERACTION_CONTEXT_PROPERTY, value : u32) -> windows_core::HRESULT);
    unsafe { SetPropertyInteractionContext(interactioncontext, contextproperty, value).ok() }
}
#[inline]
pub unsafe fn SetTapParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TAP_PARAMETER, value: f32) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn SetTapParameterInteractionContext(interactioncontext : HINTERACTIONCONTEXT, parameter : TAP_PARAMETER, value : f32) -> windows_core::HRESULT);
    unsafe { SetTapParameterInteractionContext(interactioncontext, parameter, value).ok() }
}
#[inline]
pub unsafe fn SetTranslationParameterInteractionContext(interactioncontext: HINTERACTIONCONTEXT, parameter: TRANSLATION_PARAMETER, value: f32) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn SetTranslationParameterInteractionContext(interactioncontext : HINTERACTIONCONTEXT, parameter : TRANSLATION_PARAMETER, value : f32) -> windows_core::HRESULT);
    unsafe { SetTranslationParameterInteractionContext(interactioncontext, parameter, value).ok() }
}
#[inline]
pub unsafe fn StopInteractionContext(interactioncontext: HINTERACTIONCONTEXT) -> windows_core::Result<()> {
    windows_core::link!("ninput.dll" "system" fn StopInteractionContext(interactioncontext : HINTERACTIONCONTEXT) -> windows_core::HRESULT);
    unsafe { StopInteractionContext(interactioncontext).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CROSS_SLIDE_FLAGS(pub u32);
impl CROSS_SLIDE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CROSS_SLIDE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CROSS_SLIDE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CROSS_SLIDE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CROSS_SLIDE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CROSS_SLIDE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CROSS_SLIDE_FLAGS_MAX: CROSS_SLIDE_FLAGS = CROSS_SLIDE_FLAGS(4294967295u32);
pub const CROSS_SLIDE_FLAGS_NONE: CROSS_SLIDE_FLAGS = CROSS_SLIDE_FLAGS(0u32);
pub const CROSS_SLIDE_FLAGS_REARRANGE: CROSS_SLIDE_FLAGS = CROSS_SLIDE_FLAGS(4u32);
pub const CROSS_SLIDE_FLAGS_SELECT: CROSS_SLIDE_FLAGS = CROSS_SLIDE_FLAGS(1u32);
pub const CROSS_SLIDE_FLAGS_SPEED_BUMP: CROSS_SLIDE_FLAGS = CROSS_SLIDE_FLAGS(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CROSS_SLIDE_PARAMETER {
    pub threshold: CROSS_SLIDE_THRESHOLD,
    pub distance: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CROSS_SLIDE_THRESHOLD(pub i32);
pub const CROSS_SLIDE_THRESHOLD_COUNT: CROSS_SLIDE_THRESHOLD = CROSS_SLIDE_THRESHOLD(4i32);
pub const CROSS_SLIDE_THRESHOLD_MAX: CROSS_SLIDE_THRESHOLD = CROSS_SLIDE_THRESHOLD(-1i32);
pub const CROSS_SLIDE_THRESHOLD_REARRANGE_START: CROSS_SLIDE_THRESHOLD = CROSS_SLIDE_THRESHOLD(3i32);
pub const CROSS_SLIDE_THRESHOLD_SELECT_START: CROSS_SLIDE_THRESHOLD = CROSS_SLIDE_THRESHOLD(0i32);
pub const CROSS_SLIDE_THRESHOLD_SPEED_BUMP_END: CROSS_SLIDE_THRESHOLD = CROSS_SLIDE_THRESHOLD(2i32);
pub const CROSS_SLIDE_THRESHOLD_SPEED_BUMP_START: CROSS_SLIDE_THRESHOLD = CROSS_SLIDE_THRESHOLD(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HINTERACTIONCONTEXT(pub *mut core::ffi::c_void);
impl HINTERACTIONCONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HINTERACTIONCONTEXT {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("ninput.dll" "system" fn DestroyInteractionContext(interactioncontext : *mut core::ffi::c_void) -> i32);
            unsafe {
                DestroyInteractionContext(self.0);
            }
        }
    }
}
impl Default for HINTERACTIONCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HOLD_PARAMETER(pub i32);
pub const HOLD_PARAMETER_MAX: HOLD_PARAMETER = HOLD_PARAMETER(-1i32);
pub const HOLD_PARAMETER_MAX_CONTACT_COUNT: HOLD_PARAMETER = HOLD_PARAMETER(1i32);
pub const HOLD_PARAMETER_MIN_CONTACT_COUNT: HOLD_PARAMETER = HOLD_PARAMETER(0i32);
pub const HOLD_PARAMETER_THRESHOLD_RADIUS: HOLD_PARAMETER = HOLD_PARAMETER(2i32);
pub const HOLD_PARAMETER_THRESHOLD_START_DELAY: HOLD_PARAMETER = HOLD_PARAMETER(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INERTIA_PARAMETER(pub i32);
pub const INERTIA_PARAMETER_EXPANSION_DECELERATION: INERTIA_PARAMETER = INERTIA_PARAMETER(5i32);
pub const INERTIA_PARAMETER_EXPANSION_EXPANSION: INERTIA_PARAMETER = INERTIA_PARAMETER(6i32);
pub const INERTIA_PARAMETER_MAX: INERTIA_PARAMETER = INERTIA_PARAMETER(-1i32);
pub const INERTIA_PARAMETER_ROTATION_ANGLE: INERTIA_PARAMETER = INERTIA_PARAMETER(4i32);
pub const INERTIA_PARAMETER_ROTATION_DECELERATION: INERTIA_PARAMETER = INERTIA_PARAMETER(3i32);
pub const INERTIA_PARAMETER_TRANSLATION_DECELERATION: INERTIA_PARAMETER = INERTIA_PARAMETER(1i32);
pub const INERTIA_PARAMETER_TRANSLATION_DISPLACEMENT: INERTIA_PARAMETER = INERTIA_PARAMETER(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERACTION_ARGUMENTS_CROSS_SLIDE {
    pub flags: CROSS_SLIDE_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERACTION_ARGUMENTS_MANIPULATION {
    pub delta: MANIPULATION_TRANSFORM,
    pub cumulative: MANIPULATION_TRANSFORM,
    pub velocity: MANIPULATION_VELOCITY,
    pub railsState: MANIPULATION_RAILS_STATE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERACTION_ARGUMENTS_TAP {
    pub count: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERACTION_CONFIGURATION_FLAGS(pub u32);
impl INTERACTION_CONFIGURATION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for INTERACTION_CONFIGURATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for INTERACTION_CONFIGURATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for INTERACTION_CONFIGURATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for INTERACTION_CONFIGURATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for INTERACTION_CONFIGURATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1u32);
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_EXACT: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(32u32);
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_HORIZONTAL: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(2u32);
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_REARRANGE: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(16u32);
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_SELECT: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(4u32);
pub const INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_SPEED_BUMP: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(8u32);
pub const INTERACTION_CONFIGURATION_FLAG_DRAG: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1u32);
pub const INTERACTION_CONFIGURATION_FLAG_HOLD: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1u32);
pub const INTERACTION_CONFIGURATION_FLAG_HOLD_MOUSE: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(2u32);
pub const INTERACTION_CONFIGURATION_FLAG_HOLD_MULTIPLE_FINGER: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(4u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_EXACT: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1024u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_MULTIPLE_FINGER_PANNING: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(2048u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_RAILS_X: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(256u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_RAILS_Y: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(512u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_ROTATION: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(8u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_ROTATION_INERTIA: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(64u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_SCALING: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(16u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_SCALING_INERTIA: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(128u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_INERTIA: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(32u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_X: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(2u32);
pub const INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_Y: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(4u32);
pub const INTERACTION_CONFIGURATION_FLAG_MAX: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(4294967295u32);
pub const INTERACTION_CONFIGURATION_FLAG_NONE: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(0u32);
pub const INTERACTION_CONFIGURATION_FLAG_SECONDARY_TAP: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1u32);
pub const INTERACTION_CONFIGURATION_FLAG_TAP: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(1u32);
pub const INTERACTION_CONFIGURATION_FLAG_TAP_DOUBLE: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(2u32);
pub const INTERACTION_CONFIGURATION_FLAG_TAP_MULTIPLE_FINGER: INTERACTION_CONFIGURATION_FLAGS = INTERACTION_CONFIGURATION_FLAGS(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTERACTION_CONTEXT_CONFIGURATION {
    pub interactionId: INTERACTION_ID,
    pub enable: INTERACTION_CONFIGURATION_FLAGS,
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct INTERACTION_CONTEXT_OUTPUT {
    pub interactionId: INTERACTION_ID,
    pub interactionFlags: INTERACTION_FLAGS,
    pub inputType: super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub x: f32,
    pub y: f32,
    pub arguments: INTERACTION_CONTEXT_OUTPUT_0,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for INTERACTION_CONTEXT_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union INTERACTION_CONTEXT_OUTPUT_0 {
    pub manipulation: INTERACTION_ARGUMENTS_MANIPULATION,
    pub tap: INTERACTION_ARGUMENTS_TAP,
    pub crossSlide: INTERACTION_ARGUMENTS_CROSS_SLIDE,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for INTERACTION_CONTEXT_OUTPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct INTERACTION_CONTEXT_OUTPUT2 {
    pub interactionId: INTERACTION_ID,
    pub interactionFlags: INTERACTION_FLAGS,
    pub inputType: super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub contactCount: u32,
    pub currentContactCount: u32,
    pub x: f32,
    pub y: f32,
    pub arguments: INTERACTION_CONTEXT_OUTPUT2_0,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for INTERACTION_CONTEXT_OUTPUT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub union INTERACTION_CONTEXT_OUTPUT2_0 {
    pub manipulation: INTERACTION_ARGUMENTS_MANIPULATION,
    pub tap: INTERACTION_ARGUMENTS_TAP,
    pub crossSlide: INTERACTION_ARGUMENTS_CROSS_SLIDE,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for INTERACTION_CONTEXT_OUTPUT2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type INTERACTION_CONTEXT_OUTPUT_CALLBACK = Option<unsafe extern "system" fn(clientdata: *const core::ffi::c_void, output: *const INTERACTION_CONTEXT_OUTPUT)>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type INTERACTION_CONTEXT_OUTPUT_CALLBACK2 = Option<unsafe extern "system" fn(clientdata: *const core::ffi::c_void, output: *const INTERACTION_CONTEXT_OUTPUT2)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERACTION_CONTEXT_PROPERTY(pub i32);
pub const INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS: INTERACTION_CONTEXT_PROPERTY = INTERACTION_CONTEXT_PROPERTY(3i32);
pub const INTERACTION_CONTEXT_PROPERTY_INTERACTION_UI_FEEDBACK: INTERACTION_CONTEXT_PROPERTY = INTERACTION_CONTEXT_PROPERTY(2i32);
pub const INTERACTION_CONTEXT_PROPERTY_MAX: INTERACTION_CONTEXT_PROPERTY = INTERACTION_CONTEXT_PROPERTY(-1i32);
pub const INTERACTION_CONTEXT_PROPERTY_MEASUREMENT_UNITS: INTERACTION_CONTEXT_PROPERTY = INTERACTION_CONTEXT_PROPERTY(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERACTION_FLAGS(pub u32);
impl INTERACTION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for INTERACTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for INTERACTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for INTERACTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for INTERACTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for INTERACTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const INTERACTION_FLAG_BEGIN: INTERACTION_FLAGS = INTERACTION_FLAGS(1u32);
pub const INTERACTION_FLAG_CANCEL: INTERACTION_FLAGS = INTERACTION_FLAGS(4u32);
pub const INTERACTION_FLAG_END: INTERACTION_FLAGS = INTERACTION_FLAGS(2u32);
pub const INTERACTION_FLAG_INERTIA: INTERACTION_FLAGS = INTERACTION_FLAGS(8u32);
pub const INTERACTION_FLAG_MAX: INTERACTION_FLAGS = INTERACTION_FLAGS(4294967295u32);
pub const INTERACTION_FLAG_NONE: INTERACTION_FLAGS = INTERACTION_FLAGS(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERACTION_ID(pub i32);
pub const INTERACTION_ID_CROSS_SLIDE: INTERACTION_ID = INTERACTION_ID(6i32);
pub const INTERACTION_ID_DRAG: INTERACTION_ID = INTERACTION_ID(5i32);
pub const INTERACTION_ID_HOLD: INTERACTION_ID = INTERACTION_ID(4i32);
pub const INTERACTION_ID_MANIPULATION: INTERACTION_ID = INTERACTION_ID(1i32);
pub const INTERACTION_ID_MAX: INTERACTION_ID = INTERACTION_ID(-1i32);
pub const INTERACTION_ID_NONE: INTERACTION_ID = INTERACTION_ID(0i32);
pub const INTERACTION_ID_SECONDARY_TAP: INTERACTION_ID = INTERACTION_ID(3i32);
pub const INTERACTION_ID_TAP: INTERACTION_ID = INTERACTION_ID(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERACTION_STATE(pub i32);
pub const INTERACTION_STATE_IDLE: INTERACTION_STATE = INTERACTION_STATE(0i32);
pub const INTERACTION_STATE_IN_INTERACTION: INTERACTION_STATE = INTERACTION_STATE(1i32);
pub const INTERACTION_STATE_MAX: INTERACTION_STATE = INTERACTION_STATE(-1i32);
pub const INTERACTION_STATE_POSSIBLE_DOUBLE_TAP: INTERACTION_STATE = INTERACTION_STATE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MANIPULATION_RAILS_STATE(pub i32);
pub const MANIPULATION_RAILS_STATE_FREE: MANIPULATION_RAILS_STATE = MANIPULATION_RAILS_STATE(1i32);
pub const MANIPULATION_RAILS_STATE_MAX: MANIPULATION_RAILS_STATE = MANIPULATION_RAILS_STATE(-1i32);
pub const MANIPULATION_RAILS_STATE_RAILED: MANIPULATION_RAILS_STATE = MANIPULATION_RAILS_STATE(2i32);
pub const MANIPULATION_RAILS_STATE_UNDECIDED: MANIPULATION_RAILS_STATE = MANIPULATION_RAILS_STATE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MANIPULATION_TRANSFORM {
    pub translationX: f32,
    pub translationY: f32,
    pub scale: f32,
    pub expansion: f32,
    pub rotation: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MANIPULATION_VELOCITY {
    pub velocityX: f32,
    pub velocityY: f32,
    pub velocityExpansion: f32,
    pub velocityAngular: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOUSE_WHEEL_PARAMETER(pub i32);
pub const MOUSE_WHEEL_PARAMETER_CHAR_TRANSLATION_X: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(1i32);
pub const MOUSE_WHEEL_PARAMETER_CHAR_TRANSLATION_Y: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(2i32);
pub const MOUSE_WHEEL_PARAMETER_DELTA_ROTATION: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(4i32);
pub const MOUSE_WHEEL_PARAMETER_DELTA_SCALE: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(3i32);
pub const MOUSE_WHEEL_PARAMETER_MAX: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(-1i32);
pub const MOUSE_WHEEL_PARAMETER_PAGE_TRANSLATION_X: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(5i32);
pub const MOUSE_WHEEL_PARAMETER_PAGE_TRANSLATION_Y: MOUSE_WHEEL_PARAMETER = MOUSE_WHEEL_PARAMETER(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TAP_PARAMETER(pub i32);
pub const TAP_PARAMETER_MAX: TAP_PARAMETER = TAP_PARAMETER(-1i32);
pub const TAP_PARAMETER_MAX_CONTACT_COUNT: TAP_PARAMETER = TAP_PARAMETER(1i32);
pub const TAP_PARAMETER_MIN_CONTACT_COUNT: TAP_PARAMETER = TAP_PARAMETER(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSLATION_PARAMETER(pub i32);
pub const TRANSLATION_PARAMETER_MAX: TRANSLATION_PARAMETER = TRANSLATION_PARAMETER(-1i32);
pub const TRANSLATION_PARAMETER_MAX_CONTACT_COUNT: TRANSLATION_PARAMETER = TRANSLATION_PARAMETER(1i32);
pub const TRANSLATION_PARAMETER_MIN_CONTACT_COUNT: TRANSLATION_PARAMETER = TRANSLATION_PARAMETER(0i32);
