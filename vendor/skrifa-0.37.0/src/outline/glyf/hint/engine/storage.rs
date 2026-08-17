//! Managing the storage area.
//!
//! Implements 2 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#managing-the-storage-area>

use super::{Engine, OpResult};

impl Engine<'_> {
    /// Read store.
    ///
    /// RS[] (0x43)
    ///
    /// Pops: location: Storage Area location
    /// Pushes: value: Storage Area value
    ///
    /// This instruction reads a 32 bit value from the Storage Area location
    /// popped from the stack and pushes the value read onto the stack. It pops
    /// an address from the stack and pushes the value found in that Storage
    /// Area location to the top of the stack. The number of available storage
    /// locations is specified in the maxProfile table in the font file.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#read-store>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2975>
    pub(super) fn op_rs(&mut self) -> OpResult {
        let location = self.value_stack.pop_usize()?;
        let maybe_value = self.storage.get(location);
        let value = if self.graphics.is_pedantic {
            maybe_value?
        } else {
            maybe_value.unwrap_or(0)
        };
        self.value_stack.push(value)
    }

    /// Write store.
    ///
    /// WS[] (0x42)
    ///
    /// Pops: value: Storage Area value,
    ///       location: Storage Area location
    ///
    /// This instruction writes a 32 bit value into the storage location
    /// indexed by locations. It works by popping a value and then a location
    /// from the stack. The value is placed in the Storage Area location
    /// specified by that address. The number of storage locations is specified
    /// in the maxProfile table in the font file.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#write-store>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3000>
    pub(super) fn op_ws(&mut self) -> OpResult {
        let value = self.value_stack.pop()?;
        let location = self.value_stack.pop_usize()?;
        let result = self.storage.set(location, value);
        if self.graphics.is_pedantic {
            result
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{HintErrorKind, MockEngine};

    #[test]
    fn write_read() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        for i in 0..8 {
            engine.value_stack.push(i).unwrap();
            engine.value_stack.push(i * 2).unwrap();
            engine.op_ws().unwrap();
        }
        for i in 0..8 {
            engine.value_stack.push(i).unwrap();
            engine.op_rs().unwrap();
            assert_eq!(engine.value_stack.pop().unwrap(), i * 2);
        }
    }

    #[test]
    fn pedantry() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        let oob_index = 1000;
        // Disable pedantic mode: OOB writes are ignored, OOB reads
        // push 0
        engine.graphics.is_pedantic = false;
        engine.value_stack.push(oob_index).unwrap();
        engine.value_stack.push(0).unwrap();
        engine.op_ws().unwrap();
        engine.value_stack.push(oob_index).unwrap();
        engine.op_rs().unwrap();
        // Enable pedantic mode: OOB reads/writes error
        engine.graphics.is_pedantic = true;
        engine.value_stack.push(oob_index).unwrap();
        engine.value_stack.push(0).unwrap();
        assert_eq!(
            engine.op_ws(),
            Err(HintErrorKind::InvalidStorageIndex(oob_index as _))
        );
        engine.value_stack.push(oob_index).unwrap();
        assert_eq!(
            engine.op_rs(),
            Err(HintErrorKind::InvalidStorageIndex(oob_index as _))
        );
    }
}
