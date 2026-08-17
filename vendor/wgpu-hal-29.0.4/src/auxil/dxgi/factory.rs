use alloc::{string::String, vec::Vec};
use core::ops::Deref;

use windows::{core::Interface as _, Win32::Graphics::Dxgi};

use crate::dx12::DxgiLib;

use super::result::HResult as _;

// We can rely on the presence of DXGI 1.4 since D3D12 requires WDDM 2.0, Windows 10 (1507), and so does DXGI 1.4.

fn should_keep_adapter(adapter: &Dxgi::IDXGIAdapter1) -> bool {
    let desc = unsafe { adapter.GetDesc1() }.unwrap();

    // The Intel Haswell family of iGPUs had support for the D3D12 API but it was later
    // removed due to a security vulnerability.
    //
    // We are explicitly filtering out all the devices in the family because we are now
    // getting reports of device loss at a later time than at device creation time (`D3D12CreateDevice`).
    //
    // See https://www.intel.com/content/www/us/en/support/articles/000057520/graphics.html
    // This list of device IDs is from https://dgpu-docs.intel.com/devices/hardware-table.html
    let haswell_device_ids = [
        0x0422, 0x0426, 0x042A, 0x042B, 0x042E, 0x0C22, 0x0C26, 0x0C2A, 0x0C2B, 0x0C2E, 0x0A22,
        0x0A2A, 0x0A2B, 0x0D2A, 0x0D2B, 0x0D2E, 0x0A26, 0x0A2E, 0x0D22, 0x0D26, 0x0412, 0x0416,
        0x0D12, 0x041A, 0x041B, 0x0C12, 0x0C16, 0x0C1A, 0x0C1B, 0x0C1E, 0x0A12, 0x0A1A, 0x0A1B,
        0x0D16, 0x0D1A, 0x0D1B, 0x0D1E, 0x041E, 0x0A16, 0x0A1E, 0x0402, 0x0406, 0x040A, 0x040B,
        0x040E, 0x0C02, 0x0C06, 0x0C0A, 0x0C0B, 0x0C0E, 0x0A02, 0x0A06, 0x0A0A, 0x0A0B, 0x0A0E,
        0x0D02, 0x0D06, 0x0D0A, 0x0D0B, 0x0D0E,
    ];
    if desc.VendorId == 0x8086 && haswell_device_ids.contains(&desc.DeviceId) {
        return false;
    }

    // If run completely headless, windows will show two different WARP adapters, one
    // which is lying about being an integrated card. This is so that programs
    // that ignore software adapters will actually run on headless/gpu-less machines.
    //
    // We don't want that and discourage that kind of filtering anyway, so we skip the integrated WARP.
    if desc.VendorId == 5140
        && !Dxgi::DXGI_ADAPTER_FLAG(desc.Flags as i32).contains(Dxgi::DXGI_ADAPTER_FLAG_SOFTWARE)
    {
        let adapter_name = super::conv::map_adapter_name(desc.Description);
        if adapter_name.contains("Microsoft Basic Render Driver") {
            return false;
        }
    }

    true
}

#[derive(Clone, Debug)]
pub enum DxgiAdapter {
    /// Provided by DXGI 1.4
    Adapter3(Dxgi::IDXGIAdapter3),
    /// Provided by DXGI 1.6
    Adapter4(Dxgi::IDXGIAdapter4),
}

impl DxgiAdapter {
    pub fn query_video_memory_info(
        &self,
        group: Dxgi::DXGI_MEMORY_SEGMENT_GROUP,
    ) -> Result<Dxgi::DXGI_QUERY_VIDEO_MEMORY_INFO, crate::DeviceError> {
        let mut info = Dxgi::DXGI_QUERY_VIDEO_MEMORY_INFO::default();
        unsafe { self.QueryVideoMemoryInfo(0, group, &mut info) }
            .into_device_result("QueryVideoMemoryInfo")?;
        Ok(info)
    }
}

impl Deref for DxgiAdapter {
    type Target = Dxgi::IDXGIAdapter3;

    fn deref(&self) -> &Self::Target {
        match self {
            DxgiAdapter::Adapter3(a) => a,
            DxgiAdapter::Adapter4(a) => a,
        }
    }
}

pub fn enumerate_adapters(factory: DxgiFactory) -> Vec<DxgiAdapter> {
    let mut adapters = Vec::with_capacity(8);

    for cur_index in 0.. {
        profiling::scope!("IDXGIFactory1::EnumAdapters1");
        let adapter1: Dxgi::IDXGIAdapter1 = match unsafe { factory.EnumAdapters1(cur_index) } {
            Ok(a) => a,
            Err(e) if e.code() == Dxgi::DXGI_ERROR_NOT_FOUND => break,
            Err(e) => {
                log::error!("Failed enumerating adapters: {e}");
                break;
            }
        };

        if !should_keep_adapter(&adapter1) {
            continue;
        }

        if let Ok(adapter4) = adapter1.cast::<Dxgi::IDXGIAdapter4>() {
            adapters.push(DxgiAdapter::Adapter4(adapter4));
        } else {
            let adapter3 = adapter1.cast::<Dxgi::IDXGIAdapter3>().unwrap();
            adapters.push(DxgiAdapter::Adapter3(adapter3));
        }
    }

    adapters
}

#[derive(Clone, Debug)]
pub enum DxgiFactory {
    /// Provided by DXGI 1.4
    Factory4(Dxgi::IDXGIFactory4),
    /// Provided by DXGI 1.5
    Factory5(Dxgi::IDXGIFactory5),
    /// Provided by DXGI 1.6
    Factory6(Dxgi::IDXGIFactory6),
}

impl Deref for DxgiFactory {
    type Target = Dxgi::IDXGIFactory4;

    fn deref(&self) -> &Self::Target {
        match self {
            DxgiFactory::Factory4(f) => f,
            DxgiFactory::Factory5(f) => f,
            DxgiFactory::Factory6(f) => f,
        }
    }
}

impl DxgiFactory {
    pub fn as_factory5(&self) -> Option<&Dxgi::IDXGIFactory5> {
        match self {
            Self::Factory4(_) => None,
            Self::Factory5(f) => Some(f),
            Self::Factory6(f) => Some(f),
        }
    }
}

pub fn create_factory(
    instance_flags: wgt::InstanceFlags,
) -> Result<(DxgiLib, DxgiFactory), crate::InstanceError> {
    let lib_dxgi = DxgiLib::new().map_err(|e| {
        crate::InstanceError::with_source(String::from("failed to load dxgi.dll"), e)
    })?;

    let mut factory_flags = Dxgi::DXGI_CREATE_FACTORY_FLAGS::default();

    if instance_flags.contains(wgt::InstanceFlags::VALIDATION) {
        // The `DXGI_CREATE_FACTORY_DEBUG` flag is only allowed to be passed to
        // `CreateDXGIFactory2` if the debug interface is actually available. So
        // we check for whether it exists first.
        if let Ok(Some(_)) = lib_dxgi.debug_interface1() {
            factory_flags |= Dxgi::DXGI_CREATE_FACTORY_DEBUG;
        }
    }

    let factory4 = match lib_dxgi.create_factory4(factory_flags) {
        Ok(factory) => factory,
        Err(err) => {
            return Err(crate::InstanceError::with_source(
                String::from("IDXGIFactory4 creation failed"),
                err,
            ));
        }
    };

    if let Ok(factory6) = factory4.cast::<Dxgi::IDXGIFactory6>() {
        return Ok((lib_dxgi, DxgiFactory::Factory6(factory6)));
    }

    if let Ok(factory5) = factory4.cast::<Dxgi::IDXGIFactory5>() {
        return Ok((lib_dxgi, DxgiFactory::Factory5(factory5)));
    }

    Ok((lib_dxgi, DxgiFactory::Factory4(factory4)))
}
