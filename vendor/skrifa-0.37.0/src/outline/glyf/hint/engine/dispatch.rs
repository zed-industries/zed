//! Instruction decoding and dispatch.

use read_fonts::tables::glyf::bytecode::Opcode;

use super::{super::program::Program, Engine, HintError, HintErrorKind, Instruction};

/// Maximum number of instructions we will execute in `Engine::run()`. This
/// is used to ensure termination of a hinting program.
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/include/freetype/config/ftoption.h#L744>
const MAX_RUN_INSTRUCTIONS: usize = 1_000_000;

impl<'a> Engine<'a> {
    /// Resets state for the specified program and executes all instructions.
    pub fn run_program(&mut self, program: Program, is_pedantic: bool) -> Result<(), HintError> {
        self.reset(program, is_pedantic);
        self.run()
    }

    /// Set internal state for running the specified program.
    pub fn reset(&mut self, program: Program, is_pedantic: bool) {
        self.program.reset(program);
        // Reset overall graphics state, keeping the retained bits.
        self.graphics.reset();
        self.graphics.is_pedantic = is_pedantic;
        self.loop_budget.reset();
        // Program specific setup.
        match program {
            Program::Font => {
                self.definitions.functions.reset();
                self.definitions.instructions.reset();
            }
            Program::ControlValue => {
                self.graphics.backward_compatibility = false;
            }
            Program::Glyph => {
                // Instruct control bit 1 says we reset retained graphics state
                // to default values.
                if self.graphics.instruct_control & 2 != 0 {
                    self.graphics.reset_retained();
                }
                // Set backward compatibility mode
                if self.graphics.target.preserve_linear_metrics() {
                    self.graphics.backward_compatibility = true;
                } else if self.graphics.target.is_smooth() {
                    self.graphics.backward_compatibility =
                        (self.graphics.instruct_control & 0x4) == 0;
                } else {
                    self.graphics.backward_compatibility = false;
                }
            }
        }
    }

    /// Decodes and dispatches all instructions until completion or error.
    pub fn run(&mut self) -> Result<(), HintError> {
        let mut count = 0;
        while let Some(ins) = self.decode() {
            let ins = ins?;
            self.dispatch(&ins)?;
            count += 1;
            if count > MAX_RUN_INSTRUCTIONS {
                return Err(HintError {
                    program: self.program.current,
                    glyph_id: None,
                    pc: ins.pc,
                    opcode: Some(ins.opcode),
                    kind: HintErrorKind::ExceededExecutionBudget,
                });
            }
        }
        Ok(())
    }

    /// Decodes the next instruction from the current program.
    pub fn decode(&mut self) -> Option<Result<Instruction<'a>, HintError>> {
        let ins = self.program.decoder.decode()?;
        Some(ins.map_err(|_| HintError {
            program: self.program.current,
            glyph_id: None,
            pc: self.program.decoder.pc,
            opcode: None,
            kind: HintErrorKind::UnexpectedEndOfBytecode,
        }))
    }

    /// Executes the appropriate code for the given instruction.
    pub fn dispatch(&mut self, ins: &Instruction) -> Result<(), HintError> {
        let current_program = self.program.current;
        self.dispatch_inner(ins).map_err(|kind| HintError {
            program: current_program,
            glyph_id: None,
            pc: ins.pc,
            opcode: Some(ins.opcode),
            kind,
        })
    }

    fn dispatch_inner(&mut self, ins: &Instruction) -> Result<(), HintErrorKind> {
        use Opcode::*;
        let opcode = ins.opcode;
        let raw_opcode = opcode as u8;
        match ins.opcode {
            SVTCA0 | SVTCA1 | SPVTCA0 | SPVTCA1 | SFVTCA0 | SFVTCA1 => self.op_svtca(raw_opcode)?,
            SPVTL0 | SPVTL1 | SFVTL0 | SFVTL1 => self.op_svtl(raw_opcode)?,
            SPVFS => self.op_spvfs()?,
            SFVFS => self.op_sfvfs()?,
            GPV => self.op_gpv()?,
            GFV => self.op_gfv()?,
            SFVTPV => self.op_sfvtpv()?,
            ISECT => self.op_isect()?,
            SRP0 => self.op_srp0()?,
            SRP1 => self.op_srp1()?,
            SRP2 => self.op_srp2()?,
            SZP0 => self.op_szp0()?,
            SZP1 => self.op_szp1()?,
            SZP2 => self.op_szp2()?,
            SZPS => self.op_szps()?,
            SLOOP => self.op_sloop()?,
            RTG => self.op_rtg()?,
            RTHG => self.op_rthg()?,
            SMD => self.op_smd()?,
            ELSE => self.op_else()?,
            JMPR => self.op_jmpr()?,
            SCVTCI => self.op_scvtci()?,
            SSWCI => self.op_sswci()?,
            SSW => self.op_ssw()?,
            DUP => self.op_dup()?,
            POP => self.op_pop()?,
            CLEAR => self.op_clear()?,
            SWAP => self.op_swap()?,
            DEPTH => self.op_depth()?,
            CINDEX => self.op_cindex()?,
            MINDEX => self.op_mindex()?,
            ALIGNPTS => self.op_alignpts()?,
            // UNUSED: 0x28
            UTP => self.op_utp()?,
            LOOPCALL => self.op_loopcall()?,
            CALL => self.op_call()?,
            FDEF => self.op_fdef()?,
            ENDF => self.op_endf()?,
            MDAP0 | MDAP1 => self.op_mdap(raw_opcode)?,
            IUP0 | IUP1 => self.op_iup(raw_opcode)?,
            SHP0 | SHP1 => self.op_shp(raw_opcode)?,
            SHC0 | SHC1 => self.op_shc(raw_opcode)?,
            SHZ0 | SHZ1 => self.op_shz(raw_opcode)?,
            SHPIX => self.op_shpix()?,
            IP => self.op_ip()?,
            MSIRP0 | MSIRP1 => self.op_msirp(raw_opcode)?,
            ALIGNRP => self.op_alignrp()?,
            RTDG => self.op_rtdg()?,
            MIAP0 | MIAP1 => self.op_miap(raw_opcode)?,
            NPUSHB | NPUSHW => self.op_push(&ins.inline_operands)?,
            WS => self.op_ws()?,
            RS => self.op_rs()?,
            WCVTP => self.op_wcvtp()?,
            RCVT => self.op_rcvt()?,
            GC0 | GC1 => self.op_gc(raw_opcode)?,
            SCFS => self.op_scfs()?,
            MD0 | MD1 => self.op_md(raw_opcode)?,
            MPPEM => self.op_mppem()?,
            MPS => self.op_mps()?,
            FLIPON => self.op_flipon()?,
            FLIPOFF => self.op_flipoff()?,
            // Should be unused in production fonts, but we may want to
            // support debugging at some point. Just pops a value from
            // the stack.
            DEBUG => {
                self.value_stack.pop()?;
            }
            LT => self.op_lt()?,
            LTEQ => self.op_lteq()?,
            GT => self.op_gt()?,
            GTEQ => self.op_gteq()?,
            EQ => self.op_eq()?,
            NEQ => self.op_neq()?,
            ODD => self.op_odd()?,
            EVEN => self.op_even()?,
            IF => self.op_if()?,
            EIF => self.op_eif()?,
            AND => self.op_and()?,
            OR => self.op_or()?,
            NOT => self.op_not()?,
            DELTAP1 => self.op_deltap(opcode)?,
            SDB => self.op_sdb()?,
            SDS => self.op_sds()?,
            ADD => self.op_add()?,
            SUB => self.op_sub()?,
            DIV => self.op_div()?,
            MUL => self.op_mul()?,
            ABS => self.op_abs()?,
            NEG => self.op_neg()?,
            FLOOR => self.op_floor()?,
            CEILING => self.op_ceiling()?,
            ROUND00 | ROUND01 | ROUND10 | ROUND11 => self.op_round()?,
            // "No round" means do nothing :)
            NROUND00 | NROUND01 | NROUND10 | NROUND11 => {}
            WCVTF => self.op_wcvtf()?,
            DELTAP2 | DELTAP3 => self.op_deltap(opcode)?,
            DELTAC1 | DELTAC2 | DELTAC3 => self.op_deltac(opcode)?,
            SROUND => self.op_sround()?,
            S45ROUND => self.op_s45round()?,
            JROT => self.op_jrot()?,
            JROF => self.op_jrof()?,
            ROFF => self.op_roff()?,
            // UNUSED: 0x7B
            RUTG => self.op_rutg()?,
            RDTG => self.op_rdtg()?,
            SANGW => self.op_sangw()?,
            // Unsupported instruction, do nothing
            AA => {}
            FLIPPT => self.op_flippt()?,
            FLIPRGON => self.op_fliprgon()?,
            FLIPRGOFF => self.op_fliprgoff()?,
            // UNUSED: 0x83 | 0x84
            SCANCTRL => self.op_scanctrl()?,
            SDPVTL0 | SDPVTL1 => self.op_sdpvtl(raw_opcode)?,
            GETINFO => self.op_getinfo()?,
            IDEF => self.op_idef()?,
            ROLL => self.op_roll()?,
            MAX => self.op_max()?,
            MIN => self.op_min()?,
            SCANTYPE => self.op_scantype()?,
            INSTCTRL => self.op_instctrl()?,
            // UNUSED: 0x8F | 0x90 (ADJUST?)
            GETVARIATION => self.op_getvariation()?,
            GETDATA => self.op_getdata()?,
            _ => {
                // FreeType handles MIRP, MDRP and pushes here.
                // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L7629>
                if opcode >= MIRP00000 {
                    self.op_mirp(raw_opcode)?
                } else if opcode >= MDRP00000 {
                    self.op_mdrp(raw_opcode)?
                } else if opcode >= PUSHB000 {
                    self.op_push(&ins.inline_operands)?;
                } else {
                    return self.op_unknown(opcode as u8);
                }
            }
        }
        Ok(())
    }
}
