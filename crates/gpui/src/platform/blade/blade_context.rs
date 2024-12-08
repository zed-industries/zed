use blade_graphics as gpu;
use std::sync::Arc;

#[cfg_attr(target_os = "macos", derive(Clone))]
pub struct BladeContext {
    pub(super) gpu: Arc<gpu::Context>,
}

impl BladeContext {
    pub fn new() -> anyhow::Result<Self> {
        let gpu = Arc::new(
            unsafe {
                gpu::Context::init(gpu::ContextDesc {
                    presentation: true,
                    validation: false,
                    device_id: 0, //TODO: hook up to user settings
                    ..Default::default()
                })
            }
            .map_err(|e| anyhow::anyhow!("{:?}", e))?,
        );
        Ok(Self { gpu })
    }
}
