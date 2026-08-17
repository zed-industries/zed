// Take a look at the license at the top of the repository in the LICENSE file.

pub(crate) struct CpuInner;

impl CpuInner {
    pub(crate) fn cpu_usage(&self) -> f32 {
        0.0
    }

    pub(crate) fn name(&self) -> &str {
        ""
    }

    pub(crate) fn frequency(&self) -> u64 {
        0
    }

    pub(crate) fn vendor_id(&self) -> &str {
        ""
    }

    pub(crate) fn brand(&self) -> &str {
        ""
    }
}
