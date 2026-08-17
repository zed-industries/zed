use alloc::sync::Arc;
use core::{ffi, ptr};

use once_cell::sync::Lazy;
use windows::{
    core::Interface as _,
    Win32::{Foundation::HWND, Graphics::DirectComposition},
};

use super::DynLib;

// Lazy-loaded DirectComposition library
#[derive(Debug)]
pub(crate) struct DCompLib {
    lib: Lazy<Result<DynLib, crate::SurfaceError>>,
}

impl DCompLib {
    pub(crate) fn new() -> Self {
        Self {
            lib: Lazy::new(|| unsafe {
                DynLib::new("dcomp.dll").map_err(|err| {
                    log::error!("Error loading dcomp.dll: {err}");
                    crate::SurfaceError::Other("Error loading dcomp.dll")
                })
            }),
        }
    }

    fn get_lib(&self) -> Result<&DynLib, crate::SurfaceError> {
        match self.lib.as_ref() {
            Ok(lib) => Ok(lib),
            Err(err) => Err(err.clone()),
        }
    }

    pub(crate) fn create_device(
        &self,
    ) -> Result<DirectComposition::IDCompositionDevice, crate::SurfaceError> {
        let lib = self.get_lib()?;

        // Calls windows::Win32::Graphics::DirectComposition::DCompositionCreateDevice2 on dcomp.dll
        type Fun = extern "system" fn(
            pdxdevice: *mut ffi::c_void,
            riid: *const windows_core::GUID,
            ppdcompdevice: *mut *mut ffi::c_void,
        ) -> windows_core::HRESULT;
        let func: libloading::Symbol<Fun> =
            unsafe { lib.get(c"DCompositionCreateDevice2".to_bytes()) }?;

        let mut res: Option<DirectComposition::IDCompositionDevice> = None;

        (func)(
            ptr::null_mut(),
            &DirectComposition::IDCompositionDevice::IID,
            <*mut _>::cast(&mut res),
        )
        .map(|| res.unwrap())
        .map_err(|err| {
            log::error!("DirectComposition::DCompositionCreateDevice2 failed: {err}");
            crate::SurfaceError::Other("DirectComposition::DCompositionCreateDevice2")
        })
    }
}

#[derive(Default)]
pub struct DCompState {
    inner: Option<InnerState>,
}

impl DCompState {
    /// This will create a DirectComposition device and a target for the window handle if not already initialized.
    /// If the device is already initialized, it will return the existing state.
    pub unsafe fn get_or_init(
        &mut self,
        lib: &Arc<DCompLib>,
        hwnd: &HWND,
    ) -> Result<&mut InnerState, crate::SurfaceError> {
        if self.inner.is_none() {
            self.inner = Some(unsafe { InnerState::init(lib, hwnd) }?);
        }
        Ok(self.inner.as_mut().unwrap())
    }
}

pub struct InnerState {
    pub visual: DirectComposition::IDCompositionVisual,
    pub device: DirectComposition::IDCompositionDevice,
    // Must be kept alive but is otherwise unused after initialization.
    pub _target: DirectComposition::IDCompositionTarget,
}

impl InnerState {
    /// Creates a DirectComposition device and a target for the given window handle.
    pub unsafe fn init(lib: &Arc<DCompLib>, hwnd: &HWND) -> Result<Self, crate::SurfaceError> {
        profiling::scope!("DCompState::init");
        let dcomp_device = lib.create_device()?;

        let target = unsafe { dcomp_device.CreateTargetForHwnd(*hwnd, false) }.map_err(|err| {
            log::error!("IDCompositionDevice::CreateTargetForHwnd failed: {err}");
            crate::SurfaceError::Other("IDCompositionDevice::CreateTargetForHwnd")
        })?;

        let visual = unsafe { dcomp_device.CreateVisual() }.map_err(|err| {
            log::error!("IDCompositionDevice::CreateVisual failed: {err}");
            crate::SurfaceError::Other("IDCompositionDevice::CreateVisual")
        })?;

        unsafe { target.SetRoot(&visual) }.map_err(|err| {
            log::error!("IDCompositionTarget::SetRoot failed: {err}");
            crate::SurfaceError::Other("IDCompositionTarget::SetRoot")
        })?;

        Ok(InnerState {
            visual,
            device: dcomp_device,
            _target: target,
        })
    }
}
