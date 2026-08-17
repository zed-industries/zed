use crate::Decode;
use compression_core::{unshared::Unshared, util::PartialBuffer};
use lz4::liblz4::{
    check_error, LZ4FDecompressionContext, LZ4F_createDecompressionContext, LZ4F_decompress,
    LZ4F_freeDecompressionContext, LZ4F_resetDecompressionContext, LZ4F_VERSION,
};
use std::io::Result;

#[derive(Debug)]
struct DecoderContext {
    ctx: LZ4FDecompressionContext,
}

#[derive(Debug)]
pub struct Lz4Decoder {
    ctx: Unshared<DecoderContext>,
}

impl DecoderContext {
    fn new() -> Result<Self> {
        let mut context = LZ4FDecompressionContext(core::ptr::null_mut());
        check_error(unsafe { LZ4F_createDecompressionContext(&mut context, LZ4F_VERSION) })?;
        Ok(Self { ctx: context })
    }
}

impl Drop for DecoderContext {
    fn drop(&mut self) {
        unsafe { LZ4F_freeDecompressionContext(self.ctx) };
    }
}

impl Default for Lz4Decoder {
    fn default() -> Self {
        Self {
            ctx: Unshared::new(DecoderContext::new().unwrap()),
        }
    }
}

impl Lz4Decoder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Decode for Lz4Decoder {
    fn reinit(&mut self) -> Result<()> {
        unsafe { LZ4F_resetDecompressionContext(self.ctx.get_mut().ctx) };
        Ok(())
    }

    fn decode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        let mut output_size = output.unwritten().len();
        let mut input_size = input.unwritten().len();
        let remaining = unsafe {
            check_error(LZ4F_decompress(
                self.ctx.get_mut().ctx,
                output.unwritten_mut().as_mut_ptr(),
                &mut output_size,
                input.unwritten().as_ptr(),
                &mut input_size,
                core::ptr::null(),
            ))?
        };
        input.advance(input_size);
        output.advance(output_size);
        Ok(remaining == 0)
    }

    fn flush(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        self.decode(&mut PartialBuffer::new(&[][..]), output)?;

        loop {
            let old_len = output.written().len();
            self.decode(&mut PartialBuffer::new(&[][..]), output)?;
            if output.written().len() == old_len {
                break;
            }
        }

        Ok(!output.unwritten().is_empty())
    }

    fn finish(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        self.flush(output)
    }
}
