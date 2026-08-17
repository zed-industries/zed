use xcb::ffi::xcb_generic_event_t;
use xcb::{self, dri2, glx, x};
use xcb::{Raw, Xid};

use x11::glx::*;
use x11::xlib;

use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_void};
use std::ptr;

const GLX_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
const GLX_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;

type GlXCreateContextAttribsARBProc = unsafe extern "C" fn(
    dpy: *mut xlib::Display,
    fbc: GLXFBConfig,
    share_context: GLXContext,
    direct: xlib::Bool,
    attribs: *const c_int,
) -> GLXContext;

unsafe fn load_gl_func(name: &str) -> *mut c_void {
    let cname = CString::new(name).unwrap();
    let ptr: *mut c_void = std::mem::transmute(glXGetProcAddress(cname.as_ptr() as *const u8));
    if ptr.is_null() {
        panic!("could not load {}", name);
    }
    ptr
}

fn check_glx_extension(glx_exts: &str, ext_name: &str) -> bool {
    for glx_ext in glx_exts.split(" ") {
        if glx_ext == ext_name {
            return true;
        }
    }
    false
}

static mut CTX_ERROR_OCCURED: bool = false;
unsafe extern "C" fn ctx_error_handler(
    _dpy: *mut xlib::Display,
    _ev: *mut xlib::XErrorEvent,
) -> i32 {
    CTX_ERROR_OCCURED = true;
    0
}

unsafe fn check_gl_error() {
    let err = gl::GetError();
    if err != gl::NO_ERROR {
        println!("got gl error {}", err);
    }
}

fn get_glxfbconfig(
    dpy: *mut xlib::Display,
    screen_num: i32,
    visual_attribs: &[i32],
) -> GLXFBConfig {
    unsafe {
        let mut fbcount: c_int = 0;
        let fbcs = glXChooseFBConfig(
            dpy,
            screen_num,
            visual_attribs.as_ptr(),
            &mut fbcount as *mut c_int,
        );

        if fbcount == 0 {
            panic!("could not find compatible fb config");
        }
        // we pick the first from the list
        let fbc = *fbcs;
        xlib::XFree(fbcs as *mut c_void);
        fbc
    }
}

fn main() -> xcb::Result<()> {
    let (conn, screen_num) =
        xcb::Connection::connect_with_xlib_display_and_extensions(&[], &[xcb::Extension::Dri2])?;

    conn.set_event_queue_owner(xcb::EventQueueOwner::Xcb);

    let glx_ver = conn.wait_for_reply(conn.send_request(&glx::QueryVersion {
        major_version: 1,
        minor_version: 3,
    }))?;
    assert!(glx_ver.major_version() >= 1 && glx_ver.minor_version() >= 3);

    let fbc = get_glxfbconfig(
        conn.get_raw_dpy(),
        screen_num,
        &[
            GLX_X_RENDERABLE,
            1,
            GLX_DRAWABLE_TYPE,
            GLX_WINDOW_BIT,
            GLX_RENDER_TYPE,
            GLX_RGBA_BIT,
            GLX_X_VISUAL_TYPE,
            GLX_TRUE_COLOR,
            GLX_RED_SIZE,
            8,
            GLX_GREEN_SIZE,
            8,
            GLX_BLUE_SIZE,
            8,
            GLX_ALPHA_SIZE,
            8,
            GLX_DEPTH_SIZE,
            24,
            GLX_STENCIL_SIZE,
            8,
            GLX_DOUBLEBUFFER,
            1,
            0,
        ],
    );

    let vi_ptr: *mut xlib::XVisualInfo =
        unsafe { glXGetVisualFromFBConfig(conn.get_raw_dpy(), fbc) };
    let vi = unsafe { *vi_ptr };

    // retrieving a few atoms
    let (wm_protocols, wm_del_window) = {
        let cookies = (
            conn.send_request(&x::InternAtom {
                only_if_exists: false,
                name: b"WM_PROTOCOLS",
            }),
            conn.send_request(&x::InternAtom {
                only_if_exists: false,
                name: b"WM_DELETE_WINDOW",
            }),
        );
        (
            conn.wait_for_reply(cookies.0)?.atom(),
            conn.wait_for_reply(cookies.1)?.atom(),
        )
    };

    let setup = conn.get_setup();
    let screen = setup.roots().nth(vi.screen as usize).unwrap();

    let cmap: x::Colormap = conn.generate_id();
    let win: x::Window = conn.generate_id();

    conn.send_request(&x::CreateColormap {
        alloc: x::ColormapAlloc::None,
        mid: cmap,
        window: screen.root(),
        visual: vi.visualid as u32,
    });

    conn.send_request(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid: win,
        parent: screen.root(),
        x: 0,
        y: 0,
        width: 640,
        height: 480,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: vi.visualid as u32,
        value_list: &[
            x::Cw::BackPixel(screen.white_pixel()),
            x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS),
            x::Cw::Colormap(cmap),
        ],
    });

    unsafe {
        xlib::XFree(vi_ptr as *mut c_void);
    }

    let title = "XCB OpenGL";

    conn.check_request(conn.send_request_checked(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: win,
        property: x::ATOM_WM_NAME,
        r#type: x::ATOM_STRING,
        data: title.as_bytes(),
    }))?;

    conn.check_request(conn.send_request_checked(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: win,
        property: wm_protocols,
        r#type: x::ATOM_ATOM,
        data: &[wm_del_window],
    }))?;

    conn.check_request(conn.send_request_checked(&x::MapWindow { window: win }))?;

    unsafe {
        xlib::XSync(conn.get_raw_dpy(), xlib::False);
    }

    let glx_exts =
        unsafe { CStr::from_ptr(glXQueryExtensionsString(conn.get_raw_dpy(), screen_num)) }
            .to_str()
            .unwrap();

    if !check_glx_extension(&glx_exts, "GLX_ARB_create_context") {
        panic!("could not find GLX extension GLX_ARB_create_context");
    }

    // with glx, no need of a current context is needed to load symbols
    // otherwise we would need to create a temporary legacy GL context
    // for loading symbols (at least glXCreateContextAttribsARB)
    let glx_create_context_attribs: GlXCreateContextAttribsARBProc =
        unsafe { std::mem::transmute(load_gl_func("glXCreateContextAttribsARB")) };

    // loading all other symbols
    unsafe {
        gl::load_with(|n| load_gl_func(&n));
    }

    if !gl::GenVertexArrays::is_loaded() {
        panic!("no GL3 support available!");
    }

    // installing an event handler to check if error is generated
    unsafe {
        CTX_ERROR_OCCURED = false;
    }

    let old_handler = unsafe { xlib::XSetErrorHandler(Some(ctx_error_handler)) };

    let context_attribs: [c_int; 5] = [
        GLX_CONTEXT_MAJOR_VERSION_ARB as c_int,
        3,
        GLX_CONTEXT_MINOR_VERSION_ARB as c_int,
        0,
        0,
    ];
    let ctx = unsafe {
        glx_create_context_attribs(
            conn.get_raw_dpy(),
            fbc,
            ptr::null_mut(),
            xlib::True,
            &context_attribs[0] as *const c_int,
        )
    };

    conn.flush()?;

    unsafe {
        xlib::XSync(conn.get_raw_dpy(), xlib::False);
        xlib::XSetErrorHandler(std::mem::transmute(old_handler));
    }

    if ctx.is_null() || unsafe { CTX_ERROR_OCCURED } {
        panic!("error when creating gl-3.0 context");
    }

    if unsafe { glXIsDirect(conn.get_raw_dpy(), ctx) } == 0 {
        panic!("obtained indirect rendering context")
    }

    loop {
        conn.flush()?;

        match conn.wait_for_event()? {
            xcb::Event::X(x::Event::Expose(_)) => unsafe {
                glXMakeCurrent(conn.get_raw_dpy(), win.resource_id() as xlib::XID, ctx);
                gl::ClearColor(0.5f32, 0.5f32, 1.0f32, 1.0f32);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::Flush();
                check_gl_error();
                glXSwapBuffers(conn.get_raw_dpy(), win.resource_id() as xlib::XID);
                glXMakeCurrent(conn.get_raw_dpy(), 0, ptr::null_mut());
            },
            xcb::Event::X(x::Event::KeyPress(_ev)) => {}
            xcb::Event::X(x::Event::ClientMessage(ev)) => {
                if let x::ClientMessageData::Data32([atom, ..]) = ev.data() {
                    if atom == wm_del_window.resource_id() {
                        // window "x" button clicked by user, we gracefully exit
                        break;
                    }
                }
            }

            // Following stuff is not obvious at all.
            // I've seen this as necessary in the past to handle GL when XCB owns the event queue.
            // It doesn't seem necessary anymore (in fact DRI2 is not present
            // on my system, so I won't even hit this code) but I leave it here
            // in case it is useful to someone.

            // These are libgl dri2 event that need special handling
            // see https://bugs.freedesktop.org/show_bug.cgi?id=35945#c4
            // and mailing thread starting here:
            // http://lists.freedesktop.org/archives/xcb/2015-November/010556.html
            xcb::Event::Dri2(dri2::Event::BufferSwapComplete(ev)) => unsafe {
                rewire_event(&conn, ev.as_raw())
            },
            xcb::Event::Dri2(dri2::Event::InvalidateBuffers(ev)) => unsafe {
                rewire_event(&conn, ev.as_raw())
            },
            _ => {}
        }
    }

    unsafe {
        glXDestroyContext(conn.get_raw_dpy(), ctx);
    }

    conn.send_request(&x::UnmapWindow { window: win });
    conn.send_request(&x::DestroyWindow { window: win });
    conn.send_request(&x::FreeColormap { cmap });
    conn.flush()?;

    Ok(())
}

unsafe fn rewire_event(conn: &xcb::Connection, raw_ev: *mut xcb_generic_event_t) {
    let ev_type = ((*raw_ev).response_type & 0x7f) as i32;

    if let Some(proc) = xlib::XESetWireToEvent(conn.get_raw_dpy(), ev_type, None) {
        xlib::XESetWireToEvent(conn.get_raw_dpy(), ev_type, Some(proc));
        (*raw_ev).sequence = xlib::XLastKnownRequestProcessed(conn.get_raw_dpy()) as u16;
        let mut dummy: xlib::XEvent = std::mem::zeroed();
        proc(
            conn.get_raw_dpy(),
            &mut dummy as *mut xlib::XEvent,
            raw_ev as *mut xlib::xEvent,
        );
    }
}
