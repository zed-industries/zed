//! Disassembly support for pulley bytecode.

use crate::decode::*;
use crate::imms::*;
use crate::regs::*;
use alloc::string::String;
use core::fmt::Write;

/// A Pulley bytecode disassembler.
///
/// This is implemented as an `OpVisitor`, where you pass a `Disassembler` to a
/// `Decoder` in order to disassemble instructions from a bytecode stream.
///
/// Alternatively, you can use the `Disassembler::disassemble_all` method to
/// disassemble a complete bytecode stream.
pub struct Disassembler<'a> {
    raw_bytecode: &'a [u8],
    bytecode: SafeBytecodeStream<'a>,
    disas: String,
    start_offset: usize,
    start: usize,
    temp: String,
    offsets: bool,
    hexdump: bool,
    br_tables: bool,
}

impl<'a> Disassembler<'a> {
    /// Disassemble every instruction in the given bytecode stream.
    pub fn disassemble_all(bytecode: &'a [u8]) -> Result<String> {
        let mut disas = Self::new(bytecode);
        Decoder::decode_all(&mut disas)?;
        Ok(disas.disas)
    }

    /// Create a new `Disassembler` that can be used to incrementally
    /// disassemble instructions from the given bytecode stream.
    pub fn new(bytecode: &'a [u8]) -> Self {
        Self {
            raw_bytecode: bytecode,
            bytecode: SafeBytecodeStream::new(bytecode),
            disas: String::new(),
            start: 0,
            start_offset: 0,
            temp: String::new(),
            offsets: true,
            hexdump: true,
            br_tables: true,
        }
    }

    /// Whether to prefix each instruction's disassembly with its offset.
    ///
    /// True by default.
    pub fn offsets(&mut self, offsets: bool) -> &mut Self {
        self.offsets = offsets;
        self
    }

    /// Whether to include a hexdump of the bytecode in the disassembly.
    ///
    /// True by default.
    pub fn hexdump(&mut self, hexdump: bool) -> &mut Self {
        self.hexdump = hexdump;
        self
    }

    /// Whether to include branche tables in the disassembly.
    ///
    /// True by default.
    pub fn br_tables(&mut self, enable: bool) -> &mut Self {
        self.br_tables = enable;
        self
    }

    /// Configures the offset that this function starts from, if it doesn't
    /// start from 0.
    ///
    /// This can possibly be useful when a single function at a time is being
    /// disassembled.
    pub fn start_offset(&mut self, offset: usize) -> &mut Self {
        self.start_offset = offset;
        self
    }

    /// Get the disassembly thus far.
    pub fn disas(&self) -> &str {
        &self.disas
    }

    fn disas_op(&mut self, mnemonic: &str, operands: &[&dyn Disas]) {
        write!(&mut self.temp, "{mnemonic}").unwrap();
        for (i, val) in operands.iter().enumerate() {
            if i > 0 {
                write!(&mut self.temp, ",").unwrap();
            }
            write!(&mut self.temp, " ").unwrap();
            val.disas(self.start + self.start_offset, &mut self.temp);
        }
    }

    fn disas_br_table32(&mut self, reg: XReg, amt: u32) {
        self.disas_op("br_table32", &[&reg, &amt]);
        for _ in 0..amt {
            self.after_visit();
            self.start = self.bytecode.position();
            if let Ok(offset) = PcRelOffset::decode(self.bytecode()) {
                if self.br_tables {
                    offset.disas(self.start + self.start_offset, &mut self.temp);
                }
            }
        }
    }
}

/// Anything inside an instruction that can be disassembled: registers,
/// immediates, etc...
trait Disas {
    fn disas(&self, position: usize, disas: &mut String);
}

impl Disas for XReg {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for FReg {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for VReg {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for i8 {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for i16 {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for i32 {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for i64 {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for i128 {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for u8 {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for u16 {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for u32 {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for u64 {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for u128 {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{self}").unwrap();
    }
}

impl Disas for PcRelOffset {
    fn disas(&self, position: usize, disas: &mut String) {
        let offset = i64::from(i32::from(*self));
        let target = (position as u64).wrapping_add(offset as u64);
        let (prefix, offset) = if offset < 0 {
            ("-", -offset)
        } else {
            ("", offset)
        };
        write!(disas, "{prefix}{offset:#x}    // target = {target:#x}").unwrap()
    }
}

impl Disas for U6 {
    fn disas(&self, _position: usize, disas: &mut String) {
        write!(disas, "{}", u8::from(*self)).unwrap();
    }
}

fn disas_list<T: Disas>(position: usize, disas: &mut String, iter: impl IntoIterator<Item = T>) {
    let mut iter = iter.into_iter();
    let Some(first) = iter.next() else { return };
    first.disas(position, disas);

    for item in iter {
        write!(disas, ", ").unwrap();
        item.disas(position, disas);
    }
}

impl<D, S1, S2> Disas for BinaryOperands<D, S1, S2>
where
    D: Reg + Disas,
    S1: Reg + Disas,
    S2: Reg + Disas,
{
    fn disas(&self, position: usize, disas: &mut String) {
        self.dst.disas(position, disas);
        write!(disas, ", ").unwrap();
        self.src1.disas(position, disas);
        write!(disas, ", ").unwrap();
        self.src2.disas(position, disas);
    }
}

impl<D, S1> Disas for BinaryOperands<D, S1, U6>
where
    D: Reg + Disas,
    S1: Reg + Disas,
{
    fn disas(&self, position: usize, disas: &mut String) {
        self.dst.disas(position, disas);
        write!(disas, ", ").unwrap();
        self.src1.disas(position, disas);
        write!(disas, ", ").unwrap();
        self.src2.disas(position, disas);
    }
}

impl<R: Reg + Disas> Disas for UpperRegSet<R> {
    fn disas(&self, position: usize, disas: &mut String) {
        disas_list(position, disas, *self)
    }
}

impl Disas for AddrO32 {
    fn disas(&self, position: usize, disas: &mut String) {
        self.addr.disas(position, disas);
        write!(disas, ", ").unwrap();
        self.offset.disas(position, disas);
    }
}

impl Disas for AddrZ {
    fn disas(&self, position: usize, disas: &mut String) {
        self.addr.disas(position, disas);
        write!(disas, ", ").unwrap();
        self.offset.disas(position, disas);
    }
}

impl Disas for AddrG32 {
    fn disas(&self, position: usize, disas: &mut String) {
        self.host_heap_base.disas(position, disas);
        write!(disas, ", ").unwrap();
        self.host_heap_bound.disas(position, disas);
        write!(disas, ", ").unwrap();
        self.wasm_addr.disas(position, disas);
        write!(disas, ", ").unwrap();
        self.offset.disas(position, disas);
    }
}

impl Disas for AddrG32Bne {
    fn disas(&self, position: usize, disas: &mut String) {
        self.host_heap_base.disas(position, disas);
        write!(disas, ", *[").unwrap();
        self.host_heap_bound_addr.disas(position, disas);
        write!(disas, " + ").unwrap();
        self.host_heap_bound_offset.disas(position, disas);
        write!(disas, "], ").unwrap();
        self.wasm_addr.disas(position, disas);
        write!(disas, ", ").unwrap();
        self.offset.disas(position, disas);
    }
}

macro_rules! impl_disas {
    (
        $(
            $( #[$attr:meta] )*
                $snake_name:ident = $name:ident $( {
                $(
                    $( #[$field_attr:meta] )*
                    $field:ident : $field_ty:ty
                ),*
            } )? ;
        )*
    ) => {
        $(
            impl_disas!(@one $snake_name = $name $( { $($field: $field_ty),* } )?);
        )*
    };

    // Diassembling `br_table` is a bit special as it has trailing byte after
    // the opcode of the branch table itself.
    (
        @one br_table32 = BrTable32 $( {
            $(
                $field:ident : $field_ty:ty
            ),*
        } )?
    ) => {
        fn br_table32(&mut self $( $( , $field : $field_ty )* )? ) {
            self.disas_br_table32($($($field),*)?)
        }
    };

    // All other opcodes other than `br_table` are handled in the same manner.
    (
        @one $snake_name:ident = $name:ident $( {
            $(
                $field:ident : $field_ty:ty
            ),*
        } )?
    ) => {
        fn $snake_name(&mut self $( $( , $field : $field_ty )* )? ) {
            self.disas_op(stringify!($snake_name), &[$($(&$field),*)?])
        }
    };
}

impl<'a> OpVisitor for Disassembler<'a> {
    type BytecodeStream = SafeBytecodeStream<'a>;

    fn bytecode(&mut self) -> &mut Self::BytecodeStream {
        &mut self.bytecode
    }

    type Return = ();

    fn before_visit(&mut self) {
        self.start = self.bytecode.position();
    }

    fn after_visit(&mut self) {
        if self.offsets {
            write!(&mut self.disas, "{:8x}: ", self.start + self.start_offset).unwrap();
        }
        if self.hexdump {
            let size = self.bytecode.position() - self.start;
            let mut need_space = false;
            for byte in &self.raw_bytecode[self.start..][..size] {
                let space = if need_space { " " } else { "" };
                write!(&mut self.disas, "{space}{byte:02x}").unwrap();
                need_space = true;
            }
            for _ in 0..12_usize.saturating_sub(size) {
                write!(&mut self.disas, "   ").unwrap();
            }
        }
        self.disas.push_str(&self.temp);
        self.temp.clear();

        self.disas.push('\n');
    }

    for_each_op!(impl_disas);
}

impl ExtendedOpVisitor for Disassembler<'_> {
    for_each_extended_op!(impl_disas);
}
