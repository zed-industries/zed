// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};

use super::xlib::{
    Display, Region, Screen, Visual, XEvent, XGCValues, XSelectionRequestEvent,
    XSetWindowAttributes, XrmOptionDescList, XrmValue, _XrmHashBucketRec, GC,
};

//
// functions
//

x11_link! { Xt, xt, ["libXt.so.6", "libXt.so"], 300,
  pub fn XtAddActions (_2: *mut XtActionsRec, _1: c_uint) -> (),
  pub fn XtAddCallback (_4: Widget, _3: *const c_char, _2: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut c_void)>, _1: *mut c_void) -> (),
  pub fn XtAddCallbacks (_3: Widget, _2: *const c_char, _1: XtCallbackList) -> (),
  pub fn XtAddConverter (_5: *const c_char, _4: *const c_char, _3: Option<unsafe extern "C" fn (*mut XrmValue, *mut c_uint, *mut XrmValue, *mut XrmValue)>, _2: XtConvertArgList, _1: c_uint) -> (),
  pub fn XtAddEventHandler (_5: Widget, _4: c_ulong, _3: c_char, _2: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut XEvent, *mut c_char)>, _1: *mut c_void) -> (),
  pub fn XtAddExposureToRegion (_2: *mut XEvent, _1: Region) -> (),
  pub fn XtAddGrab (_3: Widget, _2: c_char, _1: c_char) -> (),
  pub fn XtAddInput (_4: c_int, _3: *mut c_void, _2: Option<unsafe extern "C" fn (*mut c_void, *mut c_int, *mut c_ulong)>, _1: *mut c_void) -> c_ulong,
  pub fn XtAddRawEventHandler (_5: Widget, _4: c_ulong, _3: c_char, _2: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut XEvent, *mut c_char)>, _1: *mut c_void) -> (),
  pub fn XtAddSignal (_2: Option<unsafe extern "C" fn (*mut c_void, *mut c_ulong)>, _1: *mut c_void) -> c_ulong,
  pub fn XtAddTimeOut (_3: c_ulong, _2: Option<unsafe extern "C" fn (*mut c_void, *mut c_ulong)>, _1: *mut c_void) -> c_ulong,
  pub fn XtAddWorkProc (_2: Option<unsafe extern "C" fn (*mut c_void) -> c_char>, _1: *mut c_void) -> c_ulong,
  pub fn XtAllocateGC (_6: Widget, _5: c_uint, _4: c_ulong, _3: *mut XGCValues, _2: c_ulong, _1: c_ulong) -> GC,
  pub fn XtAppAddActionHook (_3: XtAppContext, _2: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut c_char, *mut XEvent, *mut *mut c_char, *mut c_uint)>, _1: *mut c_void) -> *mut c_void,
  pub fn XtAppAddActions (_3: XtAppContext, _2: *mut XtActionsRec, _1: c_uint) -> (),
  pub fn XtAppAddBlockHook (_3: XtAppContext, _2: Option<unsafe extern "C" fn (*mut c_void)>, _1: *mut c_void) -> c_ulong,
  pub fn XtAppAddConverter (_6: XtAppContext, _5: *const c_char, _4: *const c_char, _3: Option<unsafe extern "C" fn (*mut XrmValue, *mut c_uint, *mut XrmValue, *mut XrmValue)>, _2: XtConvertArgList, _1: c_uint) -> (),
  pub fn XtAppAddInput (_5: XtAppContext, _4: c_int, _3: *mut c_void, _2: Option<unsafe extern "C" fn (*mut c_void, *mut c_int, *mut c_ulong)>, _1: *mut c_void) -> c_ulong,
  pub fn XtAppAddSignal (_3: XtAppContext, _2: Option<unsafe extern "C" fn (*mut c_void, *mut c_ulong)>, _1: *mut c_void) -> c_ulong,
  pub fn XtAppAddTimeOut (_4: XtAppContext, _3: c_ulong, _2: Option<unsafe extern "C" fn (*mut c_void, *mut c_ulong)>, _1: *mut c_void) -> c_ulong,
  pub fn XtAppAddWorkProc (_3: XtAppContext, _2: Option<unsafe extern "C" fn (*mut c_void) -> c_char>, _1: *mut c_void) -> c_ulong,
  pub fn XtAppCreateShell (_6: *const c_char, _5: *const c_char, _4: WidgetClass, _3: *mut Display, _2: *mut Arg, _1: c_uint) -> Widget,
  pub fn XtAppError (_2: XtAppContext, _1: *const c_char) -> (),
  pub fn XtAppErrorMsg (_7: XtAppContext, _6: *const c_char, _5: *const c_char, _4: *const c_char, _3: *const c_char, _2: *mut *mut c_char, _1: *mut c_uint) -> (),
  pub fn XtAppGetErrorDatabase (_1: XtAppContext) -> *mut *mut _XrmHashBucketRec,
  pub fn XtAppGetErrorDatabaseText (_8: XtAppContext, _7: *const c_char, _6: *const c_char, _5: *const c_char, _4: *const c_char, _3: *mut c_char, _2: c_int, _1: *mut _XrmHashBucketRec) -> (),
  pub fn XtAppGetExitFlag (_1: XtAppContext) -> c_char,
  pub fn XtAppGetSelectionTimeout (_1: XtAppContext) -> c_ulong,
  pub fn XtAppInitialize (_9: *mut XtAppContext, _8: *const c_char, _7: XrmOptionDescList, _6: c_uint, _5: *mut c_int, _4: *mut *mut c_char, _3: *mut *mut c_char, _2: *mut Arg, _1: c_uint) -> Widget,
  pub fn XtAppLock (_1: XtAppContext) -> (),
  pub fn XtAppMainLoop (_1: XtAppContext) -> (),
  pub fn XtAppNextEvent (_2: XtAppContext, _1: *mut XEvent) -> (),
  pub fn XtAppPeekEvent (_2: XtAppContext, _1: *mut XEvent) -> c_char,
  pub fn XtAppPending (_1: XtAppContext) -> c_ulong,
  pub fn XtAppProcessEvent (_2: XtAppContext, _1: c_ulong) -> (),
  pub fn XtAppReleaseCacheRefs (_2: XtAppContext, _1: *mut *mut c_void) -> (),
  pub fn XtAppSetErrorHandler (_2: XtAppContext, _1: Option<unsafe extern "C" fn (*mut c_char)>) -> Option<unsafe extern "C" fn (*mut c_char)>,
  pub fn XtAppSetErrorMsgHandler (_2: XtAppContext, _1: Option<unsafe extern "C" fn (*mut c_char, *mut c_char, *mut c_char, *mut c_char, *mut *mut c_char, *mut c_uint)>) -> Option<unsafe extern "C" fn (*mut c_char, *mut c_char, *mut c_char, *mut c_char, *mut *mut c_char, *mut c_uint)>,
  pub fn XtAppSetExitFlag (_1: XtAppContext) -> (),
  pub fn XtAppSetFallbackResources (_2: XtAppContext, _1: *mut *mut c_char) -> (),
  pub fn XtAppSetSelectionTimeout (_2: XtAppContext, _1: c_ulong) -> (),
  pub fn XtAppSetTypeConverter (_8: XtAppContext, _7: *const c_char, _6: *const c_char, _5: Option<unsafe extern "C" fn (*mut Display, *mut XrmValue, *mut c_uint, *mut XrmValue, *mut XrmValue, *mut *mut c_void) -> c_char>, _4: XtConvertArgList, _3: c_uint, _2: c_int, _1: Option<unsafe extern "C" fn (XtAppContext, *mut XrmValue, *mut c_void, *mut XrmValue, *mut c_uint)>) -> (),
  pub fn XtAppSetWarningHandler (_2: XtAppContext, _1: Option<unsafe extern "C" fn (*mut c_char)>) -> Option<unsafe extern "C" fn (*mut c_char)>,
  pub fn XtAppSetWarningMsgHandler (_2: XtAppContext, _1: Option<unsafe extern "C" fn (*mut c_char, *mut c_char, *mut c_char, *mut c_char, *mut *mut c_char, *mut c_uint)>) -> Option<unsafe extern "C" fn (*mut c_char, *mut c_char, *mut c_char, *mut c_char, *mut *mut c_char, *mut c_uint)>,
  pub fn XtAppUnlock (_1: XtAppContext) -> (),
  pub fn XtAppWarning (_2: XtAppContext, _1: *const c_char) -> (),
  pub fn XtAppWarningMsg (_7: XtAppContext, _6: *const c_char, _5: *const c_char, _4: *const c_char, _3: *const c_char, _2: *mut *mut c_char, _1: *mut c_uint) -> (),
  pub fn XtAugmentTranslations (_2: Widget, _1: *mut _TranslationData) -> (),
  pub fn XtBuildEventMask (_1: Widget) -> c_ulong,
  pub fn XtCallAcceptFocus (_2: Widget, _1: *mut c_ulong) -> c_char,
  pub fn XtCallActionProc (_5: Widget, _4: *const c_char, _3: *mut XEvent, _2: *mut *mut c_char, _1: c_uint) -> (),
  pub fn XtCallbackExclusive (_3: Widget, _2: *mut c_void, _1: *mut c_void) -> (),
  pub fn XtCallbackNone (_3: Widget, _2: *mut c_void, _1: *mut c_void) -> (),
  pub fn XtCallbackNonexclusive (_3: Widget, _2: *mut c_void, _1: *mut c_void) -> (),
  pub fn XtCallbackPopdown (_3: Widget, _2: *mut c_void, _1: *mut c_void) -> (),
  pub fn XtCallbackReleaseCacheRef (_3: Widget, _2: *mut c_void, _1: *mut c_void) -> (),
  pub fn XtCallbackReleaseCacheRefList (_3: Widget, _2: *mut c_void, _1: *mut c_void) -> (),
  pub fn XtCallCallbackList (_3: Widget, _2: XtCallbackList, _1: *mut c_void) -> (),
  pub fn XtCallCallbacks (_3: Widget, _2: *const c_char, _1: *mut c_void) -> (),
  pub fn XtCallConverter (_7: *mut Display, _6: Option<unsafe extern "C" fn (*mut Display, *mut XrmValue, *mut c_uint, *mut XrmValue, *mut XrmValue, *mut *mut c_void) -> c_char>, _5: *mut XrmValue, _4: c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCalloc (_2: c_uint, _1: c_uint) -> *mut c_char,
  pub fn XtCancelSelectionRequest (_2: Widget, _1: c_ulong) -> (),
  pub fn XtChangeManagedSet (_6: *mut Widget, _5: c_uint, _4: Option<unsafe extern "C" fn (Widget, *mut Widget, *mut c_uint, *mut Widget, *mut c_uint, *mut c_void)>, _3: *mut c_void, _2: *mut Widget, _1: c_uint) -> (),
  pub fn XtClass (_1: Widget) -> WidgetClass,
  pub fn XtCloseDisplay (_1: *mut Display) -> (),
  pub fn XtConfigureWidget (_6: Widget, _5: c_short, _4: c_short, _3: c_ushort, _2: c_ushort, _1: c_ushort) -> (),
  pub fn XtConvert (_5: Widget, _4: *const c_char, _3: *mut XrmValue, _2: *const c_char, _1: *mut XrmValue) -> (),
  pub fn XtConvertAndStore (_5: Widget, _4: *const c_char, _3: *mut XrmValue, _2: *const c_char, _1: *mut XrmValue) -> c_char,
  pub fn XtConvertCase (_4: *mut Display, _3: c_ulong, _2: *mut c_ulong, _1: *mut c_ulong) -> (),
  pub fn XtCreateApplicationContext () -> XtAppContext,
  pub fn XtCreateApplicationShell (_4: *const c_char, _3: WidgetClass, _2: *mut Arg, _1: c_uint) -> Widget,
  pub fn XtCreateManagedWidget (_5: *const c_char, _4: WidgetClass, _3: Widget, _2: *mut Arg, _1: c_uint) -> Widget,
  pub fn XtCreatePopupShell (_5: *const c_char, _4: WidgetClass, _3: Widget, _2: *mut Arg, _1: c_uint) -> Widget,
  pub fn XtCreateSelectionRequest (_2: Widget, _1: c_ulong) -> (),
  pub fn XtCreateWidget (_5: *const c_char, _4: WidgetClass, _3: Widget, _2: *mut Arg, _1: c_uint) -> Widget,
  pub fn XtCreateWindow (_5: Widget, _4: c_uint, _3: *mut Visual, _2: c_ulong, _1: *mut XSetWindowAttributes) -> (),
  pub fn XtCvtColorToPixel (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtIntToBool (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtIntToBoolean (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtIntToColor (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtIntToFloat (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtIntToFont (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtIntToPixel (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtIntToPixmap (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtIntToShort (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtIntToUnsignedChar (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToAcceleratorTable (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToAtom (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToBool (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToBoolean (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToCommandArgArray (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToCursor (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToDimension (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToDirectoryString (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToDisplay (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToFile (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToFloat (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToFont (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToFontSet (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToFontStruct (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToGravity (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToInitialState (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToInt (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToPixel (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToRestartStyle (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToShort (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToTranslationTable (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToUnsignedChar (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtCvtStringToVisual (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XtDatabase (_1: *mut Display) -> *mut _XrmHashBucketRec,
  pub fn XtDestroyApplicationContext (_1: XtAppContext) -> (),
  pub fn XtDestroyGC (_1: GC) -> (),
  pub fn XtDestroyWidget (_1: Widget) -> (),
  pub fn XtDirectConvert (_5: Option<unsafe extern "C" fn (*mut XrmValue, *mut c_uint, *mut XrmValue, *mut XrmValue)>, _4: *mut XrmValue, _3: c_uint, _2: *mut XrmValue, _1: *mut XrmValue) -> (),
  pub fn XtDisownSelection (_3: Widget, _2: c_ulong, _1: c_ulong) -> (),
  pub fn XtDispatchEvent (_1: *mut XEvent) -> c_char,
  pub fn XtDispatchEventToWidget (_2: Widget, _1: *mut XEvent) -> c_char,
  pub fn XtDisplay (_1: Widget) -> *mut Display,
  pub fn XtDisplayInitialize (_8: XtAppContext, _7: *mut Display, _6: *const c_char, _5: *const c_char, _4: XrmOptionDescList, _3: c_uint, _2: *mut c_int, _1: *mut *mut c_char) -> (),
  pub fn XtDisplayOfObject (_1: Widget) -> *mut Display,
  pub fn XtDisplayStringConversionWarning (_3: *mut Display, _2: *const c_char, _1: *const c_char) -> (),
  pub fn XtDisplayToApplicationContext (_1: *mut Display) -> XtAppContext,
  pub fn XtError (_1: *const c_char) -> (),
  pub fn XtErrorMsg (_6: *const c_char, _5: *const c_char, _4: *const c_char, _3: *const c_char, _2: *mut *mut c_char, _1: *mut c_uint) -> (),
  pub fn XtFindFile (_4: *const c_char, _3: Substitution, _2: c_uint, _1: Option<unsafe extern "C" fn (*mut c_char) -> c_char>) -> *mut c_char,
  pub fn XtFree (_1: *mut c_char) -> (),
  pub fn XtGetActionKeysym (_2: *mut XEvent, _1: *mut c_uint) -> c_ulong,
  pub fn XtGetActionList (_3: WidgetClass, _2: *mut *mut XtActionsRec, _1: *mut c_uint) -> (),
  pub fn XtGetApplicationNameAndClass (_3: *mut Display, _2: *mut *mut c_char, _1: *mut *mut c_char) -> (),
  pub fn XtGetApplicationResources (_6: Widget, _5: *mut c_void, _4: *mut XtResource, _3: c_uint, _2: *mut Arg, _1: c_uint) -> (),
  pub fn XtGetClassExtension (_5: WidgetClass, _4: c_uint, _3: c_int, _2: c_long, _1: c_uint) -> *mut c_void,
  pub fn XtGetConstraintResourceList (_3: WidgetClass, _2: *mut *mut XtResource, _1: *mut c_uint) -> (),
  pub fn XtGetDisplays (_3: XtAppContext, _2: *mut *mut *mut Display, _1: *mut c_uint) -> (),
  pub fn XtGetErrorDatabase () -> *mut *mut _XrmHashBucketRec,
  pub fn XtGetErrorDatabaseText (_6: *const c_char, _5: *const c_char, _4: *const c_char, _3: *const c_char, _2: *mut c_char, _1: c_int) -> (),
  pub fn XtGetGC (_3: Widget, _2: c_ulong, _1: *mut XGCValues) -> GC,
  pub fn XtGetKeyboardFocusWidget (_1: Widget) -> Widget,
  pub fn XtGetKeysymTable (_3: *mut Display, _2: *mut c_uchar, _1: *mut c_int) -> *mut c_ulong,
  pub fn XtGetMultiClickTime (_1: *mut Display) -> c_int,
  pub fn XtGetResourceList (_3: WidgetClass, _2: *mut *mut XtResource, _1: *mut c_uint) -> (),
  pub fn XtGetSelectionParameters (_7: Widget, _6: c_ulong, _5: *mut c_void, _4: *mut c_ulong, _3: *mut *mut c_void, _2: *mut c_ulong, _1: *mut c_int) -> (),
  pub fn XtGetSelectionRequest (_3: Widget, _2: c_ulong, _1: *mut c_void) -> *mut XSelectionRequestEvent,
  pub fn XtGetSelectionTimeout () -> c_ulong,
  pub fn XtGetSelectionValue (_6: Widget, _5: c_ulong, _4: c_ulong, _3: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut c_ulong, *mut c_ulong, *mut c_void, *mut c_ulong, *mut c_int)>, _2: *mut c_void, _1: c_ulong) -> (),
  pub fn XtGetSelectionValueIncremental (_6: Widget, _5: c_ulong, _4: c_ulong, _3: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut c_ulong, *mut c_ulong, *mut c_void, *mut c_ulong, *mut c_int)>, _2: *mut c_void, _1: c_ulong) -> (),
  pub fn XtGetSelectionValues (_7: Widget, _6: c_ulong, _5: *mut c_ulong, _4: c_int, _3: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut c_ulong, *mut c_ulong, *mut c_void, *mut c_ulong, *mut c_int)>, _2: *mut *mut c_void, _1: c_ulong) -> (),
  pub fn XtGetSelectionValuesIncremental (_7: Widget, _6: c_ulong, _5: *mut c_ulong, _4: c_int, _3: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut c_ulong, *mut c_ulong, *mut c_void, *mut c_ulong, *mut c_int)>, _2: *mut *mut c_void, _1: c_ulong) -> (),
  pub fn XtGetSubresources (_8: Widget, _7: *mut c_void, _6: *const c_char, _5: *const c_char, _4: *mut XtResource, _3: c_uint, _2: *mut Arg, _1: c_uint) -> (),
  pub fn XtGetSubvalues (_5: *mut c_void, _4: *mut XtResource, _3: c_uint, _2: *mut Arg, _1: c_uint) -> (),
  pub fn XtGetValues (_3: Widget, _2: *mut Arg, _1: c_uint) -> (),
  pub fn XtGrabButton (_9: Widget, _8: c_int, _7: c_uint, _6: c_char, _5: c_uint, _4: c_int, _3: c_int, _2: c_ulong, _1: c_ulong) -> (),
  pub fn XtGrabKey (_6: Widget, _5: c_uchar, _4: c_uint, _3: c_char, _2: c_int, _1: c_int) -> (),
  pub fn XtGrabKeyboard (_5: Widget, _4: c_char, _3: c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XtGrabPointer (_8: Widget, _7: c_char, _6: c_uint, _5: c_int, _4: c_int, _3: c_ulong, _2: c_ulong, _1: c_ulong) -> c_int,
  pub fn XtHasCallbacks (_2: Widget, _1: *const c_char) -> XtCallbackStatus,
  pub fn XtHooksOfDisplay (_1: *mut Display) -> Widget,
  pub fn XtInitialize (_6: *const c_char, _5: *const c_char, _4: XrmOptionDescList, _3: c_uint, _2: *mut c_int, _1: *mut *mut c_char) -> Widget,
  pub fn XtInitializeWidgetClass (_1: WidgetClass) -> (),
  pub fn XtInsertEventHandler (_6: Widget, _5: c_ulong, _4: c_char, _3: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut XEvent, *mut c_char)>, _2: *mut c_void, _1: XtListPosition) -> (),
  pub fn XtInsertEventTypeHandler (_6: Widget, _5: c_int, _4: *mut c_void, _3: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut XEvent, *mut c_char)>, _2: *mut c_void, _1: XtListPosition) -> (),
  pub fn XtInsertRawEventHandler (_6: Widget, _5: c_ulong, _4: c_char, _3: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut XEvent, *mut c_char)>, _2: *mut c_void, _1: XtListPosition) -> (),
  pub fn XtInstallAccelerators (_2: Widget, _1: Widget) -> (),
  pub fn XtInstallAllAccelerators (_2: Widget, _1: Widget) -> (),
  pub fn XtIsApplicationShell (_1: Widget) -> c_char,
  pub fn XtIsComposite (_1: Widget) -> c_char,
  pub fn XtIsConstraint (_1: Widget) -> c_char,
  pub fn XtIsManaged (_1: Widget) -> c_char,
  pub fn XtIsObject (_1: Widget) -> c_char,
  pub fn XtIsOverrideShell (_1: Widget) -> c_char,
  pub fn XtIsRealized (_1: Widget) -> c_char,
  pub fn XtIsRectObj (_1: Widget) -> c_char,
  pub fn XtIsSensitive (_1: Widget) -> c_char,
  pub fn XtIsSessionShell (_1: Widget) -> c_char,
  pub fn XtIsShell (_1: Widget) -> c_char,
  pub fn XtIsSubclass (_2: Widget, _1: WidgetClass) -> c_char,
  pub fn XtIsTopLevelShell (_1: Widget) -> c_char,
  pub fn XtIsTransientShell (_1: Widget) -> c_char,
  pub fn XtIsVendorShell (_1: Widget) -> c_char,
  pub fn XtIsWidget (_1: Widget) -> c_char,
  pub fn XtIsWMShell (_1: Widget) -> c_char,
  pub fn XtKeysymToKeycodeList (_4: *mut Display, _3: c_ulong, _2: *mut *mut c_uchar, _1: *mut c_uint) -> (),
  pub fn XtLastEventProcessed (_1: *mut Display) -> *mut XEvent,
  pub fn XtLastTimestampProcessed (_1: *mut Display) -> c_ulong,
  pub fn XtMainLoop () -> (),
  pub fn XtMakeGeometryRequest (_3: Widget, _2: *mut XtWidgetGeometry, _1: *mut XtWidgetGeometry) -> XtGeometryResult,
  pub fn XtMakeResizeRequest (_5: Widget, _4: c_ushort, _3: c_ushort, _2: *mut c_ushort, _1: *mut c_ushort) -> XtGeometryResult,
  pub fn XtMalloc (_1: c_uint) -> *mut c_char,
  pub fn XtManageChild (_1: Widget) -> (),
  pub fn XtManageChildren (_2: *mut Widget, _1: c_uint) -> (),
  pub fn XtMapWidget (_1: Widget) -> (),
  pub fn XtMenuPopupAction (_4: Widget, _3: *mut XEvent, _2: *mut *mut c_char, _1: *mut c_uint) -> (),
  pub fn XtMergeArgLists (_4: *mut Arg, _3: c_uint, _2: *mut Arg, _1: c_uint) -> *mut Arg,
  pub fn XtMoveWidget (_3: Widget, _2: c_short, _1: c_short) -> (),
  pub fn XtName (_1: Widget) -> *mut c_char,
  pub fn XtNameToWidget (_2: Widget, _1: *const c_char) -> Widget,
  pub fn XtNewString (_1: *mut c_char) -> *mut c_char,
  pub fn XtNextEvent (_1: *mut XEvent) -> (),
  pub fn XtNoticeSignal (_1: c_ulong) -> (),
  pub fn XtOpenApplication (_10: *mut XtAppContext, _9: *const c_char, _8: XrmOptionDescList, _7: c_uint, _6: *mut c_int, _5: *mut *mut c_char, _4: *mut *mut c_char, _3: WidgetClass, _2: *mut Arg, _1: c_uint) -> Widget,
  pub fn XtOpenDisplay (_8: XtAppContext, _7: *const c_char, _6: *const c_char, _5: *const c_char, _4: XrmOptionDescList, _3: c_uint, _2: *mut c_int, _1: *mut *mut c_char) -> *mut Display,
  pub fn XtOverrideTranslations (_2: Widget, _1: *mut _TranslationData) -> (),
  pub fn XtOwnSelection (_6: Widget, _5: c_ulong, _4: c_ulong, _3: Option<unsafe extern "C" fn (Widget, *mut c_ulong, *mut c_ulong, *mut c_ulong, *mut *mut c_void, *mut c_ulong, *mut c_int) -> c_char>, _2: Option<unsafe extern "C" fn (Widget, *mut c_ulong)>, _1: Option<unsafe extern "C" fn (Widget, *mut c_ulong, *mut c_ulong)>) -> c_char,
  pub fn XtOwnSelectionIncremental (_8: Widget, _7: c_ulong, _6: c_ulong, _5: Option<unsafe extern "C" fn (Widget, *mut c_ulong, *mut c_ulong, *mut c_ulong, *mut *mut c_void, *mut c_ulong, *mut c_int, *mut c_ulong, *mut c_void, *mut *mut c_void) -> c_char>, _4: Option<unsafe extern "C" fn (Widget, *mut c_ulong, *mut c_void)>, _3: Option<unsafe extern "C" fn (Widget, *mut c_ulong, *mut c_ulong, *mut *mut c_void, *mut c_void)>, _2: Option<unsafe extern "C" fn (Widget, *mut c_ulong, *mut c_ulong, *mut *mut c_void, *mut c_void)>, _1: *mut c_void) -> c_char,
  pub fn XtParent (_1: Widget) -> Widget,
  pub fn XtParseAcceleratorTable (_1: *const c_char) -> *mut _TranslationData,
  pub fn XtParseTranslationTable (_1: *const c_char) -> *mut _TranslationData,
  pub fn XtPeekEvent (_1: *mut XEvent) -> c_char,
  pub fn XtPending () -> c_char,
  pub fn XtPopdown (_1: Widget) -> (),
  pub fn XtPopup (_2: Widget, _1: XtGrabKind) -> (),
  pub fn XtPopupSpringLoaded (_1: Widget) -> (),
  pub fn XtProcessEvent (_1: c_ulong) -> (),
  pub fn XtProcessLock () -> (),
  pub fn XtProcessUnlock () -> (),
  pub fn XtQueryGeometry (_3: Widget, _2: *mut XtWidgetGeometry, _1: *mut XtWidgetGeometry) -> XtGeometryResult,
  pub fn XtRealizeWidget (_1: Widget) -> (),
  pub fn XtRealloc (_2: *mut c_char, _1: c_uint) -> *mut c_char,
  pub fn XtRegisterCaseConverter (_4: *mut Display, _3: Option<unsafe extern "C" fn (*mut Display, c_ulong, *mut c_ulong, *mut c_ulong)>, _2: c_ulong, _1: c_ulong) -> (),
  pub fn XtRegisterDrawable (_3: *mut Display, _2: c_ulong, _1: Widget) -> (),
  pub fn XtRegisterExtensionSelector (_5: *mut Display, _4: c_int, _3: c_int, _2: Option<unsafe extern "C" fn (Widget, *mut c_int, *mut *mut c_void, c_int, *mut c_void)>, _1: *mut c_void) -> (),
  pub fn XtRegisterGrabAction (_5: Option<unsafe extern "C" fn (Widget, *mut XEvent, *mut *mut c_char, *mut c_uint)>, _4: c_char, _3: c_uint, _2: c_int, _1: c_int) -> (),
  pub fn XtReleaseGC (_2: Widget, _1: GC) -> (),
  pub fn XtReleasePropertyAtom (_2: Widget, _1: c_ulong) -> (),
  pub fn XtRemoveActionHook (_1: *mut c_void) -> (),
  pub fn XtRemoveAllCallbacks (_2: Widget, _1: *const c_char) -> (),
  pub fn XtRemoveBlockHook (_1: c_ulong) -> (),
  pub fn XtRemoveCallback (_4: Widget, _3: *const c_char, _2: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut c_void)>, _1: *mut c_void) -> (),
  pub fn XtRemoveCallbacks (_3: Widget, _2: *const c_char, _1: XtCallbackList) -> (),
  pub fn XtRemoveEventHandler (_5: Widget, _4: c_ulong, _3: c_char, _2: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut XEvent, *mut c_char)>, _1: *mut c_void) -> (),
  pub fn XtRemoveEventTypeHandler (_5: Widget, _4: c_int, _3: *mut c_void, _2: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut XEvent, *mut c_char)>, _1: *mut c_void) -> (),
  pub fn XtRemoveGrab (_1: Widget) -> (),
  pub fn XtRemoveInput (_1: c_ulong) -> (),
  pub fn XtRemoveRawEventHandler (_5: Widget, _4: c_ulong, _3: c_char, _2: Option<unsafe extern "C" fn (Widget, *mut c_void, *mut XEvent, *mut c_char)>, _1: *mut c_void) -> (),
  pub fn XtRemoveSignal (_1: c_ulong) -> (),
  pub fn XtRemoveTimeOut (_1: c_ulong) -> (),
  pub fn XtRemoveWorkProc (_1: c_ulong) -> (),
  pub fn XtReservePropertyAtom (_1: Widget) -> c_ulong,
  pub fn XtResizeWidget (_4: Widget, _3: c_ushort, _2: c_ushort, _1: c_ushort) -> (),
  pub fn XtResizeWindow (_1: Widget) -> (),
  pub fn XtResolvePathname (_8: *mut Display, _7: *const c_char, _6: *const c_char, _5: *const c_char, _4: *const c_char, _3: Substitution, _2: c_uint, _1: Option<unsafe extern "C" fn (*mut c_char) -> c_char>) -> *mut c_char,
  pub fn XtScreen (_1: Widget) -> *mut Screen,
  pub fn XtScreenDatabase (_1: *mut Screen) -> *mut _XrmHashBucketRec,
  pub fn XtScreenOfObject (_1: Widget) -> *mut Screen,
  pub fn XtSendSelectionRequest (_3: Widget, _2: c_ulong, _1: c_ulong) -> (),
  pub fn XtSessionGetToken (_1: Widget) -> XtCheckpointToken,
  pub fn XtSessionReturnToken (_1: XtCheckpointToken) -> (),
  pub fn XtSetErrorHandler (_1: Option<unsafe extern "C" fn (*mut c_char)>) -> (),
  pub fn XtSetErrorMsgHandler (_1: Option<unsafe extern "C" fn (*mut c_char, *mut c_char, *mut c_char, *mut c_char, *mut *mut c_char, *mut c_uint)>) -> (),
  pub fn XtSetEventDispatcher (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut XEvent) -> c_char>) -> Option<unsafe extern "C" fn (*mut XEvent) -> c_char>,
  pub fn XtSetKeyboardFocus (_2: Widget, _1: Widget) -> (),
  pub fn XtSetKeyTranslator (_2: *mut Display, _1: Option<unsafe extern "C" fn (*mut Display, c_uchar, c_uint, *mut c_uint, *mut c_ulong)>) -> (),
  pub fn XtSetLanguageProc (_3: XtAppContext, _2: Option<unsafe extern "C" fn (*mut Display, *mut c_char, *mut c_void) -> *mut c_char>, _1: *mut c_void) -> Option<unsafe extern "C" fn (*mut Display, *mut c_char, *mut c_void) -> *mut c_char>,
  pub fn XtSetMappedWhenManaged (_2: Widget, _1: c_char) -> (),
  pub fn XtSetMultiClickTime (_2: *mut Display, _1: c_int) -> (),
  pub fn XtSetSelectionParameters (_6: Widget, _5: c_ulong, _4: c_ulong, _3: *mut c_void, _2: c_ulong, _1: c_int) -> (),
  pub fn XtSetSelectionTimeout (_1: c_ulong) -> (),
  pub fn XtSetSensitive (_2: Widget, _1: c_char) -> (),
  pub fn XtSetSubvalues (_5: *mut c_void, _4: *mut XtResource, _3: c_uint, _2: *mut Arg, _1: c_uint) -> (),
  pub fn XtSetTypeConverter (_7: *const c_char, _6: *const c_char, _5: Option<unsafe extern "C" fn (*mut Display, *mut XrmValue, *mut c_uint, *mut XrmValue, *mut XrmValue, *mut *mut c_void) -> c_char>, _4: XtConvertArgList, _3: c_uint, _2: c_int, _1: Option<unsafe extern "C" fn (XtAppContext, *mut XrmValue, *mut c_void, *mut XrmValue, *mut c_uint)>) -> (),
  pub fn XtSetValues (_3: Widget, _2: *mut Arg, _1: c_uint) -> (),
  pub fn XtSetWarningHandler (_1: Option<unsafe extern "C" fn (*mut c_char)>) -> (),
  pub fn XtSetWarningMsgHandler (_1: Option<unsafe extern "C" fn (*mut c_char, *mut c_char, *mut c_char, *mut c_char, *mut *mut c_char, *mut c_uint)>) -> (),
  pub fn XtSetWMColormapWindows (_3: Widget, _2: *mut Widget, _1: c_uint) -> (),
  pub fn XtStringConversionWarning (_2: *const c_char, _1: *const c_char) -> (),
  pub fn XtSuperclass (_1: Widget) -> WidgetClass,
  pub fn XtToolkitInitialize () -> (),
  pub fn XtToolkitThreadInitialize () -> c_char,
  pub fn XtTranslateCoords (_5: Widget, _4: c_short, _3: c_short, _2: *mut c_short, _1: *mut c_short) -> (),
  pub fn XtTranslateKey (_5: *mut Display, _4: c_uchar, _3: c_uint, _2: *mut c_uint, _1: *mut c_ulong) -> (),
  pub fn XtTranslateKeycode (_5: *mut Display, _4: c_uchar, _3: c_uint, _2: *mut c_uint, _1: *mut c_ulong) -> (),
  pub fn XtUngrabButton (_3: Widget, _2: c_uint, _1: c_uint) -> (),
  pub fn XtUngrabKey (_3: Widget, _2: c_uchar, _1: c_uint) -> (),
  pub fn XtUngrabKeyboard (_2: Widget, _1: c_ulong) -> (),
  pub fn XtUngrabPointer (_2: Widget, _1: c_ulong) -> (),
  pub fn XtUninstallTranslations (_1: Widget) -> (),
  pub fn XtUnmanageChild (_1: Widget) -> (),
  pub fn XtUnmanageChildren (_2: *mut Widget, _1: c_uint) -> (),
  pub fn XtUnmapWidget (_1: Widget) -> (),
  pub fn XtUnrealizeWidget (_1: Widget) -> (),
  pub fn XtUnregisterDrawable (_2: *mut Display, _1: c_ulong) -> (),
  pub fn XtWarning (_1: *const c_char) -> (),
  pub fn XtWarningMsg (_6: *const c_char, _5: *const c_char, _4: *const c_char, _3: *const c_char, _2: *mut *mut c_char, _1: *mut c_uint) -> (),
  pub fn XtWidgetToApplicationContext (_1: Widget) -> XtAppContext,
  pub fn XtWindow (_1: Widget) -> c_ulong,
  pub fn XtWindowOfObject (_1: Widget) -> c_ulong,
  pub fn XtWindowToWidget (_2: *mut Display, _1: c_ulong) -> Widget,
variadic:
  pub fn XtAsprintf (_2: *mut *mut c_char, _1: *const c_char) -> c_uint,
  pub fn XtVaAppCreateShell (_4: *const c_char, _3: *const c_char, _2: WidgetClass, _1: *mut Display) -> Widget,
  pub fn XtVaAppInitialize (_7: *mut XtAppContext, _6: *const c_char, _5: XrmOptionDescList, _4: c_uint, _3: *mut c_int, _2: *mut *mut c_char, _1: *mut *mut c_char) -> Widget,
  pub fn XtVaCreateArgsList (_1: *mut c_void) -> *mut c_void,
  pub fn XtVaCreateManagedWidget (_3: *const c_char, _2: WidgetClass, _1: Widget) -> Widget,
  pub fn XtVaCreatePopupShell (_3: *const c_char, _2: WidgetClass, _1: Widget) -> Widget,
  pub fn XtVaCreateWidget (_3: *const c_char, _2: WidgetClass, _1: Widget) -> Widget,
  pub fn XtVaGetApplicationResources (_4: Widget, _3: *mut c_void, _2: *mut XtResource, _1: c_uint) -> (),
  pub fn XtVaGetSubresources (_6: Widget, _5: *mut c_void, _4: *const c_char, _3: *const c_char, _2: *mut XtResource, _1: c_uint) -> (),
  pub fn XtVaGetSubvalues (_3: *mut c_void, _2: *mut XtResource, _1: c_uint) -> (),
  pub fn XtVaGetValues (_1: Widget) -> (),
  pub fn XtVaOpenApplication (_8: *mut XtAppContext, _7: *const c_char, _6: XrmOptionDescList, _5: c_uint, _4: *mut c_int, _3: *mut *mut c_char, _2: *mut *mut c_char, _1: WidgetClass) -> Widget,
  pub fn XtVaSetSubvalues (_3: *mut c_void, _2: *mut XtResource, _1: c_uint) -> (),
  pub fn XtVaSetValues (_1: Widget) -> (),
globals:
}

//
// types
//

// TODO structs
#[repr(C)]
pub struct Arg;
#[repr(C)]
pub struct SubstitutionRec;
#[repr(C)]
pub struct _TranslationData;
#[repr(C)]
pub struct _WidgetClassRec;
#[repr(C)]
pub struct _WidgetRec;
#[repr(C)]
pub struct _XtActionsRec;
#[repr(C)]
pub struct _XtAppStruct;
#[repr(C)]
pub struct _XtCallbackRec;
#[repr(C)]
pub struct _XtCheckpointTokenRec;
#[repr(C)]
pub struct XtConvertArgRec;
#[repr(C)]
pub struct _XtResource;
#[repr(C)]
pub struct XtWidgetGeometry;

// C enums
pub type XtCallbackStatus = c_int;
pub type XtGeometryResult = c_int;
pub type XtGrabKind = c_int;
pub type XtListPosition = c_int;

#[allow(dead_code)]
#[cfg(test)]
#[repr(C)]
enum TestEnum {
    Variant1,
    Variant2,
}

#[test]
fn enum_size_test() {
    assert!(::std::mem::size_of::<TestEnum>() == ::std::mem::size_of::<c_int>());
}

// struct typedefs
pub type ArgList = *mut Arg;
pub type Substitution = *mut SubstitutionRec;
pub type Widget = *mut _WidgetRec;
pub type WidgetClass = *mut _WidgetClassRec;
pub type XtAccelerators = *mut _TranslationData;
pub type XtActionList = *mut _XtActionsRec;
pub type XtActionsRec = _XtActionsRec;
pub type XtAppContext = *mut _XtAppStruct;
pub type XtCallbackList = *mut _XtCallbackRec;
pub type XtCallbackRec = _XtCallbackRec;
pub type XtCheckpointToken = *mut _XtCheckpointTokenRec;
pub type XtCheckpointTokenRec = _XtCheckpointTokenRec;
pub type XtConvertArgList = *mut XtConvertArgRec;
pub type XtResource = _XtResource;
pub type XtResourceList = *mut _XtResource;
pub type XtTranslations = *mut _TranslationData;
