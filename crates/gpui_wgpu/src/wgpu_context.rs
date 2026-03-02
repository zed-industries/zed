#[cfg(not(target_family = "wasm"))]
use anyhow::Context as _;
#[cfg(not(target_family = "wasm"))]
use gpui_util::ResultExt;
use std::sync::Arc;

pub struct WgpuContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    dual_source_blending: bool,
}

impl WgpuContext {
    #[cfg(not(target_family = "wasm"))]
    pub fn new(instance: wgpu::Instance, surface: &wgpu::Surface<'_>) -> anyhow::Result<Self> {
        let device_id_filter = match std::env::var("ZED_DEVICE_ID") {
            Ok(val) => parse_pci_id(&val)
                .context("Failed to parse device ID from `ZED_DEVICE_ID` environment variable")
                .log_err(),
            Err(std::env::VarError::NotPresent) => None,
            err => {
                err.context("Failed to read value of `ZED_DEVICE_ID` environment variable")
                    .log_err();
                None
            }
        };

        let adapter = pollster::block_on(Self::select_adapter(
            &instance,
            device_id_filter,
            Some(surface),
        ))?;

        let caps = surface.get_capabilities(&adapter);
        if caps.formats.is_empty() {
            let info = adapter.get_info();
            anyhow::bail!(
                "No adapter compatible with the display surface could be found. \
                 Best candidate {:?} (backend={:?}, device={:#06x}) reports no \
                 supported surface formats.",
                info.name,
                info.backend,
                info.device,
            );
        }

        log::info!(
            "Selected GPU adapter: {:?} ({:?})",
            adapter.get_info().name,
            adapter.get_info().backend
        );

        let (device, queue, dual_source_blending) =
            pollster::block_on(Self::create_device(&adapter))?;

        Ok(Self {
            instance,
            adapter,
            device: Arc::new(device),
            queue: Arc::new(queue),
            dual_source_blending,
        })
    }

    #[cfg(target_family = "wasm")]
    pub async fn new_web() -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
            flags: wgpu::InstanceFlags::default(),
            backend_options: wgpu::BackendOptions::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to request GPU adapter: {e}"))?;

        log::info!(
            "Selected GPU adapter: {:?} ({:?})",
            adapter.get_info().name,
            adapter.get_info().backend
        );

        let (device, queue, dual_source_blending) = Self::create_device(&adapter).await?;

        Ok(Self {
            instance,
            adapter,
            device: Arc::new(device),
            queue: Arc::new(queue),
            dual_source_blending,
        })
    }

    async fn create_device(
        adapter: &wgpu::Adapter,
    ) -> anyhow::Result<(wgpu::Device, wgpu::Queue, bool)> {
        let dual_source_blending = adapter
            .features()
            .contains(wgpu::Features::DUAL_SOURCE_BLENDING);

        let mut required_features = wgpu::Features::empty();
        if dual_source_blending {
            required_features |= wgpu::Features::DUAL_SOURCE_BLENDING;
        } else {
            log::warn!(
                "Dual-source blending not available on this GPU. \
                Subpixel text antialiasing will be disabled."
            );
        }

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("gpui_device"),
                required_features,
                required_limits: wgpu::Limits::downlevel_defaults()
                    .using_resolution(adapter.limits())
                    .using_alignment(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: wgpu::Trace::Off,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
            })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create wgpu device: {e}"))?;

        Ok((device, queue, dual_source_blending))
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn instance() -> wgpu::Instance {
        wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN | wgpu::Backends::GL,
            flags: wgpu::InstanceFlags::default(),
            backend_options: wgpu::BackendOptions::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
        })
    }

    pub fn check_compatible_with_surface(&self, surface: &wgpu::Surface<'_>) -> anyhow::Result<()> {
        let caps = surface.get_capabilities(&self.adapter);
        if caps.formats.is_empty() {
            let info = self.adapter.get_info();
            anyhow::bail!(
                "Adapter {:?} (backend={:?}, device={:#06x}) is not compatible with the \
                 display surface for this window.",
                info.name,
                info.backend,
                info.device,
            );
        }
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    async fn select_adapter(
        instance: &wgpu::Instance,
        device_id_filter: Option<u32>,
        compatible_surface: Option<&wgpu::Surface<'_>>,
    ) -> anyhow::Result<wgpu::Adapter> {
        if let Some(device_id) = device_id_filter {
            let adapters: Vec<_> = instance.enumerate_adapters(wgpu::Backends::all()).await;

            if adapters.is_empty() {
                anyhow::bail!("No GPU adapters found");
            }

            let mut non_matching_adapter_infos: Vec<wgpu::AdapterInfo> = Vec::new();

            for adapter in adapters.into_iter() {
                let info = adapter.get_info();
                if info.device == device_id {
                    if let Some(surface) = compatible_surface {
                        let caps = surface.get_capabilities(&adapter);
                        if caps.formats.is_empty() {
                            log::warn!(
                                "GPU matching ZED_DEVICE_ID={:#06x} ({}) is not compatible \
                                 with the display surface. Falling back to auto-selection.",
                                device_id,
                                info.name,
                            );
                            break;
                        }
                    }
                    log::info!(
                        "Found GPU matching ZED_DEVICE_ID={:#06x}: {}",
                        device_id,
                        info.name
                    );
                    return Ok(adapter);
                } else {
                    non_matching_adapter_infos.push(info);
                }
            }

            log::warn!(
                "No compatible GPU found matching ZED_DEVICE_ID={:#06x}. Available devices:",
                device_id
            );

            for info in &non_matching_adapter_infos {
                log::warn!(
                    "  - {} (device_id={:#06x}, backend={})",
                    info.name,
                    info.device,
                    info.backend
                );
            }
        }

        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface,
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to request GPU adapter: {e}"))
    }

    pub fn supports_dual_source_blending(&self) -> bool {
        self.dual_source_blending
    }
}

#[cfg(not(target_family = "wasm"))]
fn parse_pci_id(id: &str) -> anyhow::Result<u32> {
    let mut id = id.trim();

    if id.starts_with("0x") || id.starts_with("0X") {
        id = &id[2..];
    }
    let is_hex_string = id.chars().all(|c| c.is_ascii_hexdigit());
    let is_4_chars = id.len() == 4;
    anyhow::ensure!(
        is_4_chars && is_hex_string,
        "Expected a 4 digit PCI ID in hexadecimal format"
    );

    u32::from_str_radix(id, 16).context("parsing PCI ID as hex")
}

#[cfg(test)]
mod tests {
    use super::parse_pci_id;

    #[test]
    fn test_parse_device_id() {
        assert!(parse_pci_id("0xABCD").is_ok());
        assert!(parse_pci_id("ABCD").is_ok());
        assert!(parse_pci_id("abcd").is_ok());
        assert!(parse_pci_id("1234").is_ok());
        assert!(parse_pci_id("123").is_err());
        assert_eq!(
            parse_pci_id(&format!("{:x}", 0x1234)).unwrap(),
            parse_pci_id(&format!("{:X}", 0x1234)).unwrap(),
        );

        assert_eq!(
            parse_pci_id(&format!("{:#x}", 0x1234)).unwrap(),
            parse_pci_id(&format!("{:#X}", 0x1234)).unwrap(),
        );
    }
}
