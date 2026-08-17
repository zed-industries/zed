// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::FILE;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};

use super::xlib::{
    Display, Screen, XColor, XComposeStatus, XErrorEvent, XEvent, XKeyEvent, XSizeHints,
    XStandardColormap, XVisualInfo, XrmValue, GC,
};
use super::xt::{Widget, XtAppContext};

//
// functions
//

x11_link! { Xmu, xmu, ["libXmu.so.6", "libXmu.so"], 132,
  pub fn XmuAddCloseDisplayHook (_3: *mut Display, _2: Option<unsafe extern "C" fn (*mut Display, *mut c_char) -> c_int>, _1: *mut c_char) -> *mut c_char,
  pub fn XmuAddInitializer (_2: Option<unsafe extern "C" fn (XtAppContext, *mut c_char)>, _1: *mut c_char) -> (),
  pub fn XmuAllStandardColormaps (_1: *mut Display) -> c_int,
  pub fn XmuAppendSegment (_2: *mut XmuSegment, _1: *mut XmuSegment) -> c_int,
  pub fn XmuAreaAnd (_2: *mut XmuArea, _1: *mut XmuArea) -> *mut XmuArea,
  pub fn XmuAreaCopy (_2: *mut XmuArea, _1: *mut XmuArea) -> *mut XmuArea,
  pub fn XmuAreaDup (_1: *mut XmuArea) -> *mut XmuArea,
  pub fn XmuAreaNot (_5: *mut XmuArea, _4: c_int, _3: c_int, _2: c_int, _1: c_int) -> *mut XmuArea,
  pub fn XmuAreaOrXor (_3: *mut XmuArea, _2: *mut XmuArea, _1: c_int) -> *mut XmuArea,
  pub fn XmuCallInitializers (_1: XtAppContext) -> (),
  pub fn XmuClientWindow (_2: *mut Display, _1: c_ulong) -> c_ulong,
  pub fn XmuCompareISOLatin1 (_2: *const c_char, _1: *const c_char) -> c_int,
  pub fn XmuConvertStandardSelection (_8: Widget, _7: c_ulong, _6: *mut c_ulong, _5: *mut c_ulong, _4: *mut c_ulong, _3: *mut *mut c_char, _2: *mut c_ulong, _1: *mut c_int) -> c_char,
  pub fn XmuCopyISOLatin1Lowered (_2: *mut c_char, _1: *const c_char) -> (),
  pub fn XmuCopyISOLatin1Uppered (_2: *mut c_char, _1: *const c_char) -> (),
  pub fn XmuCreateColormap (_2: *mut Display, _1: *mut XStandardColormap) -> c_int,
  pub fn XmuCreatePixmapFromBitmap (_8: *mut Display, _7: c_ulong, _6: c_ulong, _5: c_uint, _4: c_uint, _3: c_uint, _2: c_ulong, _1: c_ulong) -> c_ulong,
  pub fn XmuCreateStippledPixmap (_4: *mut Screen, _3: c_ulong, _2: c_ulong, _1: c_uint) -> c_ulong,
  pub fn XmuCursorNameToIndex (_1: *const c_char) -> c_int,
  pub fn XmuCvtBackingStoreToString (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XmuCvtFunctionToCallback (_4: *mut XrmValue, _3: *mut c_uint, _2: *mut XrmValue, _1: *mut XrmValue) -> (),
  pub fn XmuCvtGravityToString (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XmuCvtJustifyToString (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XmuCvtLongToString (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XmuCvtOrientationToString (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XmuCvtShapeStyleToString (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XmuCvtStringToBackingStore (_4: *mut XrmValue, _3: *mut c_uint, _2: *mut XrmValue, _1: *mut XrmValue) -> (),
  pub fn XmuCvtStringToBitmap (_4: *mut XrmValue, _3: *mut c_uint, _2: *mut XrmValue, _1: *mut XrmValue) -> (),
  pub fn XmuCvtStringToColorCursor (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XmuCvtStringToCursor (_4: *mut XrmValue, _3: *mut c_uint, _2: *mut XrmValue, _1: *mut XrmValue) -> (),
  pub fn XmuCvtStringToGravity (_4: *mut XrmValue, _3: *mut c_uint, _2: *mut XrmValue, _1: *mut XrmValue) -> (),
  pub fn XmuCvtStringToJustify (_4: *mut XrmValue, _3: *mut c_uint, _2: *mut XrmValue, _1: *mut XrmValue) -> (),
  pub fn XmuCvtStringToLong (_4: *mut XrmValue, _3: *mut c_uint, _2: *mut XrmValue, _1: *mut XrmValue) -> (),
  pub fn XmuCvtStringToOrientation (_4: *mut XrmValue, _3: *mut c_uint, _2: *mut XrmValue, _1: *mut XrmValue) -> (),
  pub fn XmuCvtStringToShapeStyle (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XmuCvtStringToWidget (_4: *mut XrmValue, _3: *mut c_uint, _2: *mut XrmValue, _1: *mut XrmValue) -> (),
  pub fn XmuCvtWidgetToString (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XmuDeleteStandardColormap (_3: *mut Display, _2: c_int, _1: c_ulong) -> (),
  pub fn XmuDestroyScanlineList (_1: *mut XmuScanline) -> (),
  pub fn XmuDestroySegmentList (_1: *mut XmuSegment) -> (),
  pub fn XmuDistinguishableColors (_2: *mut XColor, _1: c_int) -> c_int,
  pub fn XmuDistinguishablePixels (_4: *mut Display, _3: c_ulong, _2: *mut c_ulong, _1: c_int) -> c_int,
  pub fn XmuDQAddDisplay (_3: *mut XmuDisplayQueue, _2: *mut Display, _1: *mut c_char) -> *mut XmuDisplayQueueEntry,
  pub fn XmuDQCreate (_3: Option<unsafe extern "C" fn (*mut XmuDisplayQueue, *mut XmuDisplayQueueEntry) -> c_int>, _2: Option<unsafe extern "C" fn (*mut XmuDisplayQueue) -> c_int>, _1: *mut c_char) -> *mut XmuDisplayQueue,
  pub fn XmuDQDestroy (_2: *mut XmuDisplayQueue, _1: c_int) -> c_int,
  pub fn XmuDQLookupDisplay (_2: *mut XmuDisplayQueue, _1: *mut Display) -> *mut XmuDisplayQueueEntry,
  pub fn XmuDQRemoveDisplay (_2: *mut XmuDisplayQueue, _1: *mut Display) -> c_int,
  pub fn XmuDrawLogo (_8: *mut Display, _7: c_ulong, _6: GC, _5: GC, _4: c_int, _3: c_int, _2: c_uint, _1: c_uint) -> (),
  pub fn XmuDrawRoundedRectangle (_9: *mut Display, _8: c_ulong, _7: GC, _6: c_int, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: c_int) -> (),
  pub fn XmuFillRoundedRectangle (_9: *mut Display, _8: c_ulong, _7: GC, _6: c_int, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: c_int) -> (),
  pub fn XmuGetAtomName (_2: *mut Display, _1: c_ulong) -> *mut c_char,
  pub fn XmuGetColormapAllocation (_5: *mut XVisualInfo, _4: c_ulong, _3: *mut c_ulong, _2: *mut c_ulong, _1: *mut c_ulong) -> c_int,
  pub fn XmuGetHostname (_2: *mut c_char, _1: c_int) -> c_int,
  pub fn XmuInternAtom (_2: *mut Display, _1: AtomPtr) -> c_ulong,
  pub fn XmuInternStrings (_4: *mut Display, _3: *mut *mut c_char, _2: c_uint, _1: *mut c_ulong) -> (),
  pub fn XmuLocateBitmapFile (_8: *mut Screen, _7: *const c_char, _6: *mut c_char, _5: c_int, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut c_int) -> c_ulong,
  pub fn XmuLocatePixmapFile (_11: *mut Screen, _10: *const c_char, _9: c_ulong, _8: c_ulong, _7: c_uint, _6: *mut c_char, _5: c_int, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut c_int) -> c_ulong,
  pub fn XmuLookupAPL (_5: *mut XKeyEvent, _4: *mut c_uchar, _3: c_int, _2: *mut c_ulong, _1: *mut XComposeStatus) -> c_int,
  pub fn XmuLookupArabic (_5: *mut XKeyEvent, _4: *mut c_uchar, _3: c_int, _2: *mut c_ulong, _1: *mut XComposeStatus) -> c_int,
  pub fn XmuLookupCloseDisplayHook (_4: *mut Display, _3: *mut c_char, _2: Option<unsafe extern "C" fn (*mut Display, *mut c_char) -> c_int>, _1: *mut c_char) -> c_int,
  pub fn XmuLookupCyrillic (_5: *mut XKeyEvent, _4: *mut c_uchar, _3: c_int, _2: *mut c_ulong, _1: *mut XComposeStatus) -> c_int,
  pub fn XmuLookupGreek (_5: *mut XKeyEvent, _4: *mut c_uchar, _3: c_int, _2: *mut c_ulong, _1: *mut XComposeStatus) -> c_int,
  pub fn XmuLookupHebrew (_5: *mut XKeyEvent, _4: *mut c_uchar, _3: c_int, _2: *mut c_ulong, _1: *mut XComposeStatus) -> c_int,
  pub fn XmuLookupJISX0201 (_5: *mut XKeyEvent, _4: *mut c_uchar, _3: c_int, _2: *mut c_ulong, _1: *mut XComposeStatus) -> c_int,
  pub fn XmuLookupKana (_5: *mut XKeyEvent, _4: *mut c_uchar, _3: c_int, _2: *mut c_ulong, _1: *mut XComposeStatus) -> c_int,
  pub fn XmuLookupLatin1 (_5: *mut XKeyEvent, _4: *mut c_uchar, _3: c_int, _2: *mut c_ulong, _1: *mut XComposeStatus) -> c_int,
  pub fn XmuLookupLatin2 (_5: *mut XKeyEvent, _4: *mut c_uchar, _3: c_int, _2: *mut c_ulong, _1: *mut XComposeStatus) -> c_int,
  pub fn XmuLookupLatin3 (_5: *mut XKeyEvent, _4: *mut c_uchar, _3: c_int, _2: *mut c_ulong, _1: *mut XComposeStatus) -> c_int,
  pub fn XmuLookupLatin4 (_5: *mut XKeyEvent, _4: *mut c_uchar, _3: c_int, _2: *mut c_ulong, _1: *mut XComposeStatus) -> c_int,
  pub fn XmuLookupStandardColormap (_7: *mut Display, _6: c_int, _5: c_ulong, _4: c_uint, _3: c_ulong, _2: c_int, _1: c_int) -> c_int,
  pub fn XmuLookupString (_6: *mut XKeyEvent, _5: *mut c_uchar, _4: c_int, _3: *mut c_ulong, _2: *mut XComposeStatus, _1: c_ulong) -> c_int,
  pub fn XmuMakeAtom (_1: *const c_char) -> AtomPtr,
  pub fn XmuNameOfAtom (_1: AtomPtr) -> *mut c_char,
  pub fn XmuNCopyISOLatin1Lowered (_3: *mut c_char, _2: *const c_char, _1: c_int) -> (),
  pub fn XmuNCopyISOLatin1Uppered (_3: *mut c_char, _2: *const c_char, _1: c_int) -> (),
  pub fn XmuNewArea (_4: c_int, _3: c_int, _2: c_int, _1: c_int) -> *mut XmuArea,
  pub fn XmuNewCvtStringToWidget (_6: *mut Display, _5: *mut XrmValue, _4: *mut c_uint, _3: *mut XrmValue, _2: *mut XrmValue, _1: *mut *mut c_void) -> c_char,
  pub fn XmuNewScanline (_3: c_int, _2: c_int, _1: c_int) -> *mut XmuScanline,
  pub fn XmuNewSegment (_2: c_int, _1: c_int) -> *mut XmuSegment,
  pub fn XmuOptimizeArea (_1: *mut XmuArea) -> *mut XmuArea,
  pub fn XmuOptimizeScanline (_1: *mut XmuScanline) -> *mut XmuScanline,
  pub fn XmuPrintDefaultErrorMessage (_3: *mut Display, _2: *mut XErrorEvent, _1: *mut FILE) -> c_int,
  pub fn XmuReadBitmapData (_6: *mut FILE, _5: *mut c_uint, _4: *mut c_uint, _3: *mut *mut c_uchar, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn XmuReadBitmapDataFromFile (_6: *const c_char, _5: *mut c_uint, _4: *mut c_uint, _3: *mut *mut c_uchar, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn XmuRegisterExternalAgent (_4: Widget, _3: *mut c_void, _2: *mut XEvent, _1: *mut c_char) -> (),
  pub fn XmuReleaseStippledPixmap (_2: *mut Screen, _1: c_ulong) -> (),
  pub fn XmuRemoveCloseDisplayHook (_4: *mut Display, _3: *mut c_char, _2: Option<unsafe extern "C" fn (*mut Display, *mut c_char) -> c_int>, _1: *mut c_char) -> c_int,
  pub fn XmuReshapeWidget (_4: Widget, _3: c_int, _2: c_int, _1: c_int) -> c_char,
  pub fn XmuScanlineAnd (_2: *mut XmuScanline, _1: *mut XmuScanline) -> *mut XmuScanline,
  pub fn XmuScanlineAndSegment (_2: *mut XmuScanline, _1: *mut XmuSegment) -> *mut XmuScanline,
  pub fn XmuScanlineCopy (_2: *mut XmuScanline, _1: *mut XmuScanline) -> *mut XmuScanline,
  pub fn XmuScanlineEqu (_2: *mut XmuScanline, _1: *mut XmuScanline) -> c_int,
  pub fn XmuScanlineNot (_3: *mut XmuScanline, _2: c_int, _1: c_int) -> *mut XmuScanline,
  pub fn XmuScanlineOr (_2: *mut XmuScanline, _1: *mut XmuScanline) -> *mut XmuScanline,
  pub fn XmuScanlineOrSegment (_2: *mut XmuScanline, _1: *mut XmuSegment) -> *mut XmuScanline,
  pub fn XmuScanlineXor (_2: *mut XmuScanline, _1: *mut XmuScanline) -> *mut XmuScanline,
  pub fn XmuScanlineXorSegment (_2: *mut XmuScanline, _1: *mut XmuSegment) -> *mut XmuScanline,
  pub fn XmuScreenOfWindow (_2: *mut Display, _1: c_ulong) -> *mut Screen,
  pub fn XmuSimpleErrorHandler (_2: *mut Display, _1: *mut XErrorEvent) -> c_int,
  pub fn XmuStandardColormap (_9: *mut Display, _8: c_int, _7: c_ulong, _6: c_uint, _5: c_ulong, _4: c_ulong, _3: c_ulong, _2: c_ulong, _1: c_ulong) -> *mut XStandardColormap,
  pub fn XmuUpdateMapHints (_3: *mut Display, _2: c_ulong, _1: *mut XSizeHints) -> c_int,
  pub fn XmuValidArea (_1: *mut XmuArea) -> c_int,
  pub fn XmuValidScanline (_1: *mut XmuScanline) -> c_int,
  pub fn XmuVisualStandardColormaps (_6: *mut Display, _5: c_int, _4: c_ulong, _3: c_uint, _2: c_int, _1: c_int) -> c_int,
  pub fn XmuWnCountOwnedResources (_3: *mut XmuWidgetNode, _2: *mut XmuWidgetNode, _1: c_int) -> c_int,
  pub fn XmuWnFetchResources (_3: *mut XmuWidgetNode, _2: Widget, _1: *mut XmuWidgetNode) -> (),
  pub fn XmuWnInitializeNodes (_2: *mut XmuWidgetNode, _1: c_int) -> (),
  pub fn XmuWnNameToNode (_3: *mut XmuWidgetNode, _2: c_int, _1: *const c_char) -> *mut XmuWidgetNode,
variadic:
  pub fn XmuSnprintf (_3: *mut c_char, _2: c_int, _1: *const c_char) -> c_int,
globals:
  pub static _XA_ATOM_PAIR: AtomPtr,
  pub static _XA_CHARACTER_POSITION: AtomPtr,
  pub static _XA_CLASS: AtomPtr,
  pub static _XA_CLIENT_WINDOW: AtomPtr,
  pub static _XA_CLIPBOARD: AtomPtr,
  pub static _XA_COMPOUND_TEXT: AtomPtr,
  pub static _XA_DECNET_ADDRESS: AtomPtr,
  pub static _XA_DELETE: AtomPtr,
  pub static _XA_FILENAME: AtomPtr,
  pub static _XA_HOSTNAME: AtomPtr,
  pub static _XA_IP_ADDRESS: AtomPtr,
  pub static _XA_LENGTH: AtomPtr,
  pub static _XA_LIST_LENGTH: AtomPtr,
  pub static _XA_NAME: AtomPtr,
  pub static _XA_NET_ADDRESS: AtomPtr,
  pub static _XA_NULL: AtomPtr,
  pub static _XA_OWNER_OS: AtomPtr,
  pub static _XA_SPAN: AtomPtr,
  pub static _XA_TARGETS: AtomPtr,
  pub static _XA_TEXT: AtomPtr,
  pub static _XA_TIMESTAMP: AtomPtr,
  pub static _XA_USER: AtomPtr,
  pub static _XA_UTF8_STRING: AtomPtr,
}

//
// types
//

// TODO structs
#[repr(C)]
pub struct _AtomRec;
#[repr(C)]
pub struct _XmuArea;
#[repr(C)]
pub struct _XmuDisplayQueue;
#[repr(C)]
pub struct _XmuDisplayQueueEntry;
#[repr(C)]
pub struct _XmuScanline;
#[repr(C)]
pub struct _XmuSegment;
#[repr(C)]
pub struct _XmuWidgetNode;

// struct typedefs
pub type AtomPtr = *mut _AtomRec;
pub type XmuArea = _XmuArea;
pub type XmuDisplayQueue = _XmuDisplayQueue;
pub type XmuDisplayQueueEntry = _XmuDisplayQueueEntry;
pub type XmuScanline = _XmuScanline;
pub type XmuSegment = _XmuSegment;
pub type XmuWidgetNode = _XmuWidgetNode;
