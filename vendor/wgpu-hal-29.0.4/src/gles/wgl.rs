use alloc::{borrow::ToOwned as _, ffi::CString, string::String, sync::Arc, vec::Vec};
use core::{
    ffi::{c_int, c_void, CStr},
    mem::{self, ManuallyDrop},
    ptr,
    time::Duration,
};
use std::{
    sync::{
        mpsc::{sync_channel, SyncSender},
        LazyLock,
    },
    thread,
};

use glow::HasContext;
use glutin_wgl_sys::wgl_extra::{
    Wgl, CONTEXT_CORE_PROFILE_BIT_ARB, CONTEXT_DEBUG_BIT_ARB, CONTEXT_FLAGS_ARB,
    CONTEXT_PROFILE_MASK_ARB,
};
use hashbrown::HashSet;
use parking_lot::{Mutex, MutexGuard, RwLock};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use wgt::InstanceFlags;
use windows::{
    core::{Error, PCSTR},
    Win32::{
        Foundation,
        Graphics::{Gdi, OpenGL},
        System::LibraryLoader,
        UI::WindowsAndMessaging,
    },
};

/// The amount of time to wait while trying to obtain a lock to the adapter context
const CONTEXT_LOCK_TIMEOUT_SECS: u64 = 1;

/// A wrapper around a `[`glow::Context`]` and the required WGL context that uses locking to
/// guarantee exclusive access when shared with multiple threads.
pub struct AdapterContext {
    inner: Arc<Mutex<Inner>>,
}

unsafe impl Sync for AdapterContext {}
unsafe impl Send for AdapterContext {}

impl AdapterContext {
    pub fn is_owned(&self) -> bool {
        true
    }

    pub fn raw_context(&self) -> *mut c_void {
        match self.inner.lock().context {
            Some(ref wgl) => wgl.context.0,
            None => ptr::null_mut(),
        }
    }

    /// Obtain a lock to the WGL context and get handle to the [`glow::Context`] that can be used to
    /// do rendering.
    #[track_caller]
    pub fn lock(&self) -> AdapterContextLock<'_> {
        let inner = self
            .inner
            // Don't lock forever. If it takes longer than 1 second to get the lock we've got a
            // deadlock and should panic to show where we got stuck
            .try_lock_for(Duration::from_secs(CONTEXT_LOCK_TIMEOUT_SECS))
            .expect("Could not lock adapter context. This is most-likely a deadlock.");

        if let Some(wgl) = &inner.context {
            wgl.make_current(inner.device.dc).unwrap()
        };

        AdapterContextLock { inner }
    }

    /// Obtain a lock to the WGL context and get handle to the [`glow::Context`] that can be used to
    /// do rendering.
    ///
    /// Unlike [`lock`](Self::lock), this accepts a device to pass to `make_current` and exposes the error
    /// when `make_current` fails.
    #[track_caller]
    fn lock_with_dc(&self, device: Gdi::HDC) -> windows::core::Result<AdapterContextLock<'_>> {
        let inner = self
            .inner
            .try_lock_for(Duration::from_secs(CONTEXT_LOCK_TIMEOUT_SECS))
            .expect("Could not lock adapter context. This is most-likely a deadlock.");

        if let Some(wgl) = &inner.context {
            wgl.make_current(device)?;
        }

        Ok(AdapterContextLock { inner })
    }
}

/// A guard containing a lock to an [`AdapterContext`], while the GL context is kept current.
pub struct AdapterContextLock<'a> {
    inner: MutexGuard<'a, Inner>,
}

impl<'a> core::ops::Deref for AdapterContextLock<'a> {
    type Target = glow::Context;

    fn deref(&self) -> &Self::Target {
        &self.inner.gl
    }
}

impl<'a> Drop for AdapterContextLock<'a> {
    fn drop(&mut self) {
        if let Some(wgl) = &self.inner.context {
            wgl.unmake_current().unwrap()
        }
    }
}

struct WglContext {
    context: OpenGL::HGLRC,
}

impl WglContext {
    fn make_current(&self, device: Gdi::HDC) -> windows::core::Result<()> {
        unsafe { OpenGL::wglMakeCurrent(device, self.context) }
    }

    fn unmake_current(&self) -> windows::core::Result<()> {
        if unsafe { OpenGL::wglGetCurrentContext() }.is_invalid() {
            return Ok(());
        }
        unsafe { OpenGL::wglMakeCurrent(Default::default(), Default::default()) }
    }
}

impl Drop for WglContext {
    fn drop(&mut self) {
        if let Err(e) = unsafe { OpenGL::wglDeleteContext(self.context) } {
            log::error!("failed to delete WGL context: {e}");
        }
    }
}

unsafe impl Send for WglContext {}
unsafe impl Sync for WglContext {}

struct Inner {
    gl: ManuallyDrop<glow::Context>,
    device: InstanceDevice,
    context: Option<WglContext>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        struct CurrentGuard<'a>(&'a WglContext);
        impl Drop for CurrentGuard<'_> {
            fn drop(&mut self) {
                self.0.unmake_current().unwrap();
            }
        }

        // Context must be current when dropped. See safety docs on
        // `glow::HasContext`.
        //
        // NOTE: This is only set to `None` by `Adapter::new_external` which
        // requires the context to be current when anything that may be holding
        // the `Arc<AdapterShared>` is dropped.
        let _guard = self.context.as_ref().map(|wgl| {
            wgl.make_current(self.device.dc).unwrap();
            CurrentGuard(wgl)
        });
        // SAFETY: Field not used after this.
        unsafe { ManuallyDrop::drop(&mut self.gl) };
    }
}

unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

pub struct Instance {
    srgb_capable: bool,
    options: wgt::GlBackendOptions,
    inner: Arc<Mutex<Inner>>,
}

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

fn load_gl_func(name: &str, module: Option<Foundation::HMODULE>) -> *const c_void {
    let addr = CString::new(name.as_bytes()).unwrap();
    let mut ptr = unsafe { OpenGL::wglGetProcAddress(PCSTR(addr.as_ptr().cast())) };
    if ptr.is_none() {
        if let Some(module) = module {
            ptr = unsafe { LibraryLoader::GetProcAddress(module, PCSTR(addr.as_ptr().cast())) };
        }
    }
    ptr.map_or_else(ptr::null_mut, |p| p as *mut c_void)
}

fn get_extensions(extra: &Wgl, dc: Gdi::HDC) -> HashSet<String> {
    if extra.GetExtensionsStringARB.is_loaded() {
        unsafe { CStr::from_ptr(extra.GetExtensionsStringARB(dc.0)) }
            .to_str()
            .unwrap_or("")
    } else {
        ""
    }
    .split(' ')
    .map(|s| s.to_owned())
    .collect()
}

unsafe fn setup_pixel_format(dc: Gdi::HDC) -> Result<(), crate::InstanceError> {
    {
        let format = OpenGL::PIXELFORMATDESCRIPTOR {
            nVersion: 1,
            nSize: size_of::<OpenGL::PIXELFORMATDESCRIPTOR>() as u16,
            dwFlags: OpenGL::PFD_DRAW_TO_WINDOW
                | OpenGL::PFD_SUPPORT_OPENGL
                | OpenGL::PFD_DOUBLEBUFFER,
            iPixelType: OpenGL::PFD_TYPE_RGBA,
            cColorBits: 8,
            ..unsafe { mem::zeroed() }
        };

        let index = unsafe { OpenGL::ChoosePixelFormat(dc, &format) };
        if index == 0 {
            return Err(crate::InstanceError::with_source(
                String::from("unable to choose pixel format"),
                Error::from_thread(),
            ));
        }

        let current = unsafe { OpenGL::GetPixelFormat(dc) };

        if index != current {
            unsafe { OpenGL::SetPixelFormat(dc, index, &format) }.map_err(|e| {
                crate::InstanceError::with_source(String::from("unable to set pixel format"), e)
            })?;
        }
    }

    {
        let index = unsafe { OpenGL::GetPixelFormat(dc) };
        if index == 0 {
            return Err(crate::InstanceError::with_source(
                String::from("unable to get pixel format index"),
                Error::from_thread(),
            ));
        }
        let mut format = Default::default();
        if unsafe {
            OpenGL::DescribePixelFormat(dc, index, size_of_val(&format) as u32, Some(&mut format))
        } == 0
        {
            return Err(crate::InstanceError::with_source(
                String::from("unable to read pixel format"),
                Error::from_thread(),
            ));
        }

        if !format.dwFlags.contains(OpenGL::PFD_SUPPORT_OPENGL)
            || format.iPixelType != OpenGL::PFD_TYPE_RGBA
        {
            return Err(crate::InstanceError::new(String::from(
                "unsuitable pixel format",
            )));
        }
    }
    Ok(())
}

fn create_global_window_class() -> Result<CString, crate::InstanceError> {
    let instance = unsafe { LibraryLoader::GetModuleHandleA(None) }.map_err(|e| {
        crate::InstanceError::with_source(String::from("unable to get executable instance"), e)
    })?;

    // Use the address of `UNIQUE` as part of the window class name to ensure different
    // `wgpu` versions use different names.
    static UNIQUE: Mutex<u8> = Mutex::new(0);
    let class_addr: *const _ = &UNIQUE;
    let name = format!("wgpu Device Class {:x}\0", class_addr as usize);
    let name = CString::from_vec_with_nul(name.into_bytes()).unwrap();

    // The window class may already be registered if we are a dynamic library that got
    // unloaded & loaded back into the same process. If so, just skip creation.
    let already_exists = unsafe {
        let mut wc = mem::zeroed::<WindowsAndMessaging::WNDCLASSEXA>();
        WindowsAndMessaging::GetClassInfoExA(
            Some(instance.into()),
            PCSTR(name.as_ptr().cast()),
            &mut wc,
        )
        .is_ok()
    };
    if already_exists {
        return Ok(name);
    }

    // Use a wrapper function for compatibility with `windows-rs`.
    unsafe extern "system" fn wnd_proc(
        window: Foundation::HWND,
        msg: u32,
        wparam: Foundation::WPARAM,
        lparam: Foundation::LPARAM,
    ) -> Foundation::LRESULT {
        unsafe { WindowsAndMessaging::DefWindowProcA(window, msg, wparam, lparam) }
    }

    let window_class = WindowsAndMessaging::WNDCLASSEXA {
        cbSize: size_of::<WindowsAndMessaging::WNDCLASSEXA>() as u32,
        style: WindowsAndMessaging::CS_OWNDC,
        lpfnWndProc: Some(wnd_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: instance.into(),
        hIcon: WindowsAndMessaging::HICON::default(),
        hCursor: WindowsAndMessaging::HCURSOR::default(),
        hbrBackground: Gdi::HBRUSH::default(),
        lpszMenuName: PCSTR::null(),
        lpszClassName: PCSTR(name.as_ptr().cast()),
        hIconSm: WindowsAndMessaging::HICON::default(),
    };

    let atom = unsafe { WindowsAndMessaging::RegisterClassExA(&window_class) };

    if atom == 0 {
        return Err(crate::InstanceError::with_source(
            String::from("unable to register window class"),
            Error::from_thread(),
        ));
    }

    // We intentionally leak the window class as we only need one per process.

    Ok(name)
}

fn get_global_window_class() -> Result<CString, crate::InstanceError> {
    static GLOBAL: LazyLock<Result<CString, crate::InstanceError>> =
        LazyLock::new(create_global_window_class);
    GLOBAL.clone()
}

struct InstanceDevice {
    dc: Gdi::HDC,

    /// This is used to keep the thread owning `dc` alive until this struct is dropped.
    _tx: SyncSender<()>,
}

fn create_instance_device() -> Result<InstanceDevice, crate::InstanceError> {
    #[derive(Clone, Copy)]
    // TODO: We can get these SendSync definitions in the upstream metadata if this is the case
    struct SendDc(Gdi::HDC);
    unsafe impl Sync for SendDc {}
    unsafe impl Send for SendDc {}

    struct Window {
        window: Foundation::HWND,
    }
    impl Drop for Window {
        fn drop(&mut self) {
            if let Err(e) = unsafe { WindowsAndMessaging::DestroyWindow(self.window) } {
                log::error!("failed to destroy window: {e}");
            }
        }
    }

    let window_class = get_global_window_class()?;

    let (drop_tx, drop_rx) = sync_channel(0);
    let (setup_tx, setup_rx) = sync_channel(0);

    // We spawn a thread which owns the hidden window for this instance.
    thread::Builder::new()
        .stack_size(256 * 1024)
        .name("wgpu-hal WGL Instance Thread".to_owned())
        .spawn(move || {
            let setup = (|| {
                let instance = unsafe { LibraryLoader::GetModuleHandleA(None) }.map_err(|e| {
                    crate::InstanceError::with_source(
                        String::from("unable to get executable instance"),
                        e,
                    )
                })?;

                // Create a hidden window since we don't pass `WS_VISIBLE`.
                let window = unsafe {
                    WindowsAndMessaging::CreateWindowExA(
                        WindowsAndMessaging::WINDOW_EX_STYLE::default(),
                        PCSTR(window_class.as_ptr().cast()),
                        PCSTR(window_class.as_ptr().cast()),
                        WindowsAndMessaging::WINDOW_STYLE::default(),
                        0,
                        0,
                        1,
                        1,
                        None,
                        None,
                        Some(instance.into()),
                        None,
                    )
                }
                .map_err(|e| {
                    crate::InstanceError::with_source(
                        String::from("unable to create hidden instance window"),
                        e,
                    )
                })?;
                let window = Window { window };

                let dc = unsafe { Gdi::GetDC(Some(window.window)) };
                if dc.is_invalid() {
                    return Err(crate::InstanceError::with_source(
                        String::from("unable to create memory device"),
                        Error::from_thread(),
                    ));
                }
                let dc = DeviceContextHandle {
                    device: dc,
                    window: window.window,
                };
                unsafe { setup_pixel_format(dc.device)? };

                Ok((window, dc))
            })();

            match setup {
                Ok((_window, dc)) => {
                    setup_tx.send(Ok(SendDc(dc.device))).unwrap();
                    // Wait for the shutdown event to free the window and device context handle.
                    drop_rx.recv().ok();
                }
                Err(err) => {
                    setup_tx.send(Err(err)).unwrap();
                }
            }
        })
        .map_err(|e| {
            crate::InstanceError::with_source(String::from("unable to create instance thread"), e)
        })?;

    let dc = setup_rx.recv().unwrap()?.0;

    Ok(InstanceDevice { dc, _tx: drop_tx })
}

impl crate::Instance for Instance {
    type A = super::Api;

    unsafe fn init(desc: &crate::InstanceDescriptor<'_>) -> Result<Self, crate::InstanceError> {
        profiling::scope!("Init OpenGL (WGL) Backend");
        let opengl_module =
            unsafe { LibraryLoader::LoadLibraryA(PCSTR(c"opengl32.dll".as_ptr().cast())) }
                .map_err(|e| {
                    crate::InstanceError::with_source(
                        String::from("unable to load the OpenGL library"),
                        e,
                    )
                })?;

        let device = create_instance_device()?;
        let dc = device.dc;

        let context = unsafe { OpenGL::wglCreateContext(dc) }.map_err(|e| {
            crate::InstanceError::with_source(
                String::from("unable to create initial OpenGL context"),
                e,
            )
        })?;
        let context = WglContext { context };
        context.make_current(dc).map_err(|e| {
            crate::InstanceError::with_source(
                String::from("unable to set initial OpenGL context as current"),
                e,
            )
        })?;

        let extra = Wgl::load_with(|name| load_gl_func(name, None));
        let extensions = get_extensions(&extra, dc);

        let can_use_profile = extensions.contains("WGL_ARB_create_context_profile")
            && extra.CreateContextAttribsARB.is_loaded();

        let context = if can_use_profile {
            let attributes = [
                CONTEXT_PROFILE_MASK_ARB as c_int,
                CONTEXT_CORE_PROFILE_BIT_ARB as c_int,
                CONTEXT_FLAGS_ARB as c_int,
                if desc.flags.contains(InstanceFlags::DEBUG) {
                    CONTEXT_DEBUG_BIT_ARB as c_int
                } else {
                    0
                },
                0, // End of list
            ];
            let context =
                unsafe { extra.CreateContextAttribsARB(dc.0, ptr::null(), attributes.as_ptr()) };
            if context.is_null() {
                return Err(crate::InstanceError::with_source(
                    String::from("unable to create OpenGL context"),
                    Error::from_thread(),
                ));
            }
            WglContext {
                context: OpenGL::HGLRC(context.cast_mut()),
            }
        } else {
            context
        };

        context.make_current(dc).map_err(|e| {
            crate::InstanceError::with_source(
                String::from("unable to set OpenGL context as current"),
                e,
            )
        })?;

        let mut gl = unsafe {
            glow::Context::from_loader_function(|name| load_gl_func(name, Some(opengl_module)))
        };

        let extra = Wgl::load_with(|name| load_gl_func(name, None));
        let extensions = get_extensions(&extra, dc);

        let srgb_capable = extensions.contains("WGL_EXT_framebuffer_sRGB")
            || extensions.contains("WGL_ARB_framebuffer_sRGB")
            || gl
                .supported_extensions()
                .contains("GL_ARB_framebuffer_sRGB");

        // In contrast to OpenGL ES, OpenGL requires explicitly enabling sRGB conversions,
        // as otherwise the user has to do the sRGB conversion.
        if srgb_capable {
            unsafe { gl.enable(glow::FRAMEBUFFER_SRGB) };
        }

        if desc.flags.contains(InstanceFlags::VALIDATION) && gl.supports_debug() {
            log::debug!("Enabling GL debug output");
            unsafe { gl.enable(glow::DEBUG_OUTPUT) };
            unsafe { gl.debug_message_callback(super::gl_debug_message_callback) };
        }

        // Wrap in ManuallyDrop to make it easier to "current" the GL context before dropping this
        // GLOW context, which could also happen if a panic occurs after we uncurrent the context
        // below but before Inner is constructed.
        let gl = ManuallyDrop::new(gl);
        context.unmake_current().map_err(|e| {
            crate::InstanceError::with_source(
                String::from("unable to unset the current WGL context"),
                e,
            )
        })?;

        Ok(Instance {
            inner: Arc::new(Mutex::new(Inner {
                device,
                gl,
                context: Some(context),
            })),
            options: desc.backend_options.gl.clone(),
            srgb_capable,
        })
    }

    unsafe fn create_surface(
        &self,
        display_handle: RawDisplayHandle,
        window_handle: RawWindowHandle,
    ) -> Result<Surface, crate::InstanceError> {
        assert!(matches!(display_handle, RawDisplayHandle::Windows(_)));
        let window = if let RawWindowHandle::Win32(handle) = window_handle {
            handle
        } else {
            return Err(crate::InstanceError::new(format!(
                "unsupported window: {window_handle:?}"
            )));
        };
        Ok(Surface {
            // This cast exists because of https://github.com/rust-windowing/raw-window-handle/issues/171
            window: Foundation::HWND(window.hwnd.get() as *mut _),
            presentable: true,
            swapchain: RwLock::new(None),
            srgb_capable: self.srgb_capable,
        })
    }

    unsafe fn enumerate_adapters(
        &self,
        _surface_hint: Option<&Surface>,
    ) -> Vec<crate::ExposedAdapter<super::Api>> {
        unsafe {
            super::Adapter::expose(
                AdapterContext {
                    inner: self.inner.clone(),
                },
                self.options.clone(),
            )
        }
        .into_iter()
        .collect()
    }
}

impl super::Adapter {
    /// Creates a new external adapter using the specified loader function.
    ///
    /// # Safety
    ///
    /// - The underlying OpenGL ES context must be current.
    /// - The underlying OpenGL ES context must be current when interfacing with any objects returned by
    ///   wgpu-hal from this adapter.
    /// - The underlying OpenGL ES context must be current when dropping this adapter and when
    ///   dropping any objects returned from this adapter.
    pub unsafe fn new_external(
        fun: impl FnMut(&str) -> *const c_void,
        options: wgt::GlBackendOptions,
    ) -> Option<crate::ExposedAdapter<super::Api>> {
        let context = unsafe { glow::Context::from_loader_function(fun) };
        unsafe {
            Self::expose(
                AdapterContext {
                    inner: Arc::new(Mutex::new(Inner {
                        gl: ManuallyDrop::new(context),
                        device: create_instance_device().ok()?,
                        context: None,
                    })),
                },
                options,
            )
        }
    }

    pub fn adapter_context(&self) -> &AdapterContext {
        &self.shared.context
    }
}

impl super::Device {
    /// Returns the underlying WGL context.
    pub fn context(&self) -> &AdapterContext {
        &self.shared.context
    }
}

struct DeviceContextHandle {
    device: Gdi::HDC,
    window: Foundation::HWND,
}

impl Drop for DeviceContextHandle {
    fn drop(&mut self) {
        unsafe {
            Gdi::ReleaseDC(Some(self.window), self.device);
        };
    }
}

pub struct Swapchain {
    framebuffer: glow::Framebuffer,
    renderbuffer: glow::Renderbuffer,

    /// Extent because the window lies
    extent: wgt::Extent3d,

    format: wgt::TextureFormat,
    format_desc: super::TextureFormatDesc,
    #[allow(unused)]
    sample_type: wgt::TextureSampleType,
}

pub struct Surface {
    window: Foundation::HWND,
    pub(super) presentable: bool,
    swapchain: RwLock<Option<Swapchain>>,
    srgb_capable: bool,
}

unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}

impl Surface {
    pub(super) unsafe fn present(
        &self,
        _suf_texture: super::Texture,
        context: &AdapterContext,
    ) -> Result<(), crate::SurfaceError> {
        let swapchain = self.swapchain.read();
        let sc = swapchain.as_ref().unwrap();
        let dc = unsafe { Gdi::GetDC(Some(self.window)) };
        if dc.is_invalid() {
            log::error!(
                "unable to get the device context from window: {}",
                Error::from_thread()
            );
            return Err(crate::SurfaceError::Other(
                "unable to get the device context from window",
            ));
        }
        let dc = DeviceContextHandle {
            device: dc,
            window: self.window,
        };

        let gl = context.lock_with_dc(dc.device).map_err(|e| {
            log::error!("unable to make the OpenGL context current for surface: {e}",);
            crate::SurfaceError::Other("unable to make the OpenGL context current for surface")
        })?;

        unsafe { gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, None) };
        unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(sc.framebuffer)) };

        if self.srgb_capable {
            // Disable sRGB conversions for `glBlitFramebuffer` as behavior does diverge between
            // drivers and formats otherwise and we want to ensure no sRGB conversions happen.
            unsafe { gl.disable(glow::FRAMEBUFFER_SRGB) };
        }

        // Note the Y-flipping here. GL's presentation is not flipped,
        // but main rendering is. Therefore, we Y-flip the output positions
        // in the shader, and also this blit.
        unsafe {
            gl.blit_framebuffer(
                0,
                sc.extent.height as i32,
                sc.extent.width as i32,
                0,
                0,
                0,
                sc.extent.width as i32,
                sc.extent.height as i32,
                glow::COLOR_BUFFER_BIT,
                glow::NEAREST,
            )
        };

        if self.srgb_capable {
            unsafe { gl.enable(glow::FRAMEBUFFER_SRGB) };
        }

        unsafe { gl.bind_renderbuffer(glow::RENDERBUFFER, None) };
        unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, None) };

        if let Err(e) = unsafe { OpenGL::SwapBuffers(dc.device) } {
            log::error!("unable to swap buffers: {e}");
            return Err(crate::SurfaceError::Other("unable to swap buffers"));
        }

        Ok(())
    }

    pub fn supports_srgb(&self) -> bool {
        self.srgb_capable
    }
}

impl crate::Surface for Surface {
    type A = super::Api;

    unsafe fn configure(
        &self,
        device: &super::Device,
        config: &crate::SurfaceConfiguration,
    ) -> Result<(), crate::SurfaceError> {
        // Remove the old configuration.
        unsafe { self.unconfigure(device) };

        let dc = unsafe { Gdi::GetDC(Some(self.window)) };
        if dc.is_invalid() {
            log::error!(
                "unable to get the device context from window: {}",
                Error::from_thread()
            );
            return Err(crate::SurfaceError::Other(
                "unable to get the device context from window",
            ));
        }
        let dc = DeviceContextHandle {
            device: dc,
            window: self.window,
        };

        if let Err(e) = unsafe { setup_pixel_format(dc.device) } {
            log::error!("unable to setup surface pixel format: {e}",);
            return Err(crate::SurfaceError::Other(
                "unable to setup surface pixel format",
            ));
        }

        let format_desc = device.shared.describe_texture_format(config.format);
        let gl = &device.shared.context.lock_with_dc(dc.device).map_err(|e| {
            log::error!("unable to make the OpenGL context current for surface: {e}",);
            crate::SurfaceError::Other("unable to make the OpenGL context current for surface")
        })?;

        let renderbuffer = unsafe { gl.create_renderbuffer() }.map_err(|error| {
            log::error!("Internal swapchain renderbuffer creation failed: {error}");
            crate::DeviceError::OutOfMemory
        })?;
        unsafe { gl.bind_renderbuffer(glow::RENDERBUFFER, Some(renderbuffer)) };
        unsafe {
            gl.renderbuffer_storage(
                glow::RENDERBUFFER,
                format_desc.internal,
                config.extent.width as _,
                config.extent.height as _,
            )
        };

        let framebuffer = unsafe { gl.create_framebuffer() }.map_err(|error| {
            log::error!("Internal swapchain framebuffer creation failed: {error}");
            crate::DeviceError::OutOfMemory
        })?;
        unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(framebuffer)) };
        unsafe {
            gl.framebuffer_renderbuffer(
                glow::READ_FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::RENDERBUFFER,
                Some(renderbuffer),
            )
        };
        unsafe { gl.bind_renderbuffer(glow::RENDERBUFFER, None) };
        unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, None) };

        // Setup presentation mode
        let extra = Wgl::load_with(|name| load_gl_func(name, None));
        let extensions = get_extensions(&extra, dc.device);
        if !(extensions.contains("WGL_EXT_swap_control") && extra.SwapIntervalEXT.is_loaded()) {
            log::error!("WGL_EXT_swap_control is unsupported");
            return Err(crate::SurfaceError::Other(
                "WGL_EXT_swap_control is unsupported",
            ));
        }

        let vsync = match config.present_mode {
            wgt::PresentMode::Immediate => false,
            wgt::PresentMode::Fifo => true,
            _ => {
                log::error!("unsupported present mode: {:?}", config.present_mode);
                return Err(crate::SurfaceError::Other("unsupported present mode"));
            }
        };

        if unsafe { extra.SwapIntervalEXT(if vsync { 1 } else { 0 }) } == Foundation::FALSE.0 {
            log::error!("unable to set swap interval: {}", Error::from_thread());
            return Err(crate::SurfaceError::Other("unable to set swap interval"));
        }

        self.swapchain.write().replace(Swapchain {
            renderbuffer,
            framebuffer,
            extent: config.extent,
            format: config.format,
            format_desc,
            sample_type: wgt::TextureSampleType::Float { filterable: false },
        });

        Ok(())
    }

    unsafe fn unconfigure(&self, device: &super::Device) {
        let gl = &device.shared.context.lock();
        if let Some(sc) = self.swapchain.write().take() {
            unsafe {
                gl.delete_renderbuffer(sc.renderbuffer);
                gl.delete_framebuffer(sc.framebuffer)
            };
        }
    }

    unsafe fn acquire_texture(
        &self,
        _timeout_ms: Option<Duration>,
        _fence: &super::Fence,
    ) -> Result<crate::AcquiredSurfaceTexture<super::Api>, crate::SurfaceError> {
        let swapchain = self.swapchain.read();
        let sc = swapchain.as_ref().unwrap();
        let texture = super::Texture {
            inner: super::TextureInner::Renderbuffer {
                raw: sc.renderbuffer,
            },
            drop_guard: None,
            array_layer_count: 1,
            mip_level_count: 1,
            format: sc.format,
            format_desc: sc.format_desc.clone(),
            copy_size: crate::CopyExtent {
                width: sc.extent.width,
                height: sc.extent.height,
                depth: 1,
            },
        };
        Ok(crate::AcquiredSurfaceTexture {
            texture,
            suboptimal: false,
        })
    }
    unsafe fn discard_texture(&self, _texture: super::Texture) {}
}
