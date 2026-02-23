use anyhow::Context as _;
use std::sync::Arc;
use util::ResultExt;

pub struct WgpuContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    dual_source_blending: bool,
}

impl WgpuContext {
    pub fn new() -> anyhow::Result<Self> {
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

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN | wgpu::Backends::GL,
            flags: wgpu::InstanceFlags::default(),
            backend_options: wgpu::BackendOptions::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
        });

        let adapter = smol::block_on(Self::select_adapter(&instance, device_id_filter))?;

        log::info!(
            "Selected GPU adapter: {:?} ({:?})",
            adapter.get_info().name,
            adapter.get_info().backend
        );

        let dual_source_blending_available = adapter
            .features()
            .contains(wgpu::Features::DUAL_SOURCE_BLENDING);

        let mut required_features = wgpu::Features::empty();
        if dual_source_blending_available {
            required_features |= wgpu::Features::DUAL_SOURCE_BLENDING;
        } else {
            log::warn!(
                "Dual-source blending not available on this GPU. \
                Subpixel text antialiasing will be disabled."
            );
        }

        let (device, queue) = smol::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("gpui_device"),
            required_features,
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::MemoryUsage,
            trace: wgpu::Trace::Off,
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
        }))
        .map_err(|e| anyhow::anyhow!("Failed to create wgpu device: {e}"))?;

        Ok(Self {
            instance,
            adapter,
            device: Arc::new(device),
            queue: Arc::new(queue),
            dual_source_blending: dual_source_blending_available,
        })
    }

    async fn select_adapter(
        instance: &wgpu::Instance,
        device_id_filter: Option<u32>,
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
                "No GPU found matching ZED_DEVICE_ID={:#06x}. Available devices:",
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
                power_preference: wgpu::PowerPreference::None,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| anyhow::anyhow!("Failed to request GPU adapter: {e}"))
    }

    pub fn supports_dual_source_blending(&self) -> bool {
        self.dual_source_blending
    }
}

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
