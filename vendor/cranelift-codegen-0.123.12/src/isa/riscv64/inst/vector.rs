use crate::Reg;
use crate::isa::riscv64::lower::isle::generated_code::VecAluOpRRRR;
use crate::isa::riscv64::lower::isle::generated_code::{
    VecAMode, VecAluOpRImm5, VecAluOpRR, VecAluOpRRImm5, VecAluOpRRR, VecAluOpRRRImm5, VecAvl,
    VecElementWidth, VecLmul, VecMaskMode, VecOpCategory, VecOpMasking, VecTailMode,
};
use crate::machinst::{OperandVisitor, RegClass};
use core::fmt;

use super::{Type, UImm5};

impl VecAvl {
    pub fn _static(size: u32) -> Self {
        VecAvl::Static {
            size: UImm5::maybe_from_u8(size as u8).expect("Invalid size for AVL"),
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            VecAvl::Static { .. } => true,
        }
    }

    pub fn unwrap_static(&self) -> UImm5 {
        match self {
            VecAvl::Static { size } => *size,
        }
    }
}

// TODO: Can we tell ISLE to derive this?
impl Copy for VecAvl {}

// TODO: Can we tell ISLE to derive this?
impl PartialEq for VecAvl {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VecAvl::Static { size: lhs }, VecAvl::Static { size: rhs }) => lhs == rhs,
        }
    }
}

impl fmt::Display for VecAvl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VecAvl::Static { size } => write!(f, "{size}"),
        }
    }
}

impl VecElementWidth {
    pub fn from_type(ty: Type) -> Self {
        Self::from_bits(ty.lane_bits())
    }

    pub fn from_bits(bits: u32) -> Self {
        match bits {
            8 => VecElementWidth::E8,
            16 => VecElementWidth::E16,
            32 => VecElementWidth::E32,
            64 => VecElementWidth::E64,
            _ => panic!("Invalid number of bits for VecElementWidth: {bits}"),
        }
    }

    pub fn bits(&self) -> u32 {
        match self {
            VecElementWidth::E8 => 8,
            VecElementWidth::E16 => 16,
            VecElementWidth::E32 => 32,
            VecElementWidth::E64 => 64,
        }
    }

    pub fn encode(&self) -> u32 {
        match self {
            VecElementWidth::E8 => 0b000,
            VecElementWidth::E16 => 0b001,
            VecElementWidth::E32 => 0b010,
            VecElementWidth::E64 => 0b011,
        }
    }
}

impl fmt::Display for VecElementWidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "e{}", self.bits())
    }
}

impl VecLmul {
    pub fn encode(&self) -> u32 {
        match self {
            VecLmul::LmulF8 => 0b101,
            VecLmul::LmulF4 => 0b110,
            VecLmul::LmulF2 => 0b111,
            VecLmul::Lmul1 => 0b000,
            VecLmul::Lmul2 => 0b001,
            VecLmul::Lmul4 => 0b010,
            VecLmul::Lmul8 => 0b011,
        }
    }
}

impl fmt::Display for VecLmul {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VecLmul::LmulF8 => write!(f, "mf8"),
            VecLmul::LmulF4 => write!(f, "mf4"),
            VecLmul::LmulF2 => write!(f, "mf2"),
            VecLmul::Lmul1 => write!(f, "m1"),
            VecLmul::Lmul2 => write!(f, "m2"),
            VecLmul::Lmul4 => write!(f, "m4"),
            VecLmul::Lmul8 => write!(f, "m8"),
        }
    }
}

impl VecTailMode {
    pub fn encode(&self) -> u32 {
        match self {
            VecTailMode::Agnostic => 1,
            VecTailMode::Undisturbed => 0,
        }
    }
}

impl fmt::Display for VecTailMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VecTailMode::Agnostic => write!(f, "ta"),
            VecTailMode::Undisturbed => write!(f, "tu"),
        }
    }
}

impl VecMaskMode {
    pub fn encode(&self) -> u32 {
        match self {
            VecMaskMode::Agnostic => 1,
            VecMaskMode::Undisturbed => 0,
        }
    }
}

impl fmt::Display for VecMaskMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VecMaskMode::Agnostic => write!(f, "ma"),
            VecMaskMode::Undisturbed => write!(f, "mu"),
        }
    }
}

/// Vector Type (VType)
///
/// vtype provides the default type used to interpret the contents of the vector register file.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VType {
    pub sew: VecElementWidth,
    pub lmul: VecLmul,
    pub tail_mode: VecTailMode,
    pub mask_mode: VecMaskMode,
}

impl VType {
    // https://github.com/riscv/riscv-v-spec/blob/master/vtype-format.adoc
    pub fn encode(&self) -> u32 {
        let mut bits = 0;
        bits |= self.lmul.encode();
        bits |= self.sew.encode() << 3;
        bits |= self.tail_mode.encode() << 6;
        bits |= self.mask_mode.encode() << 7;
        bits
    }
}

impl fmt::Display for VType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.sew, self.lmul, self.tail_mode, self.mask_mode
        )
    }
}

/// Vector State (VState)
///
/// VState represents the state of the vector unit that each instruction expects before execution.
/// Unlike VType or any of the other types here, VState is not a part of the RISC-V ISA. It is
/// used by our instruction emission code to ensure that the vector unit is in the correct state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VState {
    pub avl: VecAvl,
    pub vtype: VType,
}

impl VState {
    pub fn from_type(ty: Type) -> Self {
        VState {
            avl: VecAvl::_static(ty.lane_count()),
            vtype: VType {
                sew: VecElementWidth::from_type(ty),
                lmul: VecLmul::Lmul1,
                tail_mode: VecTailMode::Agnostic,
                mask_mode: VecMaskMode::Agnostic,
            },
        }
    }
}

impl fmt::Display for VState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#avl={}, #vtype=({})", self.avl, self.vtype)
    }
}

impl VecOpCategory {
    pub fn encode(&self) -> u32 {
        // See: https://github.com/riscv/riscv-v-spec/blob/master/v-spec.adoc#101-vector-arithmetic-instruction-encoding
        match self {
            VecOpCategory::OPIVV => 0b000,
            VecOpCategory::OPFVV => 0b001,
            VecOpCategory::OPMVV => 0b010,
            VecOpCategory::OPIVI => 0b011,
            VecOpCategory::OPIVX => 0b100,
            VecOpCategory::OPFVF => 0b101,
            VecOpCategory::OPMVX => 0b110,
            VecOpCategory::OPCFG => 0b111,
        }
    }
}

impl Copy for VecOpMasking {}
impl VecOpMasking {
    pub fn is_enabled(&self) -> bool {
        match self {
            VecOpMasking::Enabled { .. } => true,
            VecOpMasking::Disabled => false,
        }
    }

    pub fn encode(&self) -> u32 {
        match self {
            VecOpMasking::Enabled { .. } => 0,
            VecOpMasking::Disabled => 1,
        }
    }
}

impl VecAluOpRRRR {
    pub fn opcode(&self) -> u32 {
        // Vector Opcode
        0x57
    }
    pub fn funct3(&self) -> u32 {
        self.category().encode()
    }

    pub fn funct6(&self) -> u32 {
        // See: https://github.com/riscv/riscv-v-spec/blob/master/inst-table.adoc
        match self {
            VecAluOpRRRR::VmaccVV | VecAluOpRRRR::VmaccVX => 0b101101,
            VecAluOpRRRR::VnmsacVV | VecAluOpRRRR::VnmsacVX => 0b101111,
            VecAluOpRRRR::VfmaccVV | VecAluOpRRRR::VfmaccVF => 0b101100,
            VecAluOpRRRR::VfnmaccVV | VecAluOpRRRR::VfnmaccVF => 0b101101,
            VecAluOpRRRR::VfmsacVV | VecAluOpRRRR::VfmsacVF => 0b101110,
            VecAluOpRRRR::VfnmsacVV | VecAluOpRRRR::VfnmsacVF => 0b101111,
            VecAluOpRRRR::Vslide1upVX => 0b001110,
        }
    }

    pub fn category(&self) -> VecOpCategory {
        match self {
            VecAluOpRRRR::VmaccVV | VecAluOpRRRR::VnmsacVV => VecOpCategory::OPMVV,
            VecAluOpRRRR::VmaccVX | VecAluOpRRRR::VnmsacVX | VecAluOpRRRR::Vslide1upVX => {
                VecOpCategory::OPMVX
            }
            VecAluOpRRRR::VfmaccVV
            | VecAluOpRRRR::VfnmaccVV
            | VecAluOpRRRR::VfmsacVV
            | VecAluOpRRRR::VfnmsacVV => VecOpCategory::OPFVV,
            VecAluOpRRRR::VfmaccVF
            | VecAluOpRRRR::VfnmaccVF
            | VecAluOpRRRR::VfmsacVF
            | VecAluOpRRRR::VfnmsacVF => VecOpCategory::OPFVF,
        }
    }

    // vs1 is the only variable source, vs2 is fixed.
    pub fn vs1_regclass(&self) -> RegClass {
        match self.category() {
            VecOpCategory::OPMVV | VecOpCategory::OPFVV => RegClass::Vector,
            VecOpCategory::OPMVX => RegClass::Int,
            VecOpCategory::OPFVF => RegClass::Float,
            _ => unreachable!(),
        }
    }
}

impl VecInstOverlapInfo for VecAluOpRRRR {
    fn forbids_src_dst_overlaps(&self) -> bool {
        match self {
            VecAluOpRRRR::Vslide1upVX => true,
            _ => false,
        }
    }
}

impl fmt::Display for VecAluOpRRRR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{self:?}");
        s.make_ascii_lowercase();
        let (opcode, category) = s.split_at(s.len() - 2);
        f.write_str(&format!("{opcode}.{category}"))
    }
}

impl VecAluOpRRRImm5 {
    pub fn opcode(&self) -> u32 {
        // Vector Opcode
        0x57
    }
    pub fn funct3(&self) -> u32 {
        self.category().encode()
    }

    pub fn funct6(&self) -> u32 {
        // See: https://github.com/riscv/riscv-v-spec/blob/master/inst-table.adoc
        match self {
            VecAluOpRRRImm5::VslideupVI => 0b001110,
        }
    }

    pub fn category(&self) -> VecOpCategory {
        match self {
            VecAluOpRRRImm5::VslideupVI => VecOpCategory::OPIVI,
        }
    }

    pub fn imm_is_unsigned(&self) -> bool {
        match self {
            VecAluOpRRRImm5::VslideupVI => true,
        }
    }
}

impl VecInstOverlapInfo for VecAluOpRRRImm5 {
    fn forbids_src_dst_overlaps(&self) -> bool {
        match self {
            VecAluOpRRRImm5::VslideupVI => true,
        }
    }
}

impl fmt::Display for VecAluOpRRRImm5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{self:?}");
        s.make_ascii_lowercase();
        let (opcode, category) = s.split_at(s.len() - 2);
        f.write_str(&format!("{opcode}.{category}"))
    }
}

impl VecAluOpRRR {
    pub fn opcode(&self) -> u32 {
        // Vector Opcode
        0x57
    }
    pub fn funct3(&self) -> u32 {
        self.category().encode()
    }
    pub fn funct6(&self) -> u32 {
        // See: https://github.com/riscv/riscv-v-spec/blob/master/inst-table.adoc
        match self {
            VecAluOpRRR::VaddVV
            | VecAluOpRRR::VaddVX
            | VecAluOpRRR::VfaddVV
            | VecAluOpRRR::VfaddVF => 0b000000,
            VecAluOpRRR::VsubVV
            | VecAluOpRRR::VsubVX
            | VecAluOpRRR::VfsubVV
            | VecAluOpRRR::VfsubVF => 0b000010,
            VecAluOpRRR::VrsubVX => 0b000011,
            VecAluOpRRR::VmulVV | VecAluOpRRR::VmulVX => 0b100101,
            VecAluOpRRR::VmulhVV | VecAluOpRRR::VmulhVX => 0b100111,
            VecAluOpRRR::VmulhuVV
            | VecAluOpRRR::VmulhuVX
            | VecAluOpRRR::VfmulVV
            | VecAluOpRRR::VfmulVF => 0b100100,
            VecAluOpRRR::VsmulVV | VecAluOpRRR::VsmulVX => 0b100111,
            VecAluOpRRR::VsllVV | VecAluOpRRR::VsllVX => 0b100101,
            VecAluOpRRR::VsrlVV | VecAluOpRRR::VsrlVX => 0b101000,
            VecAluOpRRR::VsraVV | VecAluOpRRR::VsraVX => 0b101001,
            VecAluOpRRR::VandVV | VecAluOpRRR::VandVX => 0b001001,
            VecAluOpRRR::VorVV | VecAluOpRRR::VorVX => 0b001010,
            VecAluOpRRR::VxorVV | VecAluOpRRR::VxorVX => 0b001011,
            VecAluOpRRR::VminuVV | VecAluOpRRR::VminuVX | VecAluOpRRR::VredminuVS => 0b000100,
            VecAluOpRRR::VminVV | VecAluOpRRR::VminVX => 0b000101,
            VecAluOpRRR::VmaxuVV | VecAluOpRRR::VmaxuVX | VecAluOpRRR::VredmaxuVS => 0b000110,
            VecAluOpRRR::VmaxVV | VecAluOpRRR::VmaxVX => 0b000111,
            VecAluOpRRR::VslidedownVX => 0b001111,
            VecAluOpRRR::VfrsubVF => 0b100111,
            VecAluOpRRR::VmergeVVM
            | VecAluOpRRR::VmergeVXM
            | VecAluOpRRR::VfmergeVFM
            | VecAluOpRRR::VcompressVM => 0b010111,
            VecAluOpRRR::VfdivVV
            | VecAluOpRRR::VfdivVF
            | VecAluOpRRR::VsadduVV
            | VecAluOpRRR::VsadduVX => 0b100000,
            VecAluOpRRR::VfrdivVF | VecAluOpRRR::VsaddVV | VecAluOpRRR::VsaddVX => 0b100001,
            VecAluOpRRR::VfminVV => 0b000100,
            VecAluOpRRR::VfmaxVV => 0b000110,
            VecAluOpRRR::VssubuVV | VecAluOpRRR::VssubuVX => 0b100010,
            VecAluOpRRR::VssubVV | VecAluOpRRR::VssubVX => 0b100011,
            VecAluOpRRR::VfsgnjVV | VecAluOpRRR::VfsgnjVF => 0b001000,
            VecAluOpRRR::VfsgnjnVV => 0b001001,
            VecAluOpRRR::VfsgnjxVV => 0b001010,
            VecAluOpRRR::VrgatherVV | VecAluOpRRR::VrgatherVX => 0b001100,
            VecAluOpRRR::VwadduVV | VecAluOpRRR::VwadduVX => 0b110000,
            VecAluOpRRR::VwaddVV | VecAluOpRRR::VwaddVX => 0b110001,
            VecAluOpRRR::VwsubuVV | VecAluOpRRR::VwsubuVX => 0b110010,
            VecAluOpRRR::VwsubVV | VecAluOpRRR::VwsubVX => 0b110011,
            VecAluOpRRR::VwadduWV | VecAluOpRRR::VwadduWX => 0b110100,
            VecAluOpRRR::VwaddWV | VecAluOpRRR::VwaddWX => 0b110101,
            VecAluOpRRR::VwsubuWV | VecAluOpRRR::VwsubuWX => 0b110110,
            VecAluOpRRR::VwsubWV | VecAluOpRRR::VwsubWX => 0b110111,
            VecAluOpRRR::VmseqVV
            | VecAluOpRRR::VmseqVX
            | VecAluOpRRR::VmfeqVV
            | VecAluOpRRR::VmfeqVF => 0b011000,
            VecAluOpRRR::VmsneVV
            | VecAluOpRRR::VmsneVX
            | VecAluOpRRR::VmfleVV
            | VecAluOpRRR::VmfleVF
            | VecAluOpRRR::VmandMM => 0b011001,
            VecAluOpRRR::VmsltuVV | VecAluOpRRR::VmsltuVX | VecAluOpRRR::VmorMM => 0b011010,
            VecAluOpRRR::VmsltVV
            | VecAluOpRRR::VmsltVX
            | VecAluOpRRR::VmfltVV
            | VecAluOpRRR::VmfltVF => 0b011011,
            VecAluOpRRR::VmsleuVV
            | VecAluOpRRR::VmsleuVX
            | VecAluOpRRR::VmfneVV
            | VecAluOpRRR::VmfneVF => 0b011100,
            VecAluOpRRR::VmsleVV
            | VecAluOpRRR::VmsleVX
            | VecAluOpRRR::VmfgtVF
            | VecAluOpRRR::VmnandMM => 0b011101,
            VecAluOpRRR::VmsgtuVX | VecAluOpRRR::VmnorMM => 0b011110,
            VecAluOpRRR::VmsgtVX | VecAluOpRRR::VmfgeVF => 0b011111,
        }
    }

    pub fn category(&self) -> VecOpCategory {
        match self {
            VecAluOpRRR::VaddVV
            | VecAluOpRRR::VsaddVV
            | VecAluOpRRR::VsadduVV
            | VecAluOpRRR::VsubVV
            | VecAluOpRRR::VssubVV
            | VecAluOpRRR::VssubuVV
            | VecAluOpRRR::VsmulVV
            | VecAluOpRRR::VsllVV
            | VecAluOpRRR::VsrlVV
            | VecAluOpRRR::VsraVV
            | VecAluOpRRR::VandVV
            | VecAluOpRRR::VorVV
            | VecAluOpRRR::VxorVV
            | VecAluOpRRR::VminuVV
            | VecAluOpRRR::VminVV
            | VecAluOpRRR::VmaxuVV
            | VecAluOpRRR::VmaxVV
            | VecAluOpRRR::VmergeVVM
            | VecAluOpRRR::VrgatherVV
            | VecAluOpRRR::VmseqVV
            | VecAluOpRRR::VmsneVV
            | VecAluOpRRR::VmsltuVV
            | VecAluOpRRR::VmsltVV
            | VecAluOpRRR::VmsleuVV
            | VecAluOpRRR::VmsleVV => VecOpCategory::OPIVV,
            VecAluOpRRR::VwaddVV
            | VecAluOpRRR::VwaddWV
            | VecAluOpRRR::VwadduVV
            | VecAluOpRRR::VwadduWV
            | VecAluOpRRR::VwsubVV
            | VecAluOpRRR::VwsubWV
            | VecAluOpRRR::VwsubuVV
            | VecAluOpRRR::VwsubuWV
            | VecAluOpRRR::VmulVV
            | VecAluOpRRR::VmulhVV
            | VecAluOpRRR::VmulhuVV
            | VecAluOpRRR::VredmaxuVS
            | VecAluOpRRR::VredminuVS
            | VecAluOpRRR::VcompressVM
            | VecAluOpRRR::VmandMM
            | VecAluOpRRR::VmorMM
            | VecAluOpRRR::VmnandMM
            | VecAluOpRRR::VmnorMM => VecOpCategory::OPMVV,
            VecAluOpRRR::VwaddVX
            | VecAluOpRRR::VwadduVX
            | VecAluOpRRR::VwadduWX
            | VecAluOpRRR::VwaddWX
            | VecAluOpRRR::VwsubVX
            | VecAluOpRRR::VwsubuVX
            | VecAluOpRRR::VwsubuWX
            | VecAluOpRRR::VwsubWX
            | VecAluOpRRR::VmulVX
            | VecAluOpRRR::VmulhVX
            | VecAluOpRRR::VmulhuVX => VecOpCategory::OPMVX,
            VecAluOpRRR::VaddVX
            | VecAluOpRRR::VsaddVX
            | VecAluOpRRR::VsadduVX
            | VecAluOpRRR::VsubVX
            | VecAluOpRRR::VssubVX
            | VecAluOpRRR::VssubuVX
            | VecAluOpRRR::VrsubVX
            | VecAluOpRRR::VsmulVX
            | VecAluOpRRR::VsllVX
            | VecAluOpRRR::VsrlVX
            | VecAluOpRRR::VsraVX
            | VecAluOpRRR::VandVX
            | VecAluOpRRR::VorVX
            | VecAluOpRRR::VxorVX
            | VecAluOpRRR::VminuVX
            | VecAluOpRRR::VminVX
            | VecAluOpRRR::VmaxuVX
            | VecAluOpRRR::VmaxVX
            | VecAluOpRRR::VslidedownVX
            | VecAluOpRRR::VmergeVXM
            | VecAluOpRRR::VrgatherVX
            | VecAluOpRRR::VmseqVX
            | VecAluOpRRR::VmsneVX
            | VecAluOpRRR::VmsltuVX
            | VecAluOpRRR::VmsltVX
            | VecAluOpRRR::VmsleuVX
            | VecAluOpRRR::VmsleVX
            | VecAluOpRRR::VmsgtuVX
            | VecAluOpRRR::VmsgtVX => VecOpCategory::OPIVX,
            VecAluOpRRR::VfaddVV
            | VecAluOpRRR::VfsubVV
            | VecAluOpRRR::VfmulVV
            | VecAluOpRRR::VfdivVV
            | VecAluOpRRR::VfmaxVV
            | VecAluOpRRR::VfminVV
            | VecAluOpRRR::VfsgnjVV
            | VecAluOpRRR::VfsgnjnVV
            | VecAluOpRRR::VfsgnjxVV
            | VecAluOpRRR::VmfeqVV
            | VecAluOpRRR::VmfneVV
            | VecAluOpRRR::VmfltVV
            | VecAluOpRRR::VmfleVV => VecOpCategory::OPFVV,
            VecAluOpRRR::VfaddVF
            | VecAluOpRRR::VfsubVF
            | VecAluOpRRR::VfrsubVF
            | VecAluOpRRR::VfmulVF
            | VecAluOpRRR::VfdivVF
            | VecAluOpRRR::VfrdivVF
            | VecAluOpRRR::VfmergeVFM
            | VecAluOpRRR::VfsgnjVF
            | VecAluOpRRR::VmfeqVF
            | VecAluOpRRR::VmfneVF
            | VecAluOpRRR::VmfltVF
            | VecAluOpRRR::VmfleVF
            | VecAluOpRRR::VmfgtVF
            | VecAluOpRRR::VmfgeVF => VecOpCategory::OPFVF,
        }
    }

    // vs1 is the only variable source, vs2 is fixed.
    pub fn vs1_regclass(&self) -> RegClass {
        match self.category() {
            VecOpCategory::OPIVV | VecOpCategory::OPFVV | VecOpCategory::OPMVV => RegClass::Vector,
            VecOpCategory::OPIVX | VecOpCategory::OPMVX => RegClass::Int,
            VecOpCategory::OPFVF => RegClass::Float,
            _ => unreachable!(),
        }
    }
}

impl VecInstOverlapInfo for VecAluOpRRR {
    fn forbids_src_dst_overlaps(&self) -> bool {
        match self {
            VecAluOpRRR::VrgatherVV
            | VecAluOpRRR::VrgatherVX
            | VecAluOpRRR::VcompressVM
            | VecAluOpRRR::VwadduVV
            | VecAluOpRRR::VwadduVX
            | VecAluOpRRR::VwaddVV
            | VecAluOpRRR::VwaddVX
            | VecAluOpRRR::VwadduWV
            | VecAluOpRRR::VwadduWX
            | VecAluOpRRR::VwaddWV
            | VecAluOpRRR::VwaddWX
            | VecAluOpRRR::VwsubuVV
            | VecAluOpRRR::VwsubuVX
            | VecAluOpRRR::VwsubVV
            | VecAluOpRRR::VwsubVX
            | VecAluOpRRR::VwsubuWV
            | VecAluOpRRR::VwsubuWX
            | VecAluOpRRR::VwsubWV
            | VecAluOpRRR::VwsubWX => true,
            _ => false,
        }
    }

    // Only mask writing operations, and reduction operations (`vred*`) allow mask / dst overlaps.
    fn forbids_mask_dst_overlaps(&self) -> bool {
        match self {
            VecAluOpRRR::VredmaxuVS
            | VecAluOpRRR::VredminuVS
            | VecAluOpRRR::VmandMM
            | VecAluOpRRR::VmorMM
            | VecAluOpRRR::VmnandMM
            | VecAluOpRRR::VmnorMM
            | VecAluOpRRR::VmseqVX
            | VecAluOpRRR::VmsneVX
            | VecAluOpRRR::VmsltuVX
            | VecAluOpRRR::VmsltVX
            | VecAluOpRRR::VmsleuVX
            | VecAluOpRRR::VmsleVX
            | VecAluOpRRR::VmsgtuVX
            | VecAluOpRRR::VmsgtVX
            | VecAluOpRRR::VmfeqVV
            | VecAluOpRRR::VmfneVV
            | VecAluOpRRR::VmfltVV
            | VecAluOpRRR::VmfleVV
            | VecAluOpRRR::VmfeqVF
            | VecAluOpRRR::VmfneVF
            | VecAluOpRRR::VmfltVF
            | VecAluOpRRR::VmfleVF
            | VecAluOpRRR::VmfgtVF
            | VecAluOpRRR::VmfgeVF => false,
            _ => true,
        }
    }
}

impl fmt::Display for VecAluOpRRR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suffix_length = match self {
            VecAluOpRRR::VmergeVVM | VecAluOpRRR::VmergeVXM | VecAluOpRRR::VfmergeVFM => 3,
            _ => 2,
        };

        let mut s = format!("{self:?}");
        s.make_ascii_lowercase();
        let (opcode, category) = s.split_at(s.len() - suffix_length);
        f.write_str(&format!("{opcode}.{category}"))
    }
}

impl VecAluOpRRImm5 {
    pub fn opcode(&self) -> u32 {
        // Vector Opcode
        0x57
    }
    pub fn funct3(&self) -> u32 {
        self.category().encode()
    }

    pub fn funct6(&self) -> u32 {
        // See: https://github.com/riscv/riscv-v-spec/blob/master/inst-table.adoc
        match self {
            VecAluOpRRImm5::VaddVI => 0b000000,
            VecAluOpRRImm5::VrsubVI => 0b000011,
            VecAluOpRRImm5::VsllVI => 0b100101,
            VecAluOpRRImm5::VsrlVI => 0b101000,
            VecAluOpRRImm5::VsraVI => 0b101001,
            VecAluOpRRImm5::VandVI => 0b001001,
            VecAluOpRRImm5::VorVI => 0b001010,
            VecAluOpRRImm5::VxorVI => 0b001011,
            VecAluOpRRImm5::VslidedownVI => 0b001111,
            VecAluOpRRImm5::VssrlVI => 0b101010,
            VecAluOpRRImm5::VmergeVIM => 0b010111,
            VecAluOpRRImm5::VsadduVI => 0b100000,
            VecAluOpRRImm5::VsaddVI => 0b100001,
            VecAluOpRRImm5::VrgatherVI => 0b001100,
            VecAluOpRRImm5::VmvrV => 0b100111,
            VecAluOpRRImm5::VnclipWI => 0b101111,
            VecAluOpRRImm5::VnclipuWI => 0b101110,
            VecAluOpRRImm5::VmseqVI => 0b011000,
            VecAluOpRRImm5::VmsneVI => 0b011001,
            VecAluOpRRImm5::VmsleuVI => 0b011100,
            VecAluOpRRImm5::VmsleVI => 0b011101,
            VecAluOpRRImm5::VmsgtuVI => 0b011110,
            VecAluOpRRImm5::VmsgtVI => 0b011111,
        }
    }

    pub fn category(&self) -> VecOpCategory {
        match self {
            VecAluOpRRImm5::VaddVI
            | VecAluOpRRImm5::VrsubVI
            | VecAluOpRRImm5::VsllVI
            | VecAluOpRRImm5::VsrlVI
            | VecAluOpRRImm5::VsraVI
            | VecAluOpRRImm5::VandVI
            | VecAluOpRRImm5::VorVI
            | VecAluOpRRImm5::VxorVI
            | VecAluOpRRImm5::VssrlVI
            | VecAluOpRRImm5::VslidedownVI
            | VecAluOpRRImm5::VmergeVIM
            | VecAluOpRRImm5::VsadduVI
            | VecAluOpRRImm5::VsaddVI
            | VecAluOpRRImm5::VrgatherVI
            | VecAluOpRRImm5::VmvrV
            | VecAluOpRRImm5::VnclipWI
            | VecAluOpRRImm5::VnclipuWI
            | VecAluOpRRImm5::VmseqVI
            | VecAluOpRRImm5::VmsneVI
            | VecAluOpRRImm5::VmsleuVI
            | VecAluOpRRImm5::VmsleVI
            | VecAluOpRRImm5::VmsgtuVI
            | VecAluOpRRImm5::VmsgtVI => VecOpCategory::OPIVI,
        }
    }

    pub fn imm_is_unsigned(&self) -> bool {
        match self {
            VecAluOpRRImm5::VsllVI
            | VecAluOpRRImm5::VsrlVI
            | VecAluOpRRImm5::VssrlVI
            | VecAluOpRRImm5::VsraVI
            | VecAluOpRRImm5::VslidedownVI
            | VecAluOpRRImm5::VrgatherVI
            | VecAluOpRRImm5::VmvrV
            | VecAluOpRRImm5::VnclipWI
            | VecAluOpRRImm5::VnclipuWI => true,
            VecAluOpRRImm5::VaddVI
            | VecAluOpRRImm5::VrsubVI
            | VecAluOpRRImm5::VandVI
            | VecAluOpRRImm5::VorVI
            | VecAluOpRRImm5::VxorVI
            | VecAluOpRRImm5::VmergeVIM
            | VecAluOpRRImm5::VsadduVI
            | VecAluOpRRImm5::VsaddVI
            | VecAluOpRRImm5::VmseqVI
            | VecAluOpRRImm5::VmsneVI
            | VecAluOpRRImm5::VmsleuVI
            | VecAluOpRRImm5::VmsleVI
            | VecAluOpRRImm5::VmsgtuVI
            | VecAluOpRRImm5::VmsgtVI => false,
        }
    }
}

impl VecInstOverlapInfo for VecAluOpRRImm5 {
    fn forbids_src_dst_overlaps(&self) -> bool {
        match self {
            VecAluOpRRImm5::VrgatherVI => true,
            _ => false,
        }
    }

    // Only mask writing operations, and reduction operations (`vred*`) allow mask / dst overlaps.
    fn forbids_mask_dst_overlaps(&self) -> bool {
        match self {
            VecAluOpRRImm5::VmseqVI
            | VecAluOpRRImm5::VmsneVI
            | VecAluOpRRImm5::VmsleuVI
            | VecAluOpRRImm5::VmsleVI
            | VecAluOpRRImm5::VmsgtuVI
            | VecAluOpRRImm5::VmsgtVI => false,
            _ => true,
        }
    }
}

impl fmt::Display for VecAluOpRRImm5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suffix_length = match self {
            VecAluOpRRImm5::VmergeVIM => 3,
            _ => 2,
        };

        let mut s = format!("{self:?}");
        s.make_ascii_lowercase();
        let (opcode, category) = s.split_at(s.len() - suffix_length);
        f.write_str(&format!("{opcode}.{category}"))
    }
}

impl VecAluOpRR {
    pub fn opcode(&self) -> u32 {
        // Vector Opcode
        0x57
    }

    pub fn funct3(&self) -> u32 {
        self.category().encode()
    }

    pub fn funct6(&self) -> u32 {
        // See: https://github.com/riscv/riscv-v-spec/blob/master/inst-table.adoc
        match self {
            VecAluOpRR::VmvSX | VecAluOpRR::VmvXS | VecAluOpRR::VfmvSF | VecAluOpRR::VfmvFS => {
                0b010000
            }
            VecAluOpRR::VzextVF2
            | VecAluOpRR::VzextVF4
            | VecAluOpRR::VzextVF8
            | VecAluOpRR::VsextVF2
            | VecAluOpRR::VsextVF4
            | VecAluOpRR::VsextVF8 => 0b010010,
            VecAluOpRR::VfsqrtV => 0b010011,
            VecAluOpRR::VmvVV | VecAluOpRR::VmvVX | VecAluOpRR::VfmvVF => 0b010111,
            VecAluOpRR::VfcvtxufV
            | VecAluOpRR::VfcvtxfV
            | VecAluOpRR::VfcvtrtzxufV
            | VecAluOpRR::VfcvtrtzxfV
            | VecAluOpRR::VfcvtfxuV
            | VecAluOpRR::VfcvtfxV
            | VecAluOpRR::VfwcvtffV
            | VecAluOpRR::VfncvtffW => 0b010010,
        }
    }

    pub fn category(&self) -> VecOpCategory {
        match self {
            VecAluOpRR::VmvSX => VecOpCategory::OPMVX,
            VecAluOpRR::VmvXS
            | VecAluOpRR::VzextVF2
            | VecAluOpRR::VzextVF4
            | VecAluOpRR::VzextVF8
            | VecAluOpRR::VsextVF2
            | VecAluOpRR::VsextVF4
            | VecAluOpRR::VsextVF8 => VecOpCategory::OPMVV,
            VecAluOpRR::VfmvSF | VecAluOpRR::VfmvVF => VecOpCategory::OPFVF,
            VecAluOpRR::VfmvFS
            | VecAluOpRR::VfsqrtV
            | VecAluOpRR::VfcvtxufV
            | VecAluOpRR::VfcvtxfV
            | VecAluOpRR::VfcvtrtzxufV
            | VecAluOpRR::VfcvtrtzxfV
            | VecAluOpRR::VfcvtfxuV
            | VecAluOpRR::VfcvtfxV
            | VecAluOpRR::VfwcvtffV
            | VecAluOpRR::VfncvtffW => VecOpCategory::OPFVV,
            VecAluOpRR::VmvVV => VecOpCategory::OPIVV,
            VecAluOpRR::VmvVX => VecOpCategory::OPIVX,
        }
    }

    /// Returns the auxiliary encoding field for the instruction, if any.
    pub fn aux_encoding(&self) -> u32 {
        match self {
            // VRXUNARY0
            VecAluOpRR::VmvSX => 0b00000,
            // VWXUNARY0
            VecAluOpRR::VmvXS => 0b00000,
            // VRFUNARY0
            VecAluOpRR::VfmvSF => 0b00000,
            // VWFUNARY0
            VecAluOpRR::VfmvFS => 0b00000,
            // VFUNARY1
            VecAluOpRR::VfsqrtV => 0b00000,
            // VXUNARY0
            VecAluOpRR::VzextVF8 => 0b00010,
            VecAluOpRR::VsextVF8 => 0b00011,
            VecAluOpRR::VzextVF4 => 0b00100,
            VecAluOpRR::VsextVF4 => 0b00101,
            VecAluOpRR::VzextVF2 => 0b00110,
            VecAluOpRR::VsextVF2 => 0b00111,
            // VFUNARY0
            // single-width converts
            VecAluOpRR::VfcvtxufV => 0b00000,
            VecAluOpRR::VfcvtxfV => 0b00001,
            VecAluOpRR::VfcvtrtzxufV => 0b00110,
            VecAluOpRR::VfcvtrtzxfV => 0b00111,
            VecAluOpRR::VfcvtfxuV => 0b00010,
            VecAluOpRR::VfcvtfxV => 0b00011,
            // widening converts
            VecAluOpRR::VfwcvtffV => 0b01100,
            // narrowing converts
            VecAluOpRR::VfncvtffW => 0b10100,
            // These don't have a explicit encoding table, but Section 11.16 Vector Integer Move Instruction states:
            // > The first operand specifier (vs2) must contain v0, and any other vector register number in vs2 is reserved.
            VecAluOpRR::VmvVV | VecAluOpRR::VmvVX | VecAluOpRR::VfmvVF => 0,
        }
    }

    /// Most of these opcodes have the source register encoded in the VS2 field and
    /// the `aux_encoding` field in VS1. However some special snowflakes have it the
    /// other way around. As far as I can tell only vmv.v.* are backwards.
    pub fn vs_is_vs2_encoded(&self) -> bool {
        match self {
            VecAluOpRR::VmvXS
            | VecAluOpRR::VfmvFS
            | VecAluOpRR::VfsqrtV
            | VecAluOpRR::VzextVF2
            | VecAluOpRR::VzextVF4
            | VecAluOpRR::VzextVF8
            | VecAluOpRR::VsextVF2
            | VecAluOpRR::VsextVF4
            | VecAluOpRR::VsextVF8
            | VecAluOpRR::VfcvtxufV
            | VecAluOpRR::VfcvtxfV
            | VecAluOpRR::VfcvtrtzxufV
            | VecAluOpRR::VfcvtrtzxfV
            | VecAluOpRR::VfcvtfxuV
            | VecAluOpRR::VfcvtfxV
            | VecAluOpRR::VfwcvtffV
            | VecAluOpRR::VfncvtffW => true,
            VecAluOpRR::VmvSX
            | VecAluOpRR::VfmvSF
            | VecAluOpRR::VmvVV
            | VecAluOpRR::VmvVX
            | VecAluOpRR::VfmvVF => false,
        }
    }

    pub fn dst_regclass(&self) -> RegClass {
        match self {
            VecAluOpRR::VfmvSF
            | VecAluOpRR::VmvSX
            | VecAluOpRR::VmvVV
            | VecAluOpRR::VmvVX
            | VecAluOpRR::VfmvVF
            | VecAluOpRR::VfsqrtV
            | VecAluOpRR::VzextVF2
            | VecAluOpRR::VzextVF4
            | VecAluOpRR::VzextVF8
            | VecAluOpRR::VsextVF2
            | VecAluOpRR::VsextVF4
            | VecAluOpRR::VsextVF8
            | VecAluOpRR::VfcvtxufV
            | VecAluOpRR::VfcvtxfV
            | VecAluOpRR::VfcvtrtzxufV
            | VecAluOpRR::VfcvtrtzxfV
            | VecAluOpRR::VfcvtfxuV
            | VecAluOpRR::VfcvtfxV
            | VecAluOpRR::VfwcvtffV
            | VecAluOpRR::VfncvtffW => RegClass::Vector,
            VecAluOpRR::VmvXS => RegClass::Int,
            VecAluOpRR::VfmvFS => RegClass::Float,
        }
    }

    pub fn src_regclass(&self) -> RegClass {
        match self {
            VecAluOpRR::VmvXS
            | VecAluOpRR::VfmvFS
            | VecAluOpRR::VmvVV
            | VecAluOpRR::VfsqrtV
            | VecAluOpRR::VzextVF2
            | VecAluOpRR::VzextVF4
            | VecAluOpRR::VzextVF8
            | VecAluOpRR::VsextVF2
            | VecAluOpRR::VsextVF4
            | VecAluOpRR::VsextVF8
            | VecAluOpRR::VfcvtxufV
            | VecAluOpRR::VfcvtxfV
            | VecAluOpRR::VfcvtrtzxufV
            | VecAluOpRR::VfcvtrtzxfV
            | VecAluOpRR::VfcvtfxuV
            | VecAluOpRR::VfcvtfxV
            | VecAluOpRR::VfwcvtffV
            | VecAluOpRR::VfncvtffW => RegClass::Vector,
            VecAluOpRR::VfmvSF | VecAluOpRR::VfmvVF => RegClass::Float,
            VecAluOpRR::VmvSX | VecAluOpRR::VmvVX => RegClass::Int,
        }
    }
}

impl VecInstOverlapInfo for VecAluOpRR {
    fn forbids_src_dst_overlaps(&self) -> bool {
        match self {
            VecAluOpRR::VzextVF2
            | VecAluOpRR::VzextVF4
            | VecAluOpRR::VzextVF8
            | VecAluOpRR::VsextVF2
            | VecAluOpRR::VsextVF4
            | VecAluOpRR::VsextVF8
            | VecAluOpRR::VfwcvtffV
            | VecAluOpRR::VfncvtffW => true,
            _ => false,
        }
    }
}

impl fmt::Display for VecAluOpRR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            VecAluOpRR::VmvSX => "vmv.s.x",
            VecAluOpRR::VmvXS => "vmv.x.s",
            VecAluOpRR::VfmvSF => "vfmv.s.f",
            VecAluOpRR::VfmvFS => "vfmv.f.s",
            VecAluOpRR::VfsqrtV => "vfsqrt.v",
            VecAluOpRR::VzextVF2 => "vzext.vf2",
            VecAluOpRR::VzextVF4 => "vzext.vf4",
            VecAluOpRR::VzextVF8 => "vzext.vf8",
            VecAluOpRR::VsextVF2 => "vsext.vf2",
            VecAluOpRR::VsextVF4 => "vsext.vf4",
            VecAluOpRR::VsextVF8 => "vsext.vf8",
            VecAluOpRR::VmvVV => "vmv.v.v",
            VecAluOpRR::VmvVX => "vmv.v.x",
            VecAluOpRR::VfmvVF => "vfmv.v.f",
            VecAluOpRR::VfcvtxufV => "vfcvt.xu.f.v",
            VecAluOpRR::VfcvtxfV => "vfcvt.x.f.v",
            VecAluOpRR::VfcvtrtzxufV => "vfcvt.rtz.xu.f.v",
            VecAluOpRR::VfcvtrtzxfV => "vfcvt.rtz.x.f.v",
            VecAluOpRR::VfcvtfxuV => "vfcvt.f.xu.v",
            VecAluOpRR::VfcvtfxV => "vfcvt.f.x.v",
            VecAluOpRR::VfwcvtffV => "vfwcvt.f.f.v",
            VecAluOpRR::VfncvtffW => "vfncvt.f.f.w",
        })
    }
}

impl VecAluOpRImm5 {
    pub fn opcode(&self) -> u32 {
        // Vector Opcode
        0x57
    }
    pub fn funct3(&self) -> u32 {
        self.category().encode()
    }

    pub fn funct6(&self) -> u32 {
        // See: https://github.com/riscv/riscv-v-spec/blob/master/inst-table.adoc
        match self {
            VecAluOpRImm5::VmvVI => 0b010111,
        }
    }

    pub fn category(&self) -> VecOpCategory {
        match self {
            VecAluOpRImm5::VmvVI => VecOpCategory::OPIVI,
        }
    }

    /// Returns the auxiliary encoding field for the instruction, if any.
    pub fn aux_encoding(&self) -> u32 {
        match self {
            // These don't have a explicit encoding table, but Section 11.16 Vector Integer Move Instruction states:
            // > The first operand specifier (vs2) must contain v0, and any other vector register number in vs2 is reserved.
            VecAluOpRImm5::VmvVI => 0,
        }
    }
}

impl VecInstOverlapInfo for VecAluOpRImm5 {
    fn forbids_src_dst_overlaps(&self) -> bool {
        match self {
            VecAluOpRImm5::VmvVI => false,
        }
    }
}

impl fmt::Display for VecAluOpRImm5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            VecAluOpRImm5::VmvVI => "vmv.v.i",
        })
    }
}

impl VecAMode {
    pub fn get_base_register(&self) -> Option<Reg> {
        match self {
            VecAMode::UnitStride { base, .. } => base.get_base_register(),
        }
    }

    pub fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            VecAMode::UnitStride { base, .. } => base.get_operands(collector),
        }
    }

    /// `mop` field, described in Table 7 of Section 7.2. Vector Load/Store Addressing Modes
    /// https://github.com/riscv/riscv-v-spec/blob/master/v-spec.adoc#72-vector-loadstore-addressing-modes
    pub fn mop(&self) -> u32 {
        match self {
            VecAMode::UnitStride { .. } => 0b00,
        }
    }

    /// `lumop` field, described in Table 9 of Section 7.2. Vector Load/Store Addressing Modes
    /// https://github.com/riscv/riscv-v-spec/blob/master/v-spec.adoc#72-vector-loadstore-addressing-modes
    pub fn lumop(&self) -> u32 {
        match self {
            VecAMode::UnitStride { .. } => 0b00000,
        }
    }

    /// `sumop` field, described in Table 10 of Section 7.2. Vector Load/Store Addressing Modes
    /// https://github.com/riscv/riscv-v-spec/blob/master/v-spec.adoc#72-vector-loadstore-addressing-modes
    pub fn sumop(&self) -> u32 {
        match self {
            VecAMode::UnitStride { .. } => 0b00000,
        }
    }

    /// The `nf[2:0]` field encodes the number of fields in each segment. For regular vector loads and
    /// stores, nf=0, indicating that a single value is moved between a vector register group and memory
    /// at each element position. Larger values in the nf field are used to access multiple contiguous
    /// fields within a segment as described in Section 7.8 Vector Load/Store Segment Instructions.
    ///
    /// https://github.com/riscv/riscv-v-spec/blob/master/v-spec.adoc#72-vector-loadstore-addressing-modes
    pub fn nf(&self) -> u32 {
        match self {
            VecAMode::UnitStride { .. } => 0b000,
        }
    }
}

pub trait VecInstOverlapInfo {
    /// § 5.2 Vector Operands states:
    ///
    /// A destination vector register group can overlap a source vector register group
    /// only if one of the following holds:
    ///
    ///  * The destination EEW equals the source EEW.
    ///
    ///  * The destination EEW is smaller than the source EEW and the overlap is
    ///    in the lowest-numbered part of the source register group (e.g., when LMUL=1,
    ///    vnsrl.wi v0, v0, 3 is legal, but a destination of v1 is not).
    ///
    ///  * The destination EEW is greater than the source EEW, the source EMUL is at
    ///    least 1, and the overlap is in the highest-numbered part of the destination register
    ///    group (e.g., when LMUL=8, vzext.vf4 v0, v6 is legal, but a source of v0, v2, or v4 is not).
    ///
    /// For the purpose of determining register group overlap constraints, mask elements have EEW=1.
    fn forbids_src_dst_overlaps(&self) -> bool;

    /// § 5.3 Vector Masking states:
    ///
    /// > The destination vector register group for a masked vector instruction
    /// > cannot overlap the source mask register (v0), unless the destination
    /// > vector register is being written with a mask value (e.g., compares) or
    /// > the scalar result of a reduction. These instruction encodings are reserved.
    ///
    /// In almost all instructions we should not allow the mask to be re-used as
    /// a destination register.
    fn forbids_mask_dst_overlaps(&self) -> bool {
        true
    }

    /// There are two broad categories of overlaps (see above). But we can't represent such
    /// fine grained overlaps to regalloc. So if any of the two come into play we forbid
    /// all source and destination overlaps (including masks).
    fn forbids_overlaps(&self, mask: &VecOpMasking) -> bool {
        self.forbids_src_dst_overlaps() || (mask.is_enabled() && self.forbids_mask_dst_overlaps())
    }
}
