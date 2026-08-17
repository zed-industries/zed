//! A fuzz testing oracle for roundtrip assembly-disassembly.
//!
//! This contains manual implementations of the `Arbitrary` trait for types
//! throughout this crate to avoid depending on the `arbitrary` crate
//! unconditionally (use the `fuzz` feature instead).

use crate::{
    AmodeOffset, AmodeOffsetPlusKnownOffset, AsReg, CodeSink, DeferredTarget, Fixed, Gpr, Inst,
    KnownOffset, NonRspGpr, Registers, TrapCode, Xmm,
};
use arbitrary::{Arbitrary, Result, Unstructured};
use capstone::{Capstone, arch::BuildsCapstone, arch::BuildsCapstoneSyntax, arch::x86};

/// Take a random assembly instruction and check its encoding and
/// pretty-printing against a known-good disassembler.
///
/// # Panics
///
/// This function panics to express failure as expected by the `arbitrary`
/// fuzzer infrastructure. It may fail during assembly, disassembly, or when
/// comparing the disassembled strings.
pub fn roundtrip(inst: &Inst<FuzzRegs>) {
    // Check that we can actually assemble this instruction.
    let assembled = assemble(inst);
    let expected = disassemble(&assembled, inst);

    // Check that our pretty-printed output matches the known-good output. Trim
    // off the instruction offset first.
    let expected = expected.split_once(' ').unwrap().1;
    let actual = inst.to_string();
    if expected != actual && expected.trim() != fix_up(&actual) {
        println!("> {inst}");
        println!("  debug: {inst:x?}");
        println!("  assembled: {}", pretty_print_hexadecimal(&assembled));
        println!("  expected (capstone): {expected}");
        println!("  actual (to_string):  {actual}");
        assert_eq!(expected, &actual);
    }
}

/// Use this assembler to emit machine code into a byte buffer.
///
/// This will skip any traps or label registrations, but this is fine for the
/// single-instruction disassembly we're doing here.
fn assemble(inst: &Inst<FuzzRegs>) -> Vec<u8> {
    let mut sink = TestCodeSink::default();
    inst.encode(&mut sink);
    sink.patch_labels_as_if_they_referred_to_end();
    sink.buf
}

#[derive(Default)]
struct TestCodeSink {
    buf: Vec<u8>,
    offsets_using_label: Vec<usize>,
}

impl TestCodeSink {
    /// References to labels, e.g. RIP-relative addressing, is stored with an
    /// adjustment that takes into account the distance from the relative offset
    /// to the end of the instruction, where the offset is relative to. That
    /// means that to indeed make the offset relative to the end of the
    /// instruction, which is what we pretend all labels are bound to, it's
    /// required that this adjustment is taken into account.
    ///
    /// This function will iterate over all labels bound to this code sink and
    /// pretend the label is found at the end of the `buf`. That means that the
    /// distance from the label to the end of `buf` minus 4, which is the width
    /// of the offset, is added to what's already present in the encoding buffer.
    ///
    /// This is effectively undoing the `bytes_at_end` adjustment that's part of
    /// `Amode::RipRelative` addressing.
    fn patch_labels_as_if_they_referred_to_end(&mut self) {
        let len = i32::try_from(self.buf.len()).unwrap();
        for offset in self.offsets_using_label.iter() {
            let range = self.buf[*offset..].first_chunk_mut::<4>().unwrap();
            let offset = i32::try_from(*offset).unwrap() + 4;
            let rel_distance = len - offset;
            *range = (i32::from_le_bytes(*range) + rel_distance).to_le_bytes();
        }
    }
}

impl CodeSink for TestCodeSink {
    fn put1(&mut self, v: u8) {
        self.buf.extend_from_slice(&[v]);
    }

    fn put2(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    fn put4(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    fn put8(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    fn add_trap(&mut self, _: TrapCode) {}

    fn use_target(&mut self, _: DeferredTarget) {
        let offset = self.buf.len();
        self.offsets_using_label.push(offset);
    }

    fn known_offset(&self, target: KnownOffset) -> i32 {
        panic!("unsupported known target {target:?}")
    }
}

/// Building a new `Capstone` each time is suboptimal (TODO).
fn disassemble(assembled: &[u8], original: &Inst<FuzzRegs>) -> String {
    let cs = Capstone::new()
        .x86()
        .mode(x86::ArchMode::Mode64)
        .syntax(x86::ArchSyntax::Att)
        .detail(true)
        .build()
        .expect("failed to create Capstone object");
    let insts = cs
        .disasm_all(assembled, 0x0)
        .expect("failed to disassemble");

    if insts.len() != 1 {
        println!("> {original}");
        println!("  debug: {original:x?}");
        println!("  assembled: {}", pretty_print_hexadecimal(&assembled));
        assert_eq!(insts.len(), 1, "not a single instruction");
    }

    let inst = insts.first().expect("at least one instruction");
    if assembled.len() != inst.len() {
        println!("> {original}");
        println!("  debug: {original:x?}");
        println!("  assembled: {}", pretty_print_hexadecimal(&assembled));
        println!(
            "  capstone-assembled: {}",
            pretty_print_hexadecimal(inst.bytes())
        );
        assert_eq!(assembled.len(), inst.len(), "extra bytes not disassembled");
    }

    inst.to_string()
}

fn pretty_print_hexadecimal(hex: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(hex.len() * 2);
    for b in hex {
        write!(&mut s, "{b:02X}").unwrap();
    }
    s
}

/// See `replace_signed_immediates`.
macro_rules! hex_print_signed_imm {
    ($hex:expr, $from:ty => $to:ty) => {{
        let imm = <$from>::from_str_radix($hex, 16).unwrap() as $to;
        let mut simm = String::new();
        if imm < 0 {
            simm.push_str("-");
        }
        let abs = match imm.checked_abs() {
            Some(i) => i,
            None => <$to>::MIN,
        };
        if imm > -10 && imm < 10 {
            simm.push_str(&format!("{:x}", abs));
        } else {
            simm.push_str(&format!("0x{:x}", abs));
        }
        simm
    }};
}

/// Replace signed immediates in the disassembly with their unsigned hexadecimal
/// equivalent. This is only necessary to match `capstone`'s complex
/// pretty-printing rules; e.g. `capstone` will:
/// - omit the `0x` prefix when printing `0x0` as `0`.
/// - omit the `0x` prefix when print small values (less than 10)
/// - print negative values as `-0x...` (signed hex) instead of `0xff...`
///   (normal hex)
/// - print `mov` immediates as base-10 instead of base-16 (?!).
fn replace_signed_immediates(dis: &str) -> std::borrow::Cow<'_, str> {
    match dis.find('$') {
        None => dis.into(),
        Some(idx) => {
            let (prefix, rest) = dis.split_at(idx + 1); // Skip the '$'.
            let (_, rest) = chomp("-", rest); // Skip the '-' if it's there.
            let (_, rest) = chomp("0x", rest); // Skip the '0x' if it's there.
            let n = rest.chars().take_while(char::is_ascii_hexdigit).count();
            let (hex, rest) = rest.split_at(n); // Split at next non-hex character.
            let simm = if dis.starts_with("mov") {
                u64::from_str_radix(hex, 16).unwrap().to_string()
            } else {
                match hex.len() {
                    1 | 2 => hex_print_signed_imm!(hex, u8 => i8),
                    4 => hex_print_signed_imm!(hex, u16 => i16),
                    8 => hex_print_signed_imm!(hex, u32 => i32),
                    16 => hex_print_signed_imm!(hex, u64 => i64),
                    _ => panic!("unexpected length for hex: {hex}"),
                }
            };
            format!("{prefix}{simm}{rest}").into()
        }
    }
}

// See `replace_signed_immediates`.
fn chomp<'a>(pat: &str, s: &'a str) -> (&'a str, &'a str) {
    if s.starts_with(pat) {
        s.split_at(pat.len())
    } else {
        ("", s)
    }
}

#[test]
fn replace() {
    assert_eq!(
        replace_signed_immediates("andl $0xffffff9a, %r11d"),
        "andl $-0x66, %r11d"
    );
    assert_eq!(
        replace_signed_immediates("xorq $0xffffffffffffffbc, 0x7f139ecc(%r9)"),
        "xorq $-0x44, 0x7f139ecc(%r9)"
    );
    assert_eq!(
        replace_signed_immediates("subl $0x3ca77a19, -0x1a030f40(%r14)"),
        "subl $0x3ca77a19, -0x1a030f40(%r14)"
    );
    assert_eq!(
        replace_signed_immediates("movq $0xffffffff864ae103, %rsi"),
        "movq $18446744071667638531, %rsi"
    );
}

/// Remove everything after the first semicolon in the disassembly and trim any
/// trailing spaces. This is necessary to remove the implicit operands we end up
/// printing for Cranelift's sake.
fn remove_after_semicolon(dis: &str) -> &str {
    match dis.find(';') {
        None => dis,
        Some(idx) => {
            let (prefix, _) = dis.split_at(idx);
            prefix.trim()
        }
    }
}

#[test]
fn remove_after_parenthesis_test() {
    assert_eq!(
        remove_after_semicolon("imulb 0x7658eddd(%rcx) ;; implicit: %ax"),
        "imulb 0x7658eddd(%rcx)"
    );
}

/// Run some post-processing on the disassembly to make it match Capstone.
fn fix_up(dis: &str) -> std::borrow::Cow<'_, str> {
    let dis = remove_after_semicolon(dis);
    replace_signed_immediates(&dis)
}

/// Fuzz-specific registers.
///
/// For the fuzzer, we do not need any fancy register types; see [`FuzzReg`].
#[derive(Clone, Arbitrary, Debug)]
pub struct FuzzRegs;

impl Registers for FuzzRegs {
    type ReadGpr = FuzzReg;
    type ReadWriteGpr = FuzzReg;
    type WriteGpr = FuzzReg;
    type ReadXmm = FuzzReg;
    type ReadWriteXmm = FuzzReg;
    type WriteXmm = FuzzReg;
}

/// A simple `u8` register type for fuzzing only.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuzzReg(u8);

impl<'a> Arbitrary<'a> for FuzzReg {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self(u.int_in_range(0..=15)?))
    }
}

impl AsReg for FuzzReg {
    fn new(enc: u8) -> Self {
        Self(enc)
    }
    fn enc(&self) -> u8 {
        self.0
    }
}

impl Arbitrary<'_> for AmodeOffset {
    fn arbitrary(u: &mut Unstructured<'_>) -> Result<Self> {
        // Custom implementation to try to generate some "interesting" offsets.
        // For example choose either an arbitrary 8-bit or 32-bit number as the
        // base, and then optionally shift that number to the left to create
        // multiples of constants. This can help stress some of the more
        // interesting encodings in EVEX instructions for example.
        let base = if u.arbitrary()? {
            i32::from(u.arbitrary::<i8>()?)
        } else {
            u.arbitrary::<i32>()?
        };
        Ok(match u.int_in_range(0..=5)? {
            0 => AmodeOffset::ZERO,
            n => AmodeOffset::new(base << (n - 1)),
        })
    }
}

impl Arbitrary<'_> for AmodeOffsetPlusKnownOffset {
    fn arbitrary(u: &mut Unstructured<'_>) -> Result<Self> {
        // For now, we don't generate offsets (TODO).
        Ok(Self {
            simm32: AmodeOffset::arbitrary(u)?,
            offset: None,
        })
    }
}

impl<R: AsReg, const E: u8> Arbitrary<'_> for Fixed<R, E> {
    fn arbitrary(_: &mut Unstructured<'_>) -> Result<Self> {
        Ok(Self::new(E))
    }
}

impl<R: AsReg> Arbitrary<'_> for NonRspGpr<R> {
    fn arbitrary(u: &mut Unstructured<'_>) -> Result<Self> {
        use crate::gpr::enc::*;
        let gpr = u.choose(&[
            RAX, RCX, RDX, RBX, RBP, RSI, RDI, R8, R9, R10, R11, R12, R13, R14, R15,
        ])?;
        Ok(Self::new(R::new(*gpr)))
    }
}
impl<'a, R: AsReg> Arbitrary<'a> for Gpr<R> {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(Self(R::new(u.int_in_range(0..=15)?)))
    }
}
impl<'a, R: AsReg> Arbitrary<'a> for Xmm<R> {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(Self(R::new(u.int_in_range(0..=15)?)))
    }
}

/// Helper trait that's used to be the same as `Registers` except with an extra
/// `for<'a> Arbitrary<'a>` bound on all of the associated types.
pub trait RegistersArbitrary:
    Registers<
        ReadGpr: for<'a> Arbitrary<'a>,
        ReadWriteGpr: for<'a> Arbitrary<'a>,
        WriteGpr: for<'a> Arbitrary<'a>,
        ReadXmm: for<'a> Arbitrary<'a>,
        ReadWriteXmm: for<'a> Arbitrary<'a>,
        WriteXmm: for<'a> Arbitrary<'a>,
    >
{
}

impl<R> RegistersArbitrary for R
where
    R: Registers,
    R::ReadGpr: for<'a> Arbitrary<'a>,
    R::ReadWriteGpr: for<'a> Arbitrary<'a>,
    R::WriteGpr: for<'a> Arbitrary<'a>,
    R::ReadXmm: for<'a> Arbitrary<'a>,
    R::ReadWriteXmm: for<'a> Arbitrary<'a>,
    R::WriteXmm: for<'a> Arbitrary<'a>,
{
}

#[cfg(test)]
mod test {
    use super::*;
    use arbtest::arbtest;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn smoke() {
        let count = AtomicUsize::new(0);
        arbtest(|u| {
            let inst: Inst<FuzzRegs> = u.arbitrary()?;
            roundtrip(&inst);
            println!("#{}: {inst}", count.fetch_add(1, Ordering::SeqCst));
            Ok(())
        })
        .budget_ms(1_000);

        // This will run the `roundtrip` fuzzer for one second. To repeatably
        // test a single input, append `.seed(0x<failing seed>)`.
    }

    #[test]
    fn callq() {
        for i in -500..500 {
            println!("immediate: {i}");
            let inst = crate::inst::callq_d::new(i);
            roundtrip(&inst.into());
        }
    }
}
