use crate::MemoryIndex;
use crate::component::Transcode;
use crate::fact::core_types::CoreTypes;
use crate::prelude::*;
use wasm_encoder::{EntityType, ValType};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Transcoder {
    pub from_memory: MemoryIndex,
    pub from_memory64: bool,
    pub to_memory: MemoryIndex,
    pub to_memory64: bool,
    pub op: Transcode,
}

impl Transcoder {
    pub fn name(&self) -> String {
        format!(
            "{} (mem{} => mem{})",
            self.op.desc(),
            self.from_memory.as_u32(),
            self.to_memory.as_u32(),
        )
    }

    pub fn ty(&self, types: &mut CoreTypes) -> EntityType {
        let from_ptr = if self.from_memory64 {
            ValType::I64
        } else {
            ValType::I32
        };
        let to_ptr = if self.to_memory64 {
            ValType::I64
        } else {
            ValType::I32
        };

        let ty = match self.op {
            // These direct transcodings take the source pointer, the source
            // code units, and the destination pointer.
            //
            // The memories being copied between are part of each intrinsic and
            // the destination code units are the same as the source.
            // Note that the pointers are dynamically guaranteed to be aligned
            // and in-bounds for the code units length as defined by the string
            // encoding.
            Transcode::Copy(_) | Transcode::Latin1ToUtf16 => {
                types.function(&[from_ptr, from_ptr, to_ptr], &[])
            }

            // Transcoding from utf8 to utf16 takes the from ptr/len as well as
            // a destination. The destination is valid for len*2 bytes. The
            // return value is how many code units were written to the
            // destination.
            Transcode::Utf8ToUtf16 => types.function(&[from_ptr, from_ptr, to_ptr], &[to_ptr]),

            // Transcoding to utf8 as a smaller format takes all the parameters
            // and returns the amount of space consumed in the src/destination
            Transcode::Utf16ToUtf8 | Transcode::Latin1ToUtf8 => {
                types.function(&[from_ptr, from_ptr, to_ptr, to_ptr], &[from_ptr, to_ptr])
            }

            // The return type is a tagged length which indicates which was
            // used
            Transcode::Utf16ToCompactProbablyUtf16 => {
                types.function(&[from_ptr, from_ptr, to_ptr], &[to_ptr])
            }

            // The initial step of transcoding from a fixed format to a compact
            // format. Takes the ptr/len of the source the destination
            // pointer. The destination length is implicitly the same. Returns
            // how many code units were consumed in the source, which is also
            // how many bytes were written to the destination.
            Transcode::Utf8ToLatin1 | Transcode::Utf16ToLatin1 => {
                types.function(&[from_ptr, from_ptr, to_ptr], &[from_ptr, to_ptr])
            }

            // The final step of transcoding to a compact format when the fixed
            // transcode has failed. This takes the ptr/len of the source that's
            // remaining to transcode. Then this takes the destination ptr/len
            // as well as the destination bytes written so far with latin1.
            // Finally this returns the number of code units written to the
            // destination.
            Transcode::Utf8ToCompactUtf16 | Transcode::Utf16ToCompactUtf16 => {
                types.function(&[from_ptr, from_ptr, to_ptr, to_ptr, to_ptr], &[to_ptr])
            }
        };
        EntityType::Function(ty)
    }
}
