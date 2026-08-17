pub type XID = super::__gl_imports::raw::c_ulong;
pub type Bool = super::__gl_imports::raw::c_int; // Not sure if this is correct...
pub enum Display {}

pub type Font = XID;
pub type Pixmap = XID;
pub enum Visual {} // TODO: not sure
pub type VisualID = super::__gl_imports::raw::c_ulong; // TODO: not sure
pub type Window = XID;
pub type GLXFBConfigID = XID;
pub type GLXFBConfig = *const super::__gl_imports::raw::c_void;
pub type GLXContextID = XID;
pub type GLXContext = *const super::__gl_imports::raw::c_void;
pub type GLXPixmap = XID;
pub type GLXDrawable = XID;
pub type GLXWindow = XID;
pub type GLXPbuffer = XID;
pub enum __GLXextFuncPtr_fn {}
pub type __GLXextFuncPtr = *mut __GLXextFuncPtr_fn;
pub type GLXVideoCaptureDeviceNV = XID;
pub type GLXVideoDeviceNV = super::__gl_imports::raw::c_int;
pub type GLXVideoSourceSGIX = XID;
pub type GLXFBConfigIDSGIX = XID;
pub type GLXFBConfigSGIX = *const super::__gl_imports::raw::c_void;
pub type GLXPbufferSGIX = XID;

#[repr(C)]
pub struct XVisualInfo {
    pub visual: *mut Visual,
    pub visualid: VisualID,
    pub screen: super::__gl_imports::raw::c_int,
    pub depth: super::__gl_imports::raw::c_int,
    pub class: super::__gl_imports::raw::c_int,
    pub red_mask: super::__gl_imports::raw::c_ulong,
    pub green_mask: super::__gl_imports::raw::c_ulong,
    pub blue_mask: super::__gl_imports::raw::c_ulong,
    pub colormap_size: super::__gl_imports::raw::c_int,
    pub bits_per_rgb: super::__gl_imports::raw::c_int,
}

#[repr(C)]
pub struct GLXPbufferClobberEvent {
    pub event_type: super::__gl_imports::raw::c_int, // GLX_DAMAGED or GLX_SAVED
    pub draw_type: super::__gl_imports::raw::c_int, // GLX_WINDOW or GLX_PBUFFER
    pub serial: super::__gl_imports::raw::c_ulong, // # of last request processed by server
    pub send_event: Bool, // true if this came for SendEvent request
    pub display: *const Display, // display the event was read from
    pub drawable: GLXDrawable, // XID of Drawable
    pub buffer_mask: super::__gl_imports::raw::c_uint, // mask indicating which buffers are affected
    pub aux_buffer: super::__gl_imports::raw::c_uint, // which aux buffer was affected
    pub x: super::__gl_imports::raw::c_int,
    pub y: super::__gl_imports::raw::c_int,
    pub width: super::__gl_imports::raw::c_int,
    pub height: super::__gl_imports::raw::c_int,
    pub count: super::__gl_imports::raw::c_int, // if nonzero, at least this many more
}

#[repr(C)]
pub struct GLXBufferSwapComplete {
    pub type_: super::__gl_imports::raw::c_int,
    pub serial: super::__gl_imports::raw::c_ulong, // # of last request processed by server
    pub send_event: Bool, // true if this came from a SendEvent request
    pub display: *const Display, // Display the event was read from
    pub drawable: GLXDrawable, // drawable on which event was requested in event mask
    pub event_type: super::__gl_imports::raw::c_int,
    pub ust: i64,
    pub msc: i64,
    pub sbc: i64,
}

// typedef union __GLXEvent {
//     GLXPbufferClobberEvent glxpbufferclobber;
//     GLXBufferSwapComplete glxbufferswapcomplete;
//     long pad[24];
// }

#[repr(C)]
pub struct GLXBufferClobberEventSGIX {
    pub type_: super::__gl_imports::raw::c_int,
    pub serial: super::__gl_imports::raw::c_ulong, // # of last request processed by server
    pub send_event: Bool, // true if this came for SendEvent request
    pub display: *const Display, // display the event was read from
    pub drawable: GLXDrawable, // i.d. of Drawable
    pub event_type: super::__gl_imports::raw::c_int, // GLX_DAMAGED_SGIX or GLX_SAVED_SGIX
    pub draw_type: super::__gl_imports::raw::c_int, // GLX_WINDOW_SGIX or GLX_PBUFFER_SGIX
    pub mask: super::__gl_imports::raw::c_uint, // mask indicating which buffers are affected
    pub x: super::__gl_imports::raw::c_int,
    pub y: super::__gl_imports::raw::c_int,
    pub width: super::__gl_imports::raw::c_int,
    pub height: super::__gl_imports::raw::c_int,
    pub count: super::__gl_imports::raw::c_int, // if nonzero, at least this many more
}

#[repr(C)]
pub struct GLXHyperpipeNetworkSGIX {
    pub pipeName: [super::__gl_imports::raw::c_char; 80], // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
    pub networkId: super::__gl_imports::raw::c_int,
}

#[repr(C)]
pub struct GLXHyperpipeConfigSGIX {
    pub pipeName: [super::__gl_imports::raw::c_char; 80], // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
    pub channel: super::__gl_imports::raw::c_int,
    pub participationType: super::__gl_imports::raw::c_uint,
    pub timeSlice: super::__gl_imports::raw::c_int,
}

#[repr(C)]
pub struct GLXPipeRect {
    pub pipeName: [super::__gl_imports::raw::c_char; 80], // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
    pub srcXOrigin: super::__gl_imports::raw::c_int,
    pub srcYOrigin: super::__gl_imports::raw::c_int,
    pub srcWidth: super::__gl_imports::raw::c_int,
    pub srcHeight: super::__gl_imports::raw::c_int,
    pub destXOrigin: super::__gl_imports::raw::c_int,
    pub destYOrigin: super::__gl_imports::raw::c_int,
    pub destWidth: super::__gl_imports::raw::c_int,
    pub destHeight: super::__gl_imports::raw::c_int,
}

#[repr(C)]
pub struct GLXPipeRectLimits {
    pub pipeName: [super::__gl_imports::raw::c_char; 80], // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]
    pub XOrigin: super::__gl_imports::raw::c_int,
    pub YOrigin: super::__gl_imports::raw::c_int,
    pub maxHeight: super::__gl_imports::raw::c_int,
    pub maxWidth: super::__gl_imports::raw::c_int,
}
