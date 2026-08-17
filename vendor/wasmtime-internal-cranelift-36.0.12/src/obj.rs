//! Object file builder.
//!
//! Creates ELF image based on `Compilation` information. The ELF contains
//! functions and trampolines in the ".text" section. It also contains all
//! relocation records for the linking stage. If DWARF sections exist, their
//! content will be written as well.
//!
//! The object file has symbols for each function and trampoline, as well as
//! symbols that refer to libcalls.
//!
//! The function symbol names have format "_wasm_function_N", where N is
//! `FuncIndex`. The defined wasm function symbols refer to a JIT compiled
//! function body, the imported wasm function do not. The trampolines symbol
//! names have format "_trampoline_N", where N is `SignatureIndex`.

use crate::{CompiledFunction, RelocationTarget};
use anyhow::Result;
use cranelift_codegen::TextSectionBuilder;
use cranelift_codegen::isa::unwind::{UnwindInfo, systemv};
use cranelift_control::ControlPlane;
use gimli::RunTimeEndian;
use gimli::write::{Address, EhFrame, EndianVec, FrameTable, Writer};
use object::write::{Object, SectionId, StandardSegment, Symbol, SymbolId, SymbolSection};
use object::{Architecture, SectionFlags, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::ops::Range;
use wasmtime_environ::obj;
use wasmtime_environ::{Compiler, TripleExt, Unsigned};

const TEXT_SECTION_NAME: &[u8] = b".text";

fn text_align(compiler: &dyn Compiler) -> u64 {
    // text pages will not be made executable with pulley, so the section
    // doesn't need to be padded out to page alignment boundaries.
    if compiler.triple().is_pulley() {
        0x1
    } else {
        compiler.page_size_align()
    }
}

/// A helper structure used to assemble the final text section of an executable,
/// plus unwinding information and other related details.
///
/// This builder relies on Cranelift-specific internals but assembles into a
/// generic `Object` which will get further appended to in a compiler-agnostic
/// fashion later.
pub struct ModuleTextBuilder<'a> {
    /// The target that we're compiling for, used to query target-specific
    /// information as necessary.
    compiler: &'a dyn Compiler,

    /// The object file that we're generating code into.
    obj: &'a mut Object<'static>,

    /// The WebAssembly module we're generating code for.
    text_section: SectionId,

    unwind_info: UnwindInfoBuilder<'a>,

    /// In-progress text section that we're using cranelift's `MachBuffer` to
    /// build to resolve relocations (calls) between functions.
    text: Box<dyn TextSectionBuilder>,

    ctrl_plane: ControlPlane,
}

impl<'a> ModuleTextBuilder<'a> {
    /// Creates a new builder for the text section of an executable.
    ///
    /// The `.text` section will be appended to the specified `obj` along with
    /// any unwinding or such information as necessary. The `num_funcs`
    /// parameter indicates the number of times the `append_func` function will
    /// be called. The `finish` function will panic if this contract is not met.
    pub fn new(
        obj: &'a mut Object<'static>,
        compiler: &'a dyn Compiler,
        text: Box<dyn TextSectionBuilder>,
    ) -> Self {
        // Entire code (functions and trampolines) will be placed
        // in the ".text" section.
        let text_section = obj.add_section(
            obj.segment_name(StandardSegment::Text).to_vec(),
            TEXT_SECTION_NAME.to_vec(),
            SectionKind::Text,
        );

        // If this target is Pulley then flag the text section as not needing the
        // executable bit in virtual memory which means that the runtime won't
        // try to call `Mmap::make_executable`, which makes Pulley more
        // portable.
        if compiler.triple().is_pulley() {
            let section = obj.section_mut(text_section);
            assert!(matches!(section.flags, SectionFlags::None));
            section.flags = SectionFlags::Elf {
                sh_flags: obj::SH_WASMTIME_NOT_EXECUTED,
            };
        }

        Self {
            compiler,
            obj,
            text_section,
            unwind_info: Default::default(),
            text,
            ctrl_plane: ControlPlane::default(),
        }
    }

    /// Appends the `func` specified named `name` to this object.
    ///
    /// The `resolve_reloc_target` closure is used to resolve a relocation
    /// target to an adjacent function which has already been added or will be
    /// added to this object. The argument is the relocation target specified
    /// within `CompiledFunction` and the return value must be an index where
    /// the target will be defined by the `n`th call to `append_func`.
    ///
    /// Returns the symbol associated with the function as well as the range
    /// that the function resides within the text section.
    pub fn append_func(
        &mut self,
        name: &str,
        compiled_func: &'a CompiledFunction,
        resolve_reloc_target: impl Fn(wasmtime_environ::RelocationTarget) -> usize,
    ) -> (SymbolId, Range<u64>) {
        let body = compiled_func.buffer.data();
        let alignment = compiled_func.alignment;
        let body_len = body.len() as u64;
        let off = self
            .text
            .append(true, &body, alignment, &mut self.ctrl_plane);

        let symbol_id = self.obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value: off,
            size: body_len,
            kind: SymbolKind::Text,
            scope: SymbolScope::Compilation,
            weak: false,
            section: SymbolSection::Section(self.text_section),
            flags: SymbolFlags::None,
        });

        if let Some(info) = compiled_func.unwind_info() {
            self.unwind_info.push(off, body_len, info);
        }

        for r in compiled_func.relocations() {
            let reloc_offset = off + u64::from(r.offset);
            match r.reloc_target {
                // Relocations against user-defined functions means that this is
                // a relocation against a module-local function, typically a
                // call between functions. The `text` field is given priority to
                // resolve this relocation before we actually emit an object
                // file, but if it can't handle it then we pass through the
                // relocation.
                RelocationTarget::Wasm(_) | RelocationTarget::Builtin(_) => {
                    let target = resolve_reloc_target(r.reloc_target);
                    if self
                        .text
                        .resolve_reloc(reloc_offset, r.reloc, r.addend, target)
                    {
                        continue;
                    }

                    // At this time it's expected that all relocations are
                    // handled by `text.resolve_reloc`, and anything that isn't
                    // handled is a bug in `text.resolve_reloc` or something
                    // transitively there. If truly necessary, though, then this
                    // loop could also be updated to forward the relocation to
                    // the final object file as well.
                    panic!(
                        "unresolved relocation could not be processed against \
                         {:?}: {r:?}",
                        r.reloc_target,
                    );
                }

                // This relocation is used to fill in which hostcall id is
                // desired within the `call_indirect_host` opcode of Pulley
                // itself. The relocation target is the start of the instruction
                // and the goal is to insert the static signature number, `n`,
                // into the instruction.
                //
                // At this time the instruction looks like:
                //
                //      +------+------+------+------+
                //      | OP   | OP_EXTENDED |  N   |
                //      +------+------+------+------+
                //
                // This 4-byte encoding has `OP` indicating this is an "extended
                // opcode" where `OP_EXTENDED` is a 16-bit extended opcode.
                // The `N` byte is the index of the signature being called and
                // is what's b eing filled in.
                //
                // See the `test_call_indirect_host_width` in
                // `pulley/tests/all.rs` for this guarantee as well.
                RelocationTarget::PulleyHostcall(n) => {
                    #[cfg(feature = "pulley")]
                    {
                        use pulley_interpreter::encode::Encode;
                        assert_eq!(pulley_interpreter::CallIndirectHost::WIDTH, 4);
                    }
                    let byte = u8::try_from(n).unwrap();
                    self.text.write(reloc_offset + 3, &[byte]);
                }
            };
        }
        (symbol_id, off..off + body_len)
    }

    /// Forces "veneers" to be used for inter-function calls in the text
    /// section which means that in-bounds optimized addresses are never used.
    ///
    /// This is only useful for debugging cranelift itself and typically this
    /// option is disabled.
    pub fn force_veneers(&mut self) {
        self.text.force_veneers();
    }

    /// Appends the specified amount of bytes of padding into the text section.
    ///
    /// This is only useful when fuzzing and/or debugging cranelift itself and
    /// for production scenarios `padding` is 0 and this function does nothing.
    pub fn append_padding(&mut self, padding: usize) {
        if padding == 0 {
            return;
        }
        self.text
            .append(false, &vec![0; padding], 1, &mut self.ctrl_plane);
    }

    /// Indicates that the text section has been written completely and this
    /// will finish appending it to the original object.
    ///
    /// Note that this will also write out the unwind information sections if
    /// necessary.
    pub fn finish(mut self) {
        // Finish up the text section now that we're done adding functions.
        let text = self.text.finish(&mut self.ctrl_plane);
        self.obj
            .section_mut(self.text_section)
            .set_data(text, text_align(self.compiler));

        // Append the unwind information for all our functions, if necessary.
        self.unwind_info
            .append_section(self.compiler, self.obj, self.text_section);
    }
}

/// Builder used to create unwind information for a set of functions added to a
/// text section.
#[derive(Default)]
struct UnwindInfoBuilder<'a> {
    windows_xdata: Vec<u8>,
    windows_pdata: Vec<RUNTIME_FUNCTION>,
    systemv_unwind_info: Vec<(u64, &'a systemv::UnwindInfo)>,
}

// This is a mirror of `RUNTIME_FUNCTION` in the Windows API, but defined here
// to ensure everything is always `u32` and to have it available on all
// platforms. Note that all of these specifiers here are relative to a "base
// address" which we define as the base of where the text section is eventually
// loaded.
#[expect(non_camel_case_types, reason = "matching Windows style, not Rust")]
struct RUNTIME_FUNCTION {
    begin: u32,
    end: u32,
    unwind_address: u32,
}

impl<'a> UnwindInfoBuilder<'a> {
    /// Pushes the unwind information for a function into this builder.
    ///
    /// The function being described must be located at `function_offset` within
    /// the text section itself, and the function's size is specified by
    /// `function_len`.
    ///
    /// The `info` should come from Cranelift. and is handled here depending on
    /// its flavor.
    fn push(&mut self, function_offset: u64, function_len: u64, info: &'a UnwindInfo) {
        match info {
            // Windows unwind information is stored in two locations:
            //
            // * First is the actual unwinding information which is stored
            //   in the `.xdata` section. This is where `info`'s emitted
            //   information will go into.
            // * Second are pointers to connect all this unwind information,
            //   stored in the `.pdata` section. The `.pdata` section is an
            //   array of `RUNTIME_FUNCTION` structures.
            //
            // Due to how these will be loaded at runtime the `.pdata` isn't
            // actually assembled byte-wise here. Instead that's deferred to
            // happen later during `write_windows_unwind_info` which will apply
            // a further offset to `unwind_address`.
            //
            // FIXME: in theory we could "intern" the `unwind_info` value
            // here within the `.xdata` section. Most of our unwind
            // information for functions is probably pretty similar in which
            // case the `.xdata` could be quite small and `.pdata` could
            // have multiple functions point to the same unwinding
            // information.
            UnwindInfo::WindowsX64(info) => {
                let unwind_size = info.emit_size();
                let mut unwind_info = vec![0; unwind_size];
                info.emit(&mut unwind_info);

                // `.xdata` entries are always 4-byte aligned
                while self.windows_xdata.len() % 4 != 0 {
                    self.windows_xdata.push(0x00);
                }
                let unwind_address = self.windows_xdata.len();
                self.windows_xdata.extend_from_slice(&unwind_info);

                // Record a `RUNTIME_FUNCTION` which this will point to.
                self.windows_pdata.push(RUNTIME_FUNCTION {
                    begin: u32::try_from(function_offset).unwrap(),
                    end: u32::try_from(function_offset + function_len).unwrap(),
                    unwind_address: u32::try_from(unwind_address).unwrap(),
                });
            }

            // See https://learn.microsoft.com/en-us/cpp/build/arm64-exception-handling
            UnwindInfo::WindowsArm64(info) => {
                let code_words = info.code_words();
                let mut unwind_codes = vec![0; (code_words * 4) as usize];
                info.emit(&mut unwind_codes);

                // `.xdata` entries are always 4-byte aligned
                while self.windows_xdata.len() % 4 != 0 {
                    self.windows_xdata.push(0x00);
                }

                // First word:
                // 0-17:    Function Length
                // 18-19:   Version (must be 0)
                // 20:      X bit (is exception data present?)
                // 21:      E bit (has single packed epilogue?)
                // 22-26:   Epilogue count
                // 27-31:   Code words count
                let requires_extended_counts = code_words > (1 << 5);
                let encoded_function_len = function_len / 4;
                assert!(encoded_function_len < (1 << 18), "function too large");
                let mut word1 = u32::try_from(encoded_function_len).unwrap();
                if !requires_extended_counts {
                    word1 |= u32::from(code_words) << 27;
                }
                let unwind_address = self.windows_xdata.len();
                self.windows_xdata.extend_from_slice(&word1.to_le_bytes());

                if requires_extended_counts {
                    // Extended counts word:
                    // 0-15:    Epilogue count
                    // 16-23:   Code words count
                    let extended_counts_word = (code_words as u32) << 16;
                    self.windows_xdata
                        .extend_from_slice(&extended_counts_word.to_le_bytes());
                }

                // Skip epilogue information: Per comment on [`UnwindInst`], we
                // do not emit information about epilogues.

                // Emit the unwind codes.
                self.windows_xdata.extend_from_slice(&unwind_codes);

                // Record a `RUNTIME_FUNCTION` which this will point to.
                // NOTE: `end` is not used, so leave it as 0.
                self.windows_pdata.push(RUNTIME_FUNCTION {
                    begin: u32::try_from(function_offset).unwrap(),
                    end: 0,
                    unwind_address: u32::try_from(unwind_address).unwrap(),
                });
            }

            // System-V is different enough that we just record the unwinding
            // information to get processed at a later time.
            UnwindInfo::SystemV(info) => {
                self.systemv_unwind_info.push((function_offset, info));
            }

            _ => panic!("some unwind info isn't handled here"),
        }
    }

    /// Appends the unwind information section, if any, to the `obj` specified.
    ///
    /// This function must be called immediately after the text section was
    /// added to a builder. The unwind information section must trail the text
    /// section immediately.
    ///
    /// The `text_section`'s section identifier is passed into this function.
    fn append_section(
        &self,
        compiler: &dyn Compiler,
        obj: &mut Object<'_>,
        text_section: SectionId,
    ) {
        // This write will align the text section to a page boundary and then
        // return the offset at that point. This gives us the full size of the
        // text section at that point, after alignment.
        let text_section_size = obj.append_section_data(text_section, &[], text_align(compiler));

        if self.windows_xdata.len() > 0 {
            assert!(self.systemv_unwind_info.len() == 0);
            // The `.xdata` section must come first to be just-after the `.text`
            // section for the reasons documented in `write_windows_unwind_info`
            // below.
            let segment = obj.segment_name(StandardSegment::Data).to_vec();
            let xdata_id = obj.add_section(segment, b".xdata".to_vec(), SectionKind::ReadOnlyData);
            let segment = obj.segment_name(StandardSegment::Data).to_vec();
            let pdata_id = obj.add_section(segment, b".pdata".to_vec(), SectionKind::ReadOnlyData);
            self.write_windows_unwind_info(obj, xdata_id, pdata_id, text_section_size);
        }

        if self.systemv_unwind_info.len() > 0 {
            let segment = obj.segment_name(StandardSegment::Data).to_vec();
            let section_id =
                obj.add_section(segment, b".eh_frame".to_vec(), SectionKind::ReadOnlyData);
            self.write_systemv_unwind_info(compiler, obj, section_id, text_section_size)
        }
    }

    /// This function appends a nonstandard section to the object which is only
    /// used during `CodeMemory::publish`.
    ///
    /// This custom section effectively stores a `[RUNTIME_FUNCTION; N]` into
    /// the object file itself. This way registration of unwind info can simply
    /// pass this slice to the OS itself and there's no need to recalculate
    /// anything on the other end of loading a module from a precompiled object.
    ///
    /// Support for reading this is in `crates/jit/src/unwind/winx64.rs`.
    fn write_windows_unwind_info(
        &self,
        obj: &mut Object<'_>,
        xdata_id: SectionId,
        pdata_id: SectionId,
        text_section_size: u64,
    ) {
        // Append the `.xdata` section, or the actual unwinding information
        // codes and such which were built as we found unwind information for
        // functions.
        obj.append_section_data(xdata_id, &self.windows_xdata, 4);

        // Next append the `.pdata` section, or the array of `RUNTIME_FUNCTION`
        // structures stored in the binary.
        //
        // This memory will be passed at runtime to `RtlAddFunctionTable` which
        // takes a "base address" and the entries within `RUNTIME_FUNCTION` are
        // all relative to this base address. The base address we pass is the
        // address of the text section itself so all the pointers here must be
        // text-section-relative. The `begin` and `end` fields for the function
        // it describes are already text-section-relative, but the
        // `unwind_address` field needs to be updated here since the value
        // stored right now is `xdata`-section-relative. We know that the
        // `xdata` section follows the `.text` section so the
        // `text_section_size` is added in to calculate the final
        // `.text`-section-relative address of the unwind information.
        let xdata_rva = |address| {
            let address = u64::from(address);
            let address = address + text_section_size;
            u32::try_from(address).unwrap()
        };
        let pdata = match obj.architecture() {
            Architecture::X86_64 => {
                let mut pdata = Vec::with_capacity(self.windows_pdata.len() * 3 * 4);
                for info in self.windows_pdata.iter() {
                    pdata.extend_from_slice(&info.begin.to_le_bytes());
                    pdata.extend_from_slice(&info.end.to_le_bytes());
                    pdata.extend_from_slice(&xdata_rva(info.unwind_address).to_le_bytes());
                }
                pdata
            }

            Architecture::Aarch64 => {
                // Windows Arm64 .pdata also supports packed unwind data, but
                // we're not currently using that.
                let mut pdata = Vec::with_capacity(self.windows_pdata.len() * 2 * 4);
                for info in self.windows_pdata.iter() {
                    pdata.extend_from_slice(&info.begin.to_le_bytes());
                    pdata.extend_from_slice(&xdata_rva(info.unwind_address).to_le_bytes());
                }
                pdata
            }

            _ => unimplemented!("unsupported architecture for windows unwind info"),
        };
        obj.append_section_data(pdata_id, &pdata, 4);
    }

    /// This function appends a nonstandard section to the object which is only
    /// used during `CodeMemory::publish`.
    ///
    /// This will generate a `.eh_frame` section, but not one that can be
    /// naively loaded. The goal of this section is that we can create the
    /// section once here and never again does it need to change. To describe
    /// dynamically loaded functions though each individual FDE needs to talk
    /// about the function's absolute address that it's referencing. Naturally
    /// we don't actually know the function's absolute address when we're
    /// creating an object here.
    ///
    /// To solve this problem the FDE address encoding mode is set to
    /// `DW_EH_PE_pcrel`. This means that the actual effective address that the
    /// FDE describes is a relative to the address of the FDE itself. By
    /// leveraging this relative-ness we can assume that the relative distance
    /// between the FDE and the function it describes is constant, which should
    /// allow us to generate an FDE ahead-of-time here.
    ///
    /// For now this assumes that all the code of functions will start at a
    /// page-aligned address when loaded into memory. The eh_frame encoded here
    /// then assumes that the text section is itself page aligned to its size
    /// and the eh_frame will follow just after the text section. This means
    /// that the relative offsets we're using here is the FDE going backwards
    /// into the text section itself.
    ///
    /// Note that the library we're using to create the FDEs, `gimli`, doesn't
    /// actually encode addresses relative to the FDE itself. Instead the
    /// addresses are encoded relative to the start of the `.eh_frame` section.
    /// This makes it much easier for us where we provide the relative offset
    /// from the start of `.eh_frame` to the function in the text section, which
    /// given our layout basically means the offset of the function in the text
    /// section from the end of the text section.
    ///
    /// A final note is that the reason we page-align the text section's size is
    /// so the .eh_frame lives on a separate page from the text section itself.
    /// This allows `.eh_frame` to have different virtual memory permissions,
    /// such as being purely read-only instead of read/execute like the code
    /// bits.
    fn write_systemv_unwind_info(
        &self,
        compiler: &dyn Compiler,
        obj: &mut Object<'_>,
        section_id: SectionId,
        text_section_size: u64,
    ) {
        let mut cie = match compiler.create_systemv_cie() {
            Some(cie) => cie,
            None => return,
        };
        let mut table = FrameTable::default();
        cie.fde_address_encoding = gimli::constants::DW_EH_PE_pcrel;
        let cie_id = table.add_cie(cie);

        for (text_section_off, unwind_info) in self.systemv_unwind_info.iter() {
            let backwards_off = text_section_size - text_section_off;
            let actual_offset = -i64::try_from(backwards_off).unwrap();
            // Note that gimli wants an unsigned 64-bit integer here, but
            // unwinders just use this constant for a relative addition with the
            // address of the FDE, which means that the sign doesn't actually
            // matter.
            let fde = unwind_info.to_fde(Address::Constant(actual_offset.unsigned()));
            table.add_fde(cie_id, fde);
        }
        let endian = match compiler.triple().endianness().unwrap() {
            target_lexicon::Endianness::Little => RunTimeEndian::Little,
            target_lexicon::Endianness::Big => RunTimeEndian::Big,
        };
        let mut eh_frame = EhFrame(MyVec(EndianVec::new(endian)));
        table.write_eh_frame(&mut eh_frame).unwrap();

        // Some unwinding implementations expect a terminating "empty" length so
        // a 0 is written at the end of the table for those implementations.
        let mut endian_vec = (eh_frame.0).0;
        endian_vec.write_u32(0).unwrap();
        obj.append_section_data(section_id, endian_vec.slice(), 1);

        use gimli::constants;
        use gimli::write::Error;

        struct MyVec(EndianVec<RunTimeEndian>);

        impl Writer for MyVec {
            type Endian = RunTimeEndian;

            fn endian(&self) -> RunTimeEndian {
                self.0.endian()
            }

            fn len(&self) -> usize {
                self.0.len()
            }

            fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
                self.0.write(buf)
            }

            fn write_at(&mut self, pos: usize, buf: &[u8]) -> Result<(), Error> {
                self.0.write_at(pos, buf)
            }

            // FIXME(gimli-rs/gimli#576) this is the definition we want for
            // `write_eh_pointer` but the default implementation, at the time
            // of this writing, uses `offset - val` instead of `val - offset`.
            // A PR has been merged to fix this but until that's published we
            // can't use it.
            fn write_eh_pointer(
                &mut self,
                address: Address,
                eh_pe: constants::DwEhPe,
                size: u8,
            ) -> Result<(), Error> {
                let val = match address {
                    Address::Constant(val) => val,
                    Address::Symbol { .. } => unreachable!(),
                };
                assert_eq!(eh_pe.application(), constants::DW_EH_PE_pcrel);
                let offset = self.len() as u64;
                let val = val.wrapping_sub(offset);
                self.write_eh_pointer_data(val, eh_pe.format(), size)
            }
        }
    }
}
