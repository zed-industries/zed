// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::FILE;
use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};

use crate::xlib::{Cursor, Display, XColor, XImage};

//
// functions
//

x11_link! { Xcursor, xcursor, ["libXcursor.so.1", "libXcursor.so"], 59,
  pub fn XcursorAnimateCreate (_1: *mut XcursorCursors) -> *mut XcursorAnimate,
  pub fn XcursorAnimateDestroy (_1: *mut XcursorAnimate) -> (),
  pub fn XcursorAnimateNext (_1: *mut XcursorAnimate) -> c_ulong,
  pub fn XcursorCommentCreate (_2: c_uint, _1: c_int) -> *mut XcursorComment,
  pub fn XcursorCommentDestroy (_1: *mut XcursorComment) -> (),
  pub fn XcursorCommentsCreate (_1: c_int) -> *mut XcursorComments,
  pub fn XcursorCommentsDestroy (_1: *mut XcursorComments) -> (),
  pub fn XcursorCursorsCreate (_2: *mut Display, _1: c_int) -> *mut XcursorCursors,
  pub fn XcursorCursorsDestroy (_1: *mut XcursorCursors) -> (),
  pub fn XcursorFileLoad (_3: *mut FILE, _2: *mut *mut XcursorComments, _1: *mut *mut XcursorImages) -> c_int,
  pub fn XcursorFileLoadAllImages (_1: *mut FILE) -> *mut XcursorImages,
  pub fn XcursorFileLoadImage (_2: *mut FILE, _1: c_int) -> *mut XcursorImage,
  pub fn XcursorFileLoadImages (_2: *mut FILE, _1: c_int) -> *mut XcursorImages,
  pub fn XcursorFilenameLoad (_3: *const c_char, _2: *mut *mut XcursorComments, _1: *mut *mut XcursorImages) -> c_int,
  pub fn XcursorFilenameLoadAllImages (_1: *const c_char) -> *mut XcursorImages,
  pub fn XcursorFilenameLoadCursor (_2: *mut Display, _1: *const c_char) -> c_ulong,
  pub fn XcursorFilenameLoadCursors (_2: *mut Display, _1: *const c_char) -> *mut XcursorCursors,
  pub fn XcursorFilenameLoadImage (_2: *const c_char, _1: c_int) -> *mut XcursorImage,
  pub fn XcursorFilenameLoadImages (_2: *const c_char, _1: c_int) -> *mut XcursorImages,
  pub fn XcursorFilenameSave (_3: *const c_char, _2: *const XcursorComments, _1: *const XcursorImages) -> c_int,
  pub fn XcursorFilenameSaveImages (_2: *const c_char, _1: *const XcursorImages) -> c_int,
  pub fn XcursorFileSave (_3: *mut FILE, _2: *const XcursorComments, _1: *const XcursorImages) -> c_int,
  pub fn XcursorFileSaveImages (_2: *mut FILE, _1: *const XcursorImages) -> c_int,
  pub fn XcursorGetDefaultSize (_1: *mut Display) -> c_int,
  pub fn XcursorGetTheme (_1: *mut Display) -> *mut c_char,
  pub fn XcursorGetThemeCore (_1: *mut Display) -> c_int,
  pub fn XcursorImageCreate (_2: c_int, _1: c_int) -> *mut XcursorImage,
  pub fn XcursorImageDestroy (_1: *mut XcursorImage) -> (),
  pub fn XcursorImageHash (_2: *mut XImage, _1: *mut c_uchar) -> (),
  pub fn XcursorImageLoadCursor (_2: *mut Display, _1: *const XcursorImage) -> c_ulong,
  pub fn XcursorImagesCreate (_1: c_int) -> *mut XcursorImages,
  pub fn XcursorImagesDestroy (_1: *mut XcursorImages) -> (),
  pub fn XcursorImagesLoadCursor (_2: *mut Display, _1: *const XcursorImages) -> c_ulong,
  pub fn XcursorImagesLoadCursors (_2: *mut Display, _1: *const XcursorImages) -> *mut XcursorCursors,
  pub fn XcursorImagesSetName (_2: *mut XcursorImages, _1: *const c_char) -> (),
  pub fn XcursorLibraryLoadCursor (_2: *mut Display, _1: *const c_char) -> c_ulong,
  pub fn XcursorLibraryLoadCursors (_2: *mut Display, _1: *const c_char) -> *mut XcursorCursors,
  pub fn XcursorLibraryLoadImage (_3: *const c_char, _2: *const c_char, _1: c_int) -> *mut XcursorImage,
  pub fn XcursorLibraryLoadImages (_3: *const c_char, _2: *const c_char, _1: c_int) -> *mut XcursorImages,
  pub fn XcursorLibraryPath () -> *const c_char,
  pub fn XcursorLibraryShape (_1: *const c_char) -> c_int,
  pub fn XcursorNoticeCreateBitmap (_4: *mut Display, _3: c_ulong, _2: c_uint, _1: c_uint) -> (),
  pub fn XcursorNoticePutBitmap (_3: *mut Display, _2: c_ulong, _1: *mut XImage) -> (),
  pub fn XcursorSetDefaultSize (_2: *mut Display, _1: c_int) -> c_int,
  pub fn XcursorSetTheme (_2: *mut Display, _1: *const c_char) -> c_int,
  pub fn XcursorSetThemeCore (_2: *mut Display, _1: c_int) -> c_int,
  pub fn XcursorShapeLoadCursor (_2: *mut Display, _1: c_uint) -> c_ulong,
  pub fn XcursorShapeLoadCursors (_2: *mut Display, _1: c_uint) -> *mut XcursorCursors,
  pub fn XcursorShapeLoadImage (_3: c_uint, _2: *const c_char, _1: c_int) -> *mut XcursorImage,
  pub fn XcursorShapeLoadImages (_3: c_uint, _2: *const c_char, _1: c_int) -> *mut XcursorImages,
  pub fn XcursorSupportsAnim (_1: *mut Display) -> c_int,
  pub fn XcursorSupportsARGB (_1: *mut Display) -> c_int,
  pub fn XcursorTryShapeBitmapCursor (_7: *mut Display, _6: c_ulong, _5: c_ulong, _4: *mut XColor, _3: *mut XColor, _2: c_uint, _1: c_uint) -> c_ulong,
  pub fn XcursorTryShapeCursor (_7: *mut Display, _6: c_ulong, _5: c_ulong, _4: c_uint, _3: c_uint, _2: *const XColor, _1: *const XColor) -> c_ulong,
  pub fn XcursorXcFileLoad (_3: *mut XcursorFile, _2: *mut *mut XcursorComments, _1: *mut *mut XcursorImages) -> c_int,
  pub fn XcursorXcFileLoadAllImages (_1: *mut XcursorFile) -> *mut XcursorImages,
  pub fn XcursorXcFileLoadImage (_2: *mut XcursorFile, _1: c_int) -> *mut XcursorImage,
  pub fn XcursorXcFileLoadImages (_2: *mut XcursorFile, _1: c_int) -> *mut XcursorImages,
  pub fn XcursorXcFileSave (_3: *mut XcursorFile, _2: *const XcursorComments, _1: *const XcursorImages) -> c_int,
variadic:
globals:
}

//
// types
//

pub type XcursorBool = c_int;
pub type XcursorDim = XcursorUInt;
pub type XcursorPixel = XcursorUInt;
pub type XcursorUInt = c_uint;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct _XcursorAnimate {
    pub cursors: *mut XcursorCursors,
    pub sequence: c_int,
}
pub type XcursorAnimate = _XcursorAnimate;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct _XcursorChunkHeader {
    pub header: XcursorUInt,
    pub type_: XcursorUInt,
    pub subtype: XcursorUInt,
    pub version: XcursorUInt,
}
pub type XcursorChunkHeader = _XcursorChunkHeader;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct _XcursorComment {
    pub version: XcursorUInt,
    pub comment_type: XcursorUInt,
    pub comment: *mut c_char,
}
pub type XcursorComment = _XcursorComment;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct _XcursorComments {
    pub ncomment: c_int,
    pub comments: *mut *mut XcursorComment,
}
pub type XcursorComments = _XcursorComments;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct _XcursorCursors {
    pub dpy: *mut Display,
    pub ref_: c_int,
    pub ncursor: c_int,
    pub cursors: *mut Cursor,
}
pub type XcursorCursors = _XcursorCursors;

#[derive(Debug, Copy)]
#[repr(C)]
pub struct _XcursorFile {
    pub closure: *mut c_void,
    pub read: Option<unsafe extern "C" fn(*mut XcursorFile, *mut c_uchar, c_int) -> c_int>,
    pub write: Option<unsafe extern "C" fn(*mut XcursorFile, *mut c_uchar, c_int) -> c_int>,
    pub seek: Option<unsafe extern "C" fn(*mut XcursorFile, c_long, c_int) -> c_int>,
}
pub type XcursorFile = _XcursorFile;

impl Clone for _XcursorFile {
    fn clone(&self) -> _XcursorFile {
        _XcursorFile {
            closure: self.closure,
            read: self.read,
            write: self.write,
            seek: self.seek,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct _XcursorFileHeader {
    pub magic: XcursorUInt,
    pub header: XcursorUInt,
    pub version: XcursorUInt,
    pub ntoc: XcursorUInt,
    pub tocs: *mut XcursorFileToc,
}
pub type XcursorFileHeader = _XcursorFileHeader;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct _XcursorFileToc {
    pub type_: XcursorUInt,
    pub subtype: XcursorUInt,
    pub position: XcursorUInt,
}
pub type XcursorFileToc = _XcursorFileToc;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct _XcursorImage {
    pub version: XcursorUInt,
    pub size: XcursorDim,
    pub width: XcursorDim,
    pub height: XcursorDim,
    pub xhot: XcursorDim,
    pub yhot: XcursorDim,
    pub delay: XcursorUInt,
    pub pixels: *mut XcursorPixel,
}
pub type XcursorImage = _XcursorImage;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct _XcursorImages {
    pub nimage: c_int,
    pub images: *mut *mut XcursorImage,
    pub name: *mut c_char,
}
pub type XcursorImages = _XcursorImages;
