// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong};

use super::xlib::{Display, XVisualInfo, XID};

//
// functions
//

x11_link! { Glx, gl, ["libGL.so.1", "libGL.so"], 40,
  pub fn glXChooseFBConfig (_4: *mut Display, _3: c_int, _2: *const c_int, _1: *mut c_int) -> *mut GLXFBConfig,
  pub fn glXChooseVisual (_3: *mut Display, _2: c_int, _1: *mut c_int) -> *mut XVisualInfo,
  pub fn glXCopyContext (_4: *mut Display, _3: GLXContext, _2: GLXContext, _1: c_ulong) -> (),
  pub fn glXCreateContext (_4: *mut Display, _3: *mut XVisualInfo, _2: GLXContext, _1: c_int) -> GLXContext,
  pub fn glXCreateGLXPixmap (_3: *mut Display, _2: *mut XVisualInfo, _1: c_ulong) -> c_ulong,
  pub fn glXCreateNewContext (_5: *mut Display, _4: GLXFBConfig, _3: c_int, _2: GLXContext, _1: c_int) -> GLXContext,
  pub fn glXCreatePbuffer (_3: *mut Display, _2: GLXFBConfig, _1: *const c_int) -> c_ulong,
  pub fn glXCreatePixmap (_4: *mut Display, _3: GLXFBConfig, _2: c_ulong, _1: *const c_int) -> c_ulong,
  pub fn glXCreateWindow (_4: *mut Display, _3: GLXFBConfig, _2: c_ulong, _1: *const c_int) -> c_ulong,
  pub fn glXDestroyContext (_2: *mut Display, _1: GLXContext) -> (),
  pub fn glXDestroyGLXPixmap (_2: *mut Display, _1: c_ulong) -> (),
  pub fn glXDestroyPbuffer (_2: *mut Display, _1: c_ulong) -> (),
  pub fn glXDestroyPixmap (_2: *mut Display, _1: c_ulong) -> (),
  pub fn glXDestroyWindow (_2: *mut Display, _1: c_ulong) -> (),
  pub fn glXGetClientString (_2: *mut Display, _1: c_int) -> *const c_char,
  pub fn glXGetConfig (_4: *mut Display, _3: *mut XVisualInfo, _2: c_int, _1: *mut c_int) -> c_int,
  pub fn glXGetCurrentContext () -> GLXContext,
  pub fn glXGetCurrentDisplay () -> *mut Display,
  pub fn glXGetCurrentDrawable () -> c_ulong,
  pub fn glXGetCurrentReadDrawable () -> c_ulong,
  pub fn glXGetFBConfigAttrib (_4: *mut Display, _3: GLXFBConfig, _2: c_int, _1: *mut c_int) -> c_int,
  pub fn glXGetFBConfigs (_3: *mut Display, _2: c_int, _1: *mut c_int) -> *mut GLXFBConfig,
  pub fn glXGetProcAddress (_1: *const c_uchar) -> Option<unsafe extern "C" fn ()>,
  pub fn glXGetProcAddressARB (_1: *const c_uchar) -> Option<unsafe extern "C" fn ()>,
  pub fn glXGetSelectedEvent (_3: *mut Display, _2: c_ulong, _1: *mut c_ulong) -> (),
  pub fn glXGetVisualFromFBConfig (_2: *mut Display, _1: GLXFBConfig) -> *mut XVisualInfo,
  pub fn glXIsDirect (_2: *mut Display, _1: GLXContext) -> c_int,
  pub fn glXMakeContextCurrent (_4: *mut Display, _3: c_ulong, _2: c_ulong, _1: GLXContext) -> c_int,
  pub fn glXMakeCurrent (_3: *mut Display, _2: c_ulong, _1: GLXContext) -> c_int,
  pub fn glXQueryContext (_4: *mut Display, _3: GLXContext, _2: c_int, _1: *mut c_int) -> c_int,
  pub fn glXQueryDrawable (_4: *mut Display, _3: c_ulong, _2: c_int, _1: *mut c_uint) -> (),
  pub fn glXQueryExtension (_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn glXQueryExtensionsString (_2: *mut Display, _1: c_int) -> *const c_char,
  pub fn glXQueryServerString (_3: *mut Display, _2: c_int, _1: c_int) -> *const c_char,
  pub fn glXQueryVersion (_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn glXSelectEvent (_3: *mut Display, _2: c_ulong, _1: c_ulong) -> (),
  pub fn glXSwapBuffers (_2: *mut Display, _1: c_ulong) -> (),
  pub fn glXUseXFont (_4: c_ulong, _3: c_int, _2: c_int, _1: c_int) -> (),
  pub fn glXWaitGL () -> (),
  pub fn glXWaitX () -> (),
variadic:
globals:
}

//
// types
//

// opaque structures
#[repr(C)]
pub struct __GLXcontextRec;
#[repr(C)]
pub struct __GLXFBConfigRec;

// types
pub type GLXContext = *mut __GLXcontextRec;
pub type GLXContextID = XID;
pub type GLXDrawable = XID;
pub type GLXFBConfig = *mut __GLXFBConfigRec;
pub type GLXFBConfigID = XID;
pub type GLXPbuffer = XID;
pub type GLXPixmap = XID;
pub type GLXWindow = XID;

//
// constants
//

// config caveats
pub const GLX_SLOW_CONFIG: c_int = 0x8001;
pub const GLX_NON_CONFORMANT_CONFIG: c_int = 0x800d;

// drawable type mask
pub const GLX_WINDOW_BIT: c_int = 0x0001;
pub const GLX_PIXMAP_BIT: c_int = 0x0002;
pub const GLX_PBUFFER_BIT: c_int = 0x0004;

// framebuffer attributes
pub const GLX_USE_GL: c_int = 0x0001;
pub const GLX_BUFFER_SIZE: c_int = 0x0002;
pub const GLX_LEVEL: c_int = 0x0003;
pub const GLX_RGBA: c_int = 0x0004;
pub const GLX_DOUBLEBUFFER: c_int = 0x0005;
pub const GLX_STEREO: c_int = 0x0006;
pub const GLX_AUX_BUFFERS: c_int = 0x0007;
pub const GLX_RED_SIZE: c_int = 0x0008;
pub const GLX_GREEN_SIZE: c_int = 0x0009;
pub const GLX_BLUE_SIZE: c_int = 0x000a;
pub const GLX_ALPHA_SIZE: c_int = 0x000b;
pub const GLX_DEPTH_SIZE: c_int = 0x000c;
pub const GLX_STENCIL_SIZE: c_int = 0x000d;
pub const GLX_ACCUM_RED_SIZE: c_int = 0x000e;
pub const GLX_ACCUM_GREEN_SIZE: c_int = 0x000f;
pub const GLX_ACCUM_BLUE_SIZE: c_int = 0x0010;
pub const GLX_ACCUM_ALPHA_SIZE: c_int = 0x0011;
pub const GLX_CONFIG_CAVEAT: c_int = 0x0020;
pub const GLX_X_VISUAL_TYPE: c_int = 0x0022;
pub const GLX_TRANSPARENT_TYPE: c_int = 0x0023;
pub const GLX_TRANSPARENT_INDEX_VALUE: c_int = 0x0024;
pub const GLX_TRANSPARENT_RED_VALUE: c_int = 0x0025;
pub const GLX_TRANSPARENT_GREEN_VALUE: c_int = 0x0026;
pub const GLX_TRANSPARENT_BLUE_VALUE: c_int = 0x0027;
pub const GLX_TRANSPARENT_ALPHA_VALUE: c_int = 0x0028;
pub const GLX_VISUAL_ID: c_int = 0x800B;
pub const GLX_SCREEN: c_int = 0x800C;
pub const GLX_DRAWABLE_TYPE: c_int = 0x8010;
pub const GLX_RENDER_TYPE: c_int = 0x8011;
pub const GLX_X_RENDERABLE: c_int = 0x8012;
pub const GLX_FBCONFIG_ID: c_int = 0x8013;
pub const GLX_MAX_PBUFFER_WIDTH: c_int = 0x8016;
pub const GLX_MAX_PBUFFER_HEIGHT: c_int = 0x8017;
pub const GLX_MAX_PBUFFER_PIXELS: c_int = 0x8018;
pub const GLX_SAMPLE_BUFFERS: c_int = 0x1_86a0;
pub const GLX_SAMPLES: c_int = 0x1_86a1;

// misc
pub const GLX_DONT_CARE: c_int = -1;
pub const GLX_NONE: c_int = 0x8000;

// render type mask
pub const GLX_RGBA_BIT: c_int = 0x0001;
pub const GLX_COLOR_INDEX_BIT: c_int = 0x0002;

// transparent types
pub const GLX_TRANSPARENT_RGB: c_int = 0x8008;
pub const GLX_TRANSPARENT_INDEX: c_int = 0x8009;

// visual types
pub const GLX_TRUE_COLOR: c_int = 0x8002;
pub const GLX_DIRECT_COLOR: c_int = 0x8003;
pub const GLX_PSEUDO_COLOR: c_int = 0x8004;
pub const GLX_STATIC_COLOR: c_int = 0x8005;
pub const GLX_GRAY_SCALE: c_int = 0x8006;
pub const GLX_STATIC_GRAY: c_int = 0x8007;

// glXGetConfig errors
pub const GLX_BAD_SCREEN: c_int = 1;
pub const GLX_BAD_ATTRIBUTE: c_int = 2;
pub const GLX_NO_EXTENSION: c_int = 3;
pub const GLX_BAD_VISUAL: c_int = 4;
pub const GLX_BAD_CONTEXT: c_int = 5;
pub const GLX_BAD_VALUE: c_int = 6;
pub const GLX_BAD_ENUM: c_int = 7;

// glXGetClientString names
pub const GLX_VENDOR: c_int = 1;
pub const GLX_VERSION: c_int = 2;
pub const GLX_EXTENSIONS: c_int = 3;

// drawable type mask?
pub const GLX_FRONT_LEFT_BUFFER_BIT: c_uint = 0x0001;
pub const GLX_FRONT_RIGHT_BUFFER_BIT: c_uint = 0x0002;
pub const GLX_BACK_LEFT_BUFFER_BIT: c_uint = 0x0004;
pub const GLX_BACK_RIGHT_BUFFER_BIT: c_uint = 0x0008;
pub const GLX_AUX_BUFFERS_BIT: c_uint = 0x0010;
pub const GLX_DEPTH_BUFFER_BIT: c_uint = 0x0020;
pub const GLX_STENCIL_BUFFER_BIT: c_uint = 0x0040;
pub const GLX_ACCUM_BUFFER_BIT: c_uint = 0x0080;

// render type for glXCreateNewContext
pub const GLX_RGBA_TYPE: c_int = 0x8014;
pub const GLX_COLOR_INDEX_TYPE: c_int = 0x8015;

// drawable attributes
pub const GLX_PRESERVED_CONTENTS: c_int = 0x801B;
pub const GLX_LARGEST_PBUFFER: c_int = 0x801C;
pub const GLX_WIDTH: c_int = 0x801D;
pub const GLX_HEIGHT: c_int = 0x801E;
pub const GLX_PBUFFER_HEIGHT: c_int = 0x8040;
pub const GLX_PBUFFER_WIDTH: c_int = 0x8041;

// other?
pub const GLX_EVENT_MASK: c_int = 0x801F;

// event mask
pub const GLX_PBUFFER_CLOBBER_MASK: c_ulong = 0x0800_0000;

// event types
pub const GLX_DAMAGED: c_int = 0x8020;
pub const GLX_SAVED: c_int = 0x8021;

// drawable types
pub const GLX_WINDOW: c_int = 0x8022;
pub const GLX_PBUFFER: c_int = 0x8023;

//
// ARB extensions
//

pub mod arb {
    use std::os::raw::c_int;

    // context attributes
    pub const GLX_CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
    pub const GLX_CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
    pub const GLX_CONTEXT_FLAGS_ARB: c_int = 0x2094;
    pub const GLX_CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;

    // context flags
    pub const GLX_CONTEXT_DEBUG_BIT_ARB: c_int = 0x0001;
    pub const GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: c_int = 0x0002;

    // context profile mask
    pub const GLX_CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x0001;
    pub const GLX_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: c_int = 0x0002;
}

//
// EXT extensions
//

pub mod ext {
    use std::os::raw::c_int;

    // drawable attributes
    pub const GLX_SWAP_INTERVAL_EXT: c_int = 0x20f1;
    pub const GLX_MAX_SWAP_INTERVAL_EXT: c_int = 0x20f2;
}
