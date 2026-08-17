//! A WebAssembly encoder.
//!
//! The main builder is the [`Module`]. You can build a section with a
//! section-specific builder, like [`TypeSection`] or [`ImportSection`], and
//! then add it to the module with [`Module::section`]. When you are finished
//! building the module, call either [`Module::as_slice`] or [`Module::finish`]
//! to get the encoded bytes. The former gives a shared reference to the
//! underlying bytes as a slice, while the latter gives you ownership of them as
//! a vector.
//!
//! # Example
//!
//! If we wanted to build this module:
//!
//! ```wasm
//! (module
//!   (type (func (param i32 i32) (result i32)))
//!   (func (type 0)
//!     local.get 0
//!     local.get 1
//!     i32.add)
//!   (export "f" (func 0)))
//! ```
//!
//! then we would do this:
//!
//! ```
//! use wasm_encoder::{
//!     CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction,
//!     Module, TypeSection, ValType,
//! };
//!
//! let mut module = Module::new();
//!
//! // Encode the type section.
//! let mut types = TypeSection::new();
//! let params = vec![ValType::I32, ValType::I32];
//! let results = vec![ValType::I32];
//! types.function(params, results);
//! module.section(&types);
//!
//! // Encode the function section.
//! let mut functions = FunctionSection::new();
//! let type_index = 0;
//! functions.function(type_index);
//! module.section(&functions);
//!
//! // Encode the export section.
//! let mut exports = ExportSection::new();
//! exports.export("f", ExportKind::Func, 0);
//! module.section(&exports);
//!
//! // Encode the code section.
//! let mut codes = CodeSection::new();
//! let locals = vec![];
//! let mut f = Function::new(locals);
//! f.instruction(&Instruction::LocalGet(0));
//! f.instruction(&Instruction::LocalGet(1));
//! f.instruction(&Instruction::I32Add);
//! f.instruction(&Instruction::End);
//! codes.function(&f);
//! module.section(&codes);
//!
//! // Extract the encoded Wasm bytes for this module.
//! let wasm_bytes = module.finish();
//!
//! // We generated a valid Wasm module!
//! assert!(wasmparser::validate(&wasm_bytes).is_ok());
//! ```

#![deny(missing_docs, missing_debug_implementations)]

mod component;
mod core;
mod raw;

pub use self::component::*;
pub use self::core::*;
pub use self::raw::*;

/// Implemented by types that can be encoded into a byte sink.
pub trait Encode {
    /// Encode the type into the given byte sink.
    fn encode(&self, sink: &mut Vec<u8>);
}

impl<T: Encode + ?Sized> Encode for &'_ T {
    fn encode(&self, sink: &mut Vec<u8>) {
        T::encode(self, sink)
    }
}

impl<T: Encode> Encode for [T] {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.len().encode(sink);
        for item in self {
            item.encode(sink);
        }
    }
}

impl Encode for [u8] {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.len().encode(sink);
        sink.extend(self);
    }
}

impl Encode for str {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.len().encode(sink);
        sink.extend_from_slice(self.as_bytes());
    }
}

impl Encode for usize {
    fn encode(&self, sink: &mut Vec<u8>) {
        assert!(*self <= u32::max_value() as usize);
        (*self as u32).encode(sink)
    }
}

impl Encode for u32 {
    fn encode(&self, sink: &mut Vec<u8>) {
        leb128::write::unsigned(sink, (*self).into()).unwrap();
    }
}

impl Encode for i32 {
    fn encode(&self, sink: &mut Vec<u8>) {
        leb128::write::signed(sink, (*self).into()).unwrap();
    }
}

impl Encode for u64 {
    fn encode(&self, sink: &mut Vec<u8>) {
        leb128::write::unsigned(sink, *self).unwrap();
    }
}

impl Encode for i64 {
    fn encode(&self, sink: &mut Vec<u8>) {
        leb128::write::signed(sink, *self).unwrap();
    }
}

impl Encode for f32 {
    fn encode(&self, sink: &mut Vec<u8>) {
        let bits = self.to_bits();
        sink.extend(bits.to_le_bytes())
    }
}

impl Encode for f64 {
    fn encode(&self, sink: &mut Vec<u8>) {
        let bits = self.to_bits();
        sink.extend(bits.to_le_bytes())
    }
}

fn encode_vec<T, V>(elements: V, sink: &mut Vec<u8>)
where
    T: Encode,
    V: IntoIterator<Item = T>,
    V::IntoIter: ExactSizeIterator,
{
    let elements = elements.into_iter();
    u32::try_from(elements.len()).unwrap().encode(sink);
    for x in elements {
        x.encode(sink);
    }
}

impl<T> Encode for Option<T>
where
    T: Encode,
{
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            Some(v) => {
                sink.push(0x01);
                v.encode(sink);
            }
            None => sink.push(0x00),
        }
    }
}

fn encoding_size(n: u32) -> usize {
    let mut buf = [0u8; 5];
    leb128::write::unsigned(&mut &mut buf[..], n.into()).unwrap()
}

fn encode_section(sink: &mut Vec<u8>, count: u32, bytes: &[u8]) {
    (encoding_size(count) + bytes.len()).encode(sink);
    count.encode(sink);
    sink.extend(bytes);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_encodes_an_empty_module() {
        let bytes = Module::new().finish();
        assert_eq!(bytes, [0x00, b'a', b's', b'm', 0x01, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn it_encodes_an_empty_component() {
        let bytes = Component::new().finish();
        assert_eq!(bytes, [0x00, b'a', b's', b'm', 0x0d, 0x00, 0x01, 0x00]);
    }
}
