use bitflags::bitflags;
use libc::{c_float, c_void, pid_t};

#[repr(C)]
pub struct __CGEvent(c_void);

pub type CGEventRef = *mut __CGEvent;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGMouseButton {
    #[doc(alias = "kCGMouseButtonLeft")]
    Left   = 0,
    #[doc(alias = "kCGMouseButtonRight")]
    Right  = 1,
    #[doc(alias = "kCGMouseButtonCenter")]
    Center = 2,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGScrollEventUnit {
    #[doc(alias = "kCGScrollEventUnitPixel")]
    Pixel = 0,
    #[doc(alias = "kCGScrollEventUnitLine")]
    Line  = 1,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGMomentumScrollPhase {
    #[doc(alias = "kCGMomentumScrollPhaseNone")]
    None     = 0,
    #[doc(alias = "kCGMomentumScrollPhaseBegin")]
    Begin    = 1,
    #[doc(alias = "kCGMomentumScrollPhaseContinue")]
    Continue = 2,
    #[doc(alias = "kCGMomentumScrollPhaseEnd")]
    End      = 3,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGScrollPhase {
    #[doc(alias = "kCGScrollPhaseBegan")]
    Began     = 1,
    #[doc(alias = "kCGScrollPhaseChanged")]
    Changed   = 2,
    #[doc(alias = "kCGScrollPhaseEnded")]
    Ended     = 4,
    #[doc(alias = "kCGScrollPhaseCancelled")]
    Cancelled = 8,
    #[doc(alias = "kCGScrollPhaseMayBegin")]
    MayBegin  = 128,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGGesturePhase {
    #[doc(alias = "kCGGesturePhaseNone")]
    None      = 0,
    #[doc(alias = "kCGGesturePhaseBegan")]
    Began     = 1,
    #[doc(alias = "kCGGesturePhaseChanged")]
    Changed   = 2,
    #[doc(alias = "kCGGesturePhaseEnded")]
    Ended     = 4,
    #[doc(alias = "kCGGesturePhaseCancelled")]
    Cancelled = 8,
    #[doc(alias = "kCGGesturePhaseMayBegin")]
    MayBegin  = 128,
}

// IOKit/hidsystem/IOLLEvent.h
bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct CGEventFlags: u64 {
        #[doc(alias = "kCGEventFlagMaskAlphaShift")]
        const MaskAlphaShift = 0x00010000;
        #[doc(alias = "kCGEventFlagMaskShift")]
        const MaskShift = 0x00020000;
        #[doc(alias = "kCGEventFlagMaskControl")]
        const MaskControl = 0x00040000;
        #[doc(alias = "kCGEventFlagMaskAlternate")]
        const MaskAlternate = 0x00080000;
        #[doc(alias = "kCGEventFlagMaskCommand")]
        const MaskCommand = 0x00100000;
        #[doc(alias = "kCGEventFlagMaskHelp")]
        const MaskHelp = 0x00400000;
        #[doc(alias = "kCGEventFlagMaskSecondaryFn")]
        const MaskSecondaryFn = 0x00800000;
        #[doc(alias = "kCGEventFlagMaskNumericPad")]
        const MaskNumericPad = 0x00200000;
        #[doc(alias = "kCGEventFlagMaskNonCoalesced")]
        const MaskNonCoalesced = 0x00000100;
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGEventType {
    #[doc(alias = "kCGEventNull")]
    Null                   = 0,
    #[doc(alias = "kCGEventLeftMouseDown")]
    LeftMouseDown          = 1,
    #[doc(alias = "kCGEventLeftMouseUp")]
    LeftMouseUp            = 2,
    #[doc(alias = "kCGEventRightMouseDown")]
    RightMouseDown         = 3,
    #[doc(alias = "kCGEventRightMouseUp")]
    RightMouseUp           = 4,
    #[doc(alias = "kCGEventMouseMoved")]
    MouseMoved             = 5,
    #[doc(alias = "kCGEventLeftMouseDragged")]
    LeftMouseDragged       = 6,
    #[doc(alias = "kCGEventRightMouseDragged")]
    RightMouseDragged      = 7,
    #[doc(alias = "kCGEventKeyDown")]
    KeyDown                = 10,
    #[doc(alias = "kCGEventKeyUp")]
    KeyUp                  = 11,
    #[doc(alias = "kCGEventFlagsChanged")]
    FlagsChanged           = 12,
    #[doc(alias = "kCGEventScrollWheel")]
    ScrollWheel            = 22,
    #[doc(alias = "kCGEventTabletPointer")]
    TabletPointer          = 23,
    #[doc(alias = "kCGEventTabletProximity")]
    TabletProximity        = 24,
    #[doc(alias = "kCGEventOtherMouseDown")]
    OtherMouseDown         = 25,
    #[doc(alias = "kCGEventOtherMouseUp")]
    OtherMouseUp           = 26,
    #[doc(alias = "kCGEventOtherMouseDragged")]
    OtherMouseDragged      = 27,
    #[doc(alias = "kCGEventTapDisabledByTimeout")]
    TapDisabledByTimeout   = 0xFFFFFFFE,
    #[doc(alias = "kCGEventTapDisabledByUserInput")]
    TapDisabledByUserInput = 0xFFFFFFFF,
}

pub type CGEventTimestamp = u64;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGEventField {
    #[doc(alias = "kCGMouseEventNumber")]
    MouseEventNumber               = 0,
    #[doc(alias = "kCGMouseEventClickState")]
    MouseEventClickState           = 1,
    #[doc(alias = "kCGMouseEventPressure")]
    MouseEventPressure             = 2,
    #[doc(alias = "kCGMouseEventButtonNumber")]
    MouseEventButtonNumber         = 3,
    #[doc(alias = "kCGMouseEventDeltaX")]
    MouseEventDeltaX               = 4,
    #[doc(alias = "kCGMouseEventDeltaY")]
    MouseEventDeltaY               = 5,
    #[doc(alias = "kCGMouseEventInstantMouser")]
    MouseEventInstantMouser        = 6,
    #[doc(alias = "kCGMouseEventSubtype")]
    MouseEventSubtype              = 7,
    #[doc(alias = "kCGKeyboardEventAutorepeat")]
    KeyboardEventAutorepeat        = 8,
    #[doc(alias = "kCGKeyboardEventKeycode")]
    KeyboardEventKeycode           = 9,
    #[doc(alias = "kCGKeyboardEventKeyboardType")]
    KeyboardEventKeyboardType      = 10,
    #[doc(alias = "kCGScrollWheelEventDeltaAxis1")]
    ScrollWheelEventDeltaAxis1     = 11,
    #[doc(alias = "kCGScrollWheelEventDeltaAxis2")]
    ScrollWheelEventDeltaAxis2     = 12,
    #[doc(alias = "kCGScrollWheelEventDeltaAxis3")]
    ScrollWheelEventDeltaAxis3     = 13,
    #[doc(alias = "kCGScrollWheelEventFixedPtDeltaAxis1")]
    ScrollWheelEventFixedPtDeltaAxis1 = 93,
    #[doc(alias = "kCGScrollWheelEventFixedPtDeltaAxis2")]
    ScrollWheelEventFixedPtDeltaAxis2 = 94,
    #[doc(alias = "kCGScrollWheelEventFixedPtDeltaAxis3")]
    ScrollWheelEventFixedPtDeltaAxis3 = 95,
    #[doc(alias = "kCGScrollWheelEventPointDeltaAxis1")]
    ScrollWheelEventPointDeltaAxis1 = 96,
    #[doc(alias = "kCGScrollWheelEventPointDeltaAxis2")]
    ScrollWheelEventPointDeltaAxis2 = 97,
    #[doc(alias = "kCGScrollWheelEventPointDeltaAxis3")]
    ScrollWheelEventPointDeltaAxis3 = 98,
    #[doc(alias = "kCGScrollWheelEventScrollPhase")]
    ScrollWheelEventScrollPhase    = 99,
    #[doc(alias = "kCGScrollWheelEventScrollCount")]
    ScrollWheelEventScrollCount    = 100,
    #[doc(alias = "kCGScrollWheelEventMomentumPhase")]
    ScrollWheelEventMomentumPhase  = 123,
    #[doc(alias = "kCGScrollWheelEventInstantMouser")]
    ScrollWheelEventInstantMouser  = 14,
    #[doc(alias = "kCGTabletEventPointX")]
    TabletEventPointX              = 15,
    #[doc(alias = "kCGTabletEventPointY")]
    TabletEventPointY              = 16,
    #[doc(alias = "kCGTabletEventPointZ")]
    TabletEventPointZ              = 17,
    #[doc(alias = "kCGTabletEventPointButtons")]
    TabletEventPointButtons        = 18,
    #[doc(alias = "kCGTabletEventPointPressure")]
    TabletEventPointPressure       = 19,
    #[doc(alias = "kCGTabletEventTiltX")]
    TabletEventTiltX               = 20,
    #[doc(alias = "kCGTabletEventTiltY")]
    TabletEventTiltY               = 21,
    #[doc(alias = "kCGTabletEventRotation")]
    TabletEventRotation            = 22,
    #[doc(alias = "kCGTabletEventTangentialPressure")]
    TabletEventTangentialPressure  = 23,
    #[doc(alias = "kCGTabletEventDeviceID")]
    TabletEventDeviceID            = 24,
    #[doc(alias = "kCGTabletEventVendor1")]
    TabletEventVendor1             = 25,
    #[doc(alias = "kCGTabletEventVendor2")]
    TabletEventVendor2             = 26,
    #[doc(alias = "kCGTabletEventVendor3")]
    TabletEventVendor3             = 27,
    #[doc(alias = "kCGTabletProximityEventVendorID")]
    TabletProximityEventVendorID   = 28,
    #[doc(alias = "kCGTabletProximityEventTabletID")]
    TabletProximityEventTabletID   = 29,
    #[doc(alias = "kCGTabletProximityEventPointerID")]
    TabletProximityEventPointerID  = 30,
    #[doc(alias = "kCGTabletProximityEventDeviceID")]
    TabletProximityEventDeviceID   = 31,
    #[doc(alias = "kCGTabletProximityEventSystemTabletID")]
    TabletProximityEventSystemTabletID = 32,
    #[doc(alias = "kCGTabletProximityEventVendorPointerType")]
    TabletProximityEventVendorPointerType = 33,
    #[doc(alias = "kCGTabletProximityEventVendorPointerSerialNumber")]
    TabletProximityEventVendorPointerSerialNumber = 34,
    #[doc(alias = "kCGTabletProximityEventVendorUniqueID")]
    TabletProximityEventVendorUniqueID = 35,
    #[doc(alias = "kCGTabletProximityEventCapabilityMask")]
    TabletProximityEventCapabilityMask = 36,
    #[doc(alias = "kCGTabletProximityEventPointerType")]
    TabletProximityEventPointerType = 37,
    #[doc(alias = "kCGTabletProximityEventEnterProximity")]
    TabletProximityEventEnterProximity = 38,
    #[doc(alias = "kCGEventTargetProcessSerialNumber")]
    EventTargetProcessSerialNumber = 39,
    #[doc(alias = "kCGEventTargetUnixProcessID")]
    EventTargetUnixProcessID       = 40,
    #[doc(alias = "kCGEventSourceUnixProcessID")]
    EventSourceUnixProcessID       = 41,
    #[doc(alias = "kCGEventSourceUserData")]
    EventSourceUserData            = 42,
    #[doc(alias = "kCGEventSourceUserID")]
    EventSourceUserID              = 43,
    #[doc(alias = "kCGEventSourceGroupID")]
    EventSourceGroupID             = 44,
    #[doc(alias = "kCGEventSourceStateID")]
    EventSourceStateID             = 45,
    #[doc(alias = "kCGScrollWheelEventIsContinuous")]
    ScrollWheelEventIsContinuous   = 88,
    #[doc(alias = "kCGMouseEventWindowUnderMousePointer")]
    MouseEventWindowUnderMousePointer = 91,
    #[doc(alias = "kCGMouseEventWindowUnderMousePointerThatCanHandleThisEvent")]
    MouseEventWindowUnderMousePointerThatCanHandleThisEvent = 92,
    #[doc(alias = "kCGEventUnacceleratedPointerMovementX")]
    EventUnacceleratedPointerMovementX = 170,
    #[doc(alias = "kCGEventUnacceleratedPointerMovementY")]
    EventUnacceleratedPointerMovementY = 171,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGEventMouseSubtype {
    #[doc(alias = "kCGEventMouseSubtypeDefault")]
    Default         = 0,
    #[doc(alias = "kCGEventMouseSubtypeTabletPoint")]
    TabletPoint     = 1,
    #[doc(alias = "kCGEventMouseSubtypeTabletProximity")]
    TabletProximity = 2,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGEventTapLocation {
    #[doc(alias = "kCGHIDEventTap")]
    HIDEventTap              = 0,
    #[doc(alias = "kCGSessionEventTap")]
    SessionEventTap          = 1,
    #[doc(alias = "kCGAnnotatedSessionEventTap")]
    AnnotatedSessionEventTap = 2,
}

impl Default for CGEventTapLocation {
    fn default() -> Self {
        CGEventTapLocation::HIDEventTap
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGEventTapPlacement {
    #[doc(alias = "kCGHeadInsertEventTap")]
    HeadInsertEventTap = 0,
    #[doc(alias = "kCGTailAppendEventTap")]
    TailAppendEventTap = 1,
}

pub type CGEventMask = u64;

pub const kCGEventMaskForAllEvents: CGEventMask = !0;

#[repr(C)]
pub struct __CGEventTapProxy(c_void);

pub type CGEventTapProxy = *const __CGEventTapProxy;

pub type CGEventTapCallBack = extern "C" fn(proxy: CGEventTapProxy, type_: CGEventType, event: CGEventRef, userInfo: *mut c_void) -> CGEventRef;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGEventTapOptions {
    #[doc(alias = "kCGEventTapOptionDefault")]
    Default    = 0,
    #[doc(alias = "kCGEventTapOptionListenOnly")]
    ListenOnly = 1,
}

impl Default for CGEventTapOptions {
    fn default() -> Self {
        CGEventTapOptions::Default
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CGEventTapInformation {
    pub eventTapID: u64,
    pub tapPoint: CGEventTapLocation,
    pub options: CGEventTapOptions,
    pub eventsOfInterest: CGEventMask,
    pub tappingProcess: pid_t,
    pub processBeingTapped: pid_t,
    pub enabled: bool,
    pub minUsecLatency: c_float,
    pub avgUsecLatency: c_float,
    pub maxUsecLatency: c_float,
}

#[repr(C)]
pub struct __CGEventSource(c_void);

pub type CGEventSourceRef = *mut __CGEventSource;

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGEventSourceStateID {
    #[doc(alias = "kCGEventSourceStatePrivate")]
    Private              = -1,
    #[doc(alias = "kCGEventSourceStateCombinedSessionState")]
    CombinedSessionState = 0,
    #[doc(alias = "kCGEventSourceStateHIDSystemState")]
    HIDSystemState       = 1,
}

pub type CGEventSourceKeyboardType = u32;
