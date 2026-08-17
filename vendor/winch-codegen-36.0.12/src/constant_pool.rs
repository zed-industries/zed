//! An ISA-independent constant pool.

use cranelift_codegen::{
    MachBuffer, VCodeConstant, VCodeConstantData, VCodeConstants, VCodeInst, ir,
};

pub(crate) struct ConstantPool {
    inner: ir::ConstantPool,
    constants: VCodeConstants,
}

impl ConstantPool {
    pub fn new() -> Self {
        Self {
            inner: ir::ConstantPool::new(),
            constants: Default::default(),
        }
    }

    /// Register a constant and return a handle, ready for emission.
    pub fn register<I: VCodeInst>(
        &mut self,
        data: &[u8],
        buffer: &mut MachBuffer<I>,
    ) -> VCodeConstant {
        let constant_handle = self.inner.insert(data.into());
        let constant_data = self.inner.get(constant_handle);

        // NB: The insertion will only happen if the pool doesn't already use the constant data.
        // The reason why the order of operations is apparently inversed is to be sure to insert
        // the `VCodeConstantData` in the `MachBuffer` only once, as no deduplication happens at
        // such layer.
        let vcode_constant_data = VCodeConstantData::Pool(constant_handle, constant_data.clone());
        let must_register = !self.constants.pool_uses(&vcode_constant_data);
        let vcode_constant = self.constants.insert(VCodeConstantData::Pool(
            constant_handle,
            constant_data.clone(),
        ));

        if must_register {
            buffer.register_constant(&vcode_constant, &vcode_constant_data);
        }
        vcode_constant
    }

    /// Get the finalized constants.
    pub fn constants(self) -> VCodeConstants {
        self.constants
    }
}
