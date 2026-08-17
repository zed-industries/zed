use super::address_transform::AddressTransform;
use super::dbi_log;
use crate::debug::ModuleMemoryOffset;
use crate::debug::transform::debug_transform_logging::{
    dbi_log_enabled, log_get_value_loc, log_get_value_name, log_get_value_ranges,
};
use crate::translate::get_vmctx_value_label;
use anyhow::{Context, Error, Result};
use core::fmt;
use cranelift_codegen::LabelValueLoc;
use cranelift_codegen::ValueLabelsRanges;
use cranelift_codegen::ir::ValueLabel;
use cranelift_codegen::isa::TargetIsa;
use gimli::{Expression, Operation, Reader, ReaderOffset, write};
use itertools::Itertools;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug)]
pub struct FunctionFrameInfo<'a> {
    pub value_ranges: &'a ValueLabelsRanges,
    pub memory_offset: ModuleMemoryOffset,
}

struct ExpressionWriter(write::EndianVec<gimli::RunTimeEndian>);

enum VmctxBase {
    Reg(u16),
    OnStack,
}

impl ExpressionWriter {
    fn new() -> Self {
        let endian = gimli::RunTimeEndian::Little;
        let writer = write::EndianVec::new(endian);
        ExpressionWriter(writer)
    }

    fn write_op(&mut self, op: gimli::DwOp) -> write::Result<()> {
        self.write_u8(op.0)
    }

    fn write_op_reg(&mut self, reg: u16) -> write::Result<()> {
        if reg < 32 {
            self.write_u8(gimli::constants::DW_OP_reg0.0 + reg as u8)
        } else {
            self.write_op(gimli::constants::DW_OP_regx)?;
            self.write_uleb128(reg.into())
        }
    }

    fn write_op_breg(&mut self, reg: u16) -> write::Result<()> {
        if reg < 32 {
            self.write_u8(gimli::constants::DW_OP_breg0.0 + reg as u8)
        } else {
            self.write_op(gimli::constants::DW_OP_bregx)?;
            self.write_uleb128(reg.into())
        }
    }

    fn write_u8(&mut self, b: u8) -> write::Result<()> {
        write::Writer::write_u8(&mut self.0, b)
    }

    fn write_u32(&mut self, b: u32) -> write::Result<()> {
        write::Writer::write_u32(&mut self.0, b)
    }

    fn write_uleb128(&mut self, i: u64) -> write::Result<()> {
        write::Writer::write_uleb128(&mut self.0, i)
    }

    fn write_sleb128(&mut self, i: i64) -> write::Result<()> {
        write::Writer::write_sleb128(&mut self.0, i)
    }

    fn into_vec(self) -> Vec<u8> {
        self.0.into_vec()
    }

    fn gen_address_of_memory_base_pointer(
        &mut self,
        vmctx: VmctxBase,
        memory_base: &ModuleMemoryOffset,
    ) -> write::Result<()> {
        match *memory_base {
            ModuleMemoryOffset::Defined(offset) => match vmctx {
                VmctxBase::Reg(reg) => {
                    self.write_op_breg(reg)?;
                    self.write_sleb128(offset.into())?;
                }
                VmctxBase::OnStack => {
                    self.write_op(gimli::constants::DW_OP_consts)?;
                    self.write_sleb128(offset.into())?;
                    self.write_op(gimli::constants::DW_OP_plus)?;
                }
            },
            ModuleMemoryOffset::Imported {
                offset_to_vm_memory_definition,
                offset_to_memory_base,
            } => {
                match vmctx {
                    VmctxBase::Reg(reg) => {
                        self.write_op_breg(reg)?;
                        self.write_sleb128(offset_to_vm_memory_definition.into())?;
                    }
                    VmctxBase::OnStack => {
                        if offset_to_vm_memory_definition > 0 {
                            self.write_op(gimli::constants::DW_OP_consts)?;
                            self.write_sleb128(offset_to_vm_memory_definition.into())?;
                        }
                        self.write_op(gimli::constants::DW_OP_plus)?;
                    }
                }
                self.write_op(gimli::constants::DW_OP_deref)?;
                if offset_to_memory_base > 0 {
                    self.write_op(gimli::constants::DW_OP_consts)?;
                    self.write_sleb128(offset_to_memory_base.into())?;
                    self.write_op(gimli::constants::DW_OP_plus)?;
                }
            }
            ModuleMemoryOffset::None => return Err(write::Error::InvalidAttributeValue),
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum CompiledExpressionPart {
    // Untranslated DWARF expression.
    Code(Vec<u8>),
    // The wasm-local DWARF operator. The label points to `ValueLabel`.
    // The trailing field denotes that the operator was last in sequence,
    // and it is the DWARF location (not a pointer).
    Local {
        label: ValueLabel,
        trailing: bool,
    },
    // Dereference is needed.
    Deref,
    // Jumping in the expression.
    Jump {
        conditionally: bool,
        target: JumpTargetMarker,
    },
    // Floating landing pad.
    LandingPad(JumpTargetMarker),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledExpression {
    parts: Vec<CompiledExpressionPart>,
    need_deref: bool,
}

impl CompiledExpression {
    pub fn vmctx() -> CompiledExpression {
        CompiledExpression::from_label(get_vmctx_value_label())
    }

    pub fn from_label(label: ValueLabel) -> CompiledExpression {
        CompiledExpression {
            parts: vec![CompiledExpressionPart::Local {
                label,
                trailing: true,
            }],
            need_deref: false,
        }
    }
}

fn translate_loc(
    loc: LabelValueLoc,
    isa: &dyn TargetIsa,
    add_stack_value: bool,
) -> Result<Option<Vec<u8>>> {
    Ok(match loc {
        LabelValueLoc::Reg(r) => {
            let machine_reg = isa.map_regalloc_reg_to_dwarf(r)?;
            let mut writer = ExpressionWriter::new();
            if add_stack_value {
                writer.write_op_reg(machine_reg)?;
            } else {
                writer.write_op_breg(machine_reg)?;
                writer.write_sleb128(0)?;
            }
            Some(writer.into_vec())
        }
        LabelValueLoc::CFAOffset(off) => {
            let mut writer = ExpressionWriter::new();
            writer.write_op(gimli::constants::DW_OP_fbreg)?;
            writer.write_sleb128(off)?;
            if !add_stack_value {
                writer.write_op(gimli::constants::DW_OP_deref)?;
            }
            return Ok(Some(writer.into_vec()));
        }
    })
}

fn append_memory_deref(
    buf: &mut Vec<u8>,
    frame_info: &FunctionFrameInfo,
    vmctx_loc: LabelValueLoc,
    isa: &dyn TargetIsa,
) -> Result<bool> {
    let mut writer = ExpressionWriter::new();
    let vmctx_base = match vmctx_loc {
        LabelValueLoc::Reg(r) => VmctxBase::Reg(isa.map_regalloc_reg_to_dwarf(r)?),
        LabelValueLoc::CFAOffset(off) => {
            writer.write_op(gimli::constants::DW_OP_fbreg)?;
            writer.write_sleb128(off)?;
            writer.write_op(gimli::constants::DW_OP_deref)?;
            VmctxBase::OnStack
        }
    };
    writer.gen_address_of_memory_base_pointer(vmctx_base, &frame_info.memory_offset)?;
    writer.write_op(gimli::constants::DW_OP_deref)?;
    writer.write_op(gimli::constants::DW_OP_swap)?;
    writer.write_op(gimli::constants::DW_OP_const4u)?;
    writer.write_u32(0xffff_ffff)?;
    writer.write_op(gimli::constants::DW_OP_and)?;
    writer.write_op(gimli::constants::DW_OP_plus)?;
    buf.extend(writer.into_vec());
    Ok(true)
}

pub struct BuiltCompiledExpression<TIter> {
    pub expressions: TIter,
    pub covers_entire_scope: bool,
}

impl CompiledExpression {
    pub fn is_simple(&self) -> bool {
        if let [CompiledExpressionPart::Code(_)] = self.parts.as_slice() {
            true
        } else {
            self.parts.is_empty()
        }
    }

    pub fn build(&self) -> Option<write::Expression> {
        if let [CompiledExpressionPart::Code(code)] = self.parts.as_slice() {
            return Some(write::Expression::raw(code.to_vec()));
        }
        // locals found, not supported
        None
    }

    pub fn build_with_locals<'a>(
        &'a self,
        scope: &'a [(u64, u64)], // wasm ranges
        addr_tr: &'a AddressTransform,
        frame_info: Option<&'a FunctionFrameInfo>,
        isa: &'a dyn TargetIsa,
    ) -> BuiltCompiledExpression<
        impl Iterator<Item = Result<(write::Address, u64, write::Expression)>> + use<'a>,
    > {
        enum BuildWithLocalsResult<'a> {
            Empty,
            Simple(
                Box<dyn Iterator<Item = (write::Address, u64)> + 'a>,
                Vec<u8>,
            ),
            Ranges(Box<dyn Iterator<Item = Result<(usize, usize, usize, Vec<u8>)>> + 'a>),
        }
        impl Iterator for BuildWithLocalsResult<'_> {
            type Item = Result<(write::Address, u64, write::Expression)>;
            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    BuildWithLocalsResult::Empty => None,
                    BuildWithLocalsResult::Simple(it, code) => it
                        .next()
                        .map(|(addr, len)| Ok((addr, len, write::Expression::raw(code.to_vec())))),
                    BuildWithLocalsResult::Ranges(it) => it.next().map(|r| {
                        r.map(|(symbol, start, end, code_buf)| {
                            (
                                write::Address::Symbol {
                                    symbol,
                                    addend: start as i64,
                                },
                                (end - start) as u64,
                                write::Expression::raw(code_buf),
                            )
                        })
                    }),
                }
            }
        }

        if scope.is_empty() {
            return BuiltCompiledExpression {
                expressions: BuildWithLocalsResult::Empty,
                covers_entire_scope: false,
            };
        }

        // If it a simple DWARF code, no need in locals processing. Just translate
        // the scope ranges.
        if let [CompiledExpressionPart::Code(code)] = self.parts.as_slice() {
            return BuiltCompiledExpression {
                expressions: BuildWithLocalsResult::Simple(
                    Box::new(scope.iter().flat_map(move |(wasm_start, wasm_end)| {
                        addr_tr.translate_ranges(*wasm_start, *wasm_end)
                    })),
                    code.clone(),
                ),
                covers_entire_scope: false,
            };
        }

        let vmctx_label = get_vmctx_value_label();

        // Some locals are present, preparing and divided ranges based on the scope
        // and frame_info data.
        let mut ranges_builder = ValueLabelRangesBuilder::new(scope, addr_tr, frame_info, isa);
        for p in self.parts.iter() {
            match p {
                CompiledExpressionPart::Code(_)
                | CompiledExpressionPart::Jump { .. }
                | CompiledExpressionPart::LandingPad { .. } => (),
                CompiledExpressionPart::Local { label, .. } => ranges_builder.process_label(*label),
                CompiledExpressionPart::Deref => ranges_builder.process_label(vmctx_label),
            }
        }
        if self.need_deref {
            ranges_builder.process_label(vmctx_label);
        }

        let ranges = ranges_builder.into_ranges();
        let expressions = BuildWithLocalsResult::Ranges(Box::new(
            ranges
                .ranges
                .map(
                    move |CachedValueLabelRange {
                              func_index,
                              start,
                              end,
                              label_location,
                          }| {
                        // build expression
                        let mut code_buf = Vec::new();
                        let mut jump_positions = Vec::new();
                        let mut landing_positions = HashMap::new();

                        macro_rules! deref {
                            () => {
                                if let (Some(vmctx_loc), Some(frame_info)) =
                                    (label_location.get(&vmctx_label), frame_info)
                                {
                                    if !append_memory_deref(
                                        &mut code_buf,
                                        frame_info,
                                        *vmctx_loc,
                                        isa,
                                    )? {
                                        return Ok(None);
                                    }
                                } else {
                                    return Ok(None);
                                }
                            };
                        }
                        for part in &self.parts {
                            match part {
                                CompiledExpressionPart::Code(c) => {
                                    code_buf.extend_from_slice(c.as_slice())
                                }
                                CompiledExpressionPart::LandingPad(marker) => {
                                    landing_positions.insert(marker.clone(), code_buf.len());
                                }
                                CompiledExpressionPart::Jump {
                                    conditionally,
                                    target,
                                } => {
                                    code_buf.push(
                                        match conditionally {
                                            true => gimli::constants::DW_OP_bra,
                                            false => gimli::constants::DW_OP_skip,
                                        }
                                        .0,
                                    );
                                    code_buf.push(!0);
                                    code_buf.push(!0); // these will be relocated below
                                    jump_positions.push((target.clone(), code_buf.len()));
                                }
                                CompiledExpressionPart::Local { label, trailing } => {
                                    let loc =
                                        *label_location.get(&label).context("label_location")?;
                                    if let Some(expr) = translate_loc(loc, isa, *trailing)? {
                                        code_buf.extend_from_slice(&expr)
                                    } else {
                                        return Ok(None);
                                    }
                                }
                                CompiledExpressionPart::Deref => deref!(),
                            }
                        }
                        if self.need_deref {
                            deref!();
                        }

                        for (marker, new_from) in jump_positions {
                            // relocate jump targets
                            let new_to = landing_positions[&marker];
                            let new_diff = new_to as isize - new_from as isize;
                            // FIXME: use encoding? LittleEndian for now...
                            code_buf[new_from - 2..new_from]
                                .copy_from_slice(&(new_diff as i16).to_le_bytes());
                        }
                        Ok(Some((func_index, start, end, code_buf)))
                    },
                )
                .filter_map(Result::transpose),
        ));

        BuiltCompiledExpression {
            expressions,
            covers_entire_scope: ranges.covers_entire_scope,
        }
    }
}

fn is_old_expression_format(buf: &[u8]) -> bool {
    // Heuristic to detect old variable expression format without DW_OP_fbreg:
    // DW_OP_plus_uconst op must be present, but not DW_OP_fbreg.
    if buf.contains(&(gimli::constants::DW_OP_fbreg.0)) {
        // Stop check if DW_OP_fbreg exist.
        return false;
    }
    buf.contains(&(gimli::constants::DW_OP_plus_uconst.0))
}

pub fn compile_expression<R>(
    expr: &Expression<R>,
    encoding: gimli::Encoding,
    frame_base: Option<&CompiledExpression>,
) -> Result<Option<CompiledExpression>, Error>
where
    R: Reader,
{
    // Bail when `frame_base` is complicated.
    if let Some(expr) = frame_base {
        if expr.parts.iter().any(|p| match p {
            CompiledExpressionPart::Jump { .. } => true,
            _ => false,
        }) {
            return Ok(None);
        }
    }

    // jump_targets key is offset in buf starting from the end
    // (see also `unread_bytes` below)
    let mut jump_targets: HashMap<u64, JumpTargetMarker> = HashMap::new();
    let mut pc = expr.0.clone();

    let buf = expr.0.to_slice()?;
    let mut parts = Vec::new();
    macro_rules! push {
        ($part:expr) => {{
            let part = $part;
            if let (CompiledExpressionPart::Code(cc2), Some(CompiledExpressionPart::Code(cc1))) =
                (&part, parts.last_mut())
            {
                cc1.extend_from_slice(cc2);
            } else {
                parts.push(part)
            }
        }};
    }
    let mut need_deref = false;
    if is_old_expression_format(&buf) && frame_base.is_some() {
        // Still supporting old DWARF variable expressions without fbreg.
        parts.extend_from_slice(&frame_base.unwrap().parts);
        if let Some(CompiledExpressionPart::Local { trailing, .. }) = parts.last_mut() {
            *trailing = false;
        }
        need_deref = frame_base.unwrap().need_deref;
    }
    let mut code_chunk = Vec::new();
    macro_rules! flush_code_chunk {
        () => {
            if !code_chunk.is_empty() {
                push!(CompiledExpressionPart::Code(code_chunk));
                code_chunk = Vec::new();
                let _ = code_chunk; // suppresses warning for final flush
            }
        };
    }

    // Find all landing pads by scanning bytes, do not care about
    // false location at this moment.
    // Looks hacky but it is fast; does not need to be really exact.
    if buf.len() > 2 {
        for i in 0..buf.len() - 2 {
            let op = buf[i];
            if op == gimli::constants::DW_OP_bra.0 || op == gimli::constants::DW_OP_skip.0 {
                // TODO fix for big-endian
                let offset = i16::from_le_bytes([buf[i + 1], buf[i + 2]]);
                let origin = i + 3;
                // Discarding out-of-bounds jumps (also some of falsely detected ops)
                if (offset >= 0 && offset as usize + origin <= buf.len())
                    || (offset < 0 && -offset as usize <= origin)
                {
                    let target = buf.len() as isize - origin as isize - offset as isize;
                    jump_targets.insert(target as u64, JumpTargetMarker::new());
                }
            }
        }
    }

    while !pc.is_empty() {
        let unread_bytes = pc.len().into_u64();
        if let Some(marker) = jump_targets.get(&unread_bytes) {
            flush_code_chunk!();
            parts.push(CompiledExpressionPart::LandingPad(marker.clone()));
        }

        need_deref = true;

        let pos = pc.offset_from(&expr.0).into_u64() as usize;
        let op = Operation::parse(&mut pc, encoding)?;
        match op {
            Operation::FrameOffset { offset } => {
                // Expand DW_OP_fbreg into frame location and DW_OP_plus_uconst.
                if frame_base.is_some() {
                    // Add frame base expressions.
                    flush_code_chunk!();
                    parts.extend_from_slice(&frame_base.unwrap().parts);
                }
                if let Some(CompiledExpressionPart::Local { trailing, .. }) = parts.last_mut() {
                    // Reset local trailing flag.
                    *trailing = false;
                }
                // Append DW_OP_plus_uconst part.
                let mut writer = ExpressionWriter::new();
                writer.write_op(gimli::constants::DW_OP_plus_uconst)?;
                writer.write_uleb128(offset as u64)?;
                code_chunk.extend(writer.into_vec());
                continue;
            }
            Operation::Drop { .. }
            | Operation::Pick { .. }
            | Operation::Swap { .. }
            | Operation::Rot { .. }
            | Operation::Nop { .. }
            | Operation::UnsignedConstant { .. }
            | Operation::SignedConstant { .. }
            | Operation::ConstantIndex { .. }
            | Operation::PlusConstant { .. }
            | Operation::Abs { .. }
            | Operation::And { .. }
            | Operation::Or { .. }
            | Operation::Xor { .. }
            | Operation::Shl { .. }
            | Operation::Plus { .. }
            | Operation::Minus { .. }
            | Operation::Div { .. }
            | Operation::Mod { .. }
            | Operation::Mul { .. }
            | Operation::Neg { .. }
            | Operation::Not { .. }
            | Operation::Lt { .. }
            | Operation::Gt { .. }
            | Operation::Le { .. }
            | Operation::Ge { .. }
            | Operation::Eq { .. }
            | Operation::Ne { .. }
            | Operation::TypedLiteral { .. }
            | Operation::Convert { .. }
            | Operation::Reinterpret { .. }
            | Operation::Piece { .. } => (),
            Operation::Bra { target } | Operation::Skip { target } => {
                flush_code_chunk!();
                let arc_to = (pc.len().into_u64() as isize - target as isize) as u64;
                let marker = match jump_targets.get(&arc_to) {
                    Some(m) => m.clone(),
                    None => {
                        // Marker not found: probably out of bounds.
                        return Ok(None);
                    }
                };
                push!(CompiledExpressionPart::Jump {
                    conditionally: match op {
                        Operation::Bra { .. } => true,
                        _ => false,
                    },
                    target: marker,
                });
                continue;
            }
            Operation::StackValue => {
                need_deref = false;

                // Find extra stack_value, that follow wasm-local operators,
                // and mark such locals with special flag.
                if let (Some(CompiledExpressionPart::Local { trailing, .. }), true) =
                    (parts.last_mut(), code_chunk.is_empty())
                {
                    *trailing = true;
                    continue;
                }
            }
            Operation::Deref { .. } => {
                flush_code_chunk!();
                push!(CompiledExpressionPart::Deref);
                // Don't re-enter the loop here (i.e. continue), because the
                // DW_OP_deref still needs to be kept.
            }
            Operation::WasmLocal { index } => {
                flush_code_chunk!();
                let label = ValueLabel::from_u32(index);
                push!(CompiledExpressionPart::Local {
                    label,
                    trailing: false,
                });
                continue;
            }
            Operation::Shr { .. } | Operation::Shra { .. } => {
                // Insert value normalisation part.
                // The semantic value is 32 bits (TODO: check unit)
                // but the target architecture is 64-bits. So we'll
                // clean out the upper 32 bits (in a sign-correct way)
                // to avoid contamination of the result with randomness.
                let mut writer = ExpressionWriter::new();
                writer.write_op(gimli::constants::DW_OP_plus_uconst)?;
                writer.write_uleb128(32)?; // increase shift amount
                writer.write_op(gimli::constants::DW_OP_swap)?;
                writer.write_op(gimli::constants::DW_OP_const1u)?;
                writer.write_u8(32)?;
                writer.write_op(gimli::constants::DW_OP_shl)?;
                writer.write_op(gimli::constants::DW_OP_swap)?;
                code_chunk.extend(writer.into_vec());
                // Don't re-enter the loop here (i.e. continue), because the
                // DW_OP_shr* still needs to be kept.
            }
            Operation::Address { .. }
            | Operation::AddressIndex { .. }
            | Operation::Call { .. }
            | Operation::Register { .. }
            | Operation::RegisterOffset { .. }
            | Operation::CallFrameCFA
            | Operation::PushObjectAddress
            | Operation::TLS
            | Operation::ImplicitValue { .. }
            | Operation::ImplicitPointer { .. }
            | Operation::EntryValue { .. }
            | Operation::ParameterRef { .. } => {
                return Ok(None);
            }
            Operation::WasmGlobal { index: _ } | Operation::WasmStack { index: _ } => {
                // TODO support those two
                return Ok(None);
            }
        }
        let chunk = &buf[pos..pc.offset_from(&expr.0).into_u64() as usize];
        code_chunk.extend_from_slice(chunk);
    }

    flush_code_chunk!();
    if let Some(marker) = jump_targets.get(&0) {
        parts.push(CompiledExpressionPart::LandingPad(marker.clone()));
    }

    Ok(Some(CompiledExpression { parts, need_deref }))
}

#[derive(Debug, Clone)]
struct CachedValueLabelRange {
    func_index: usize,
    start: usize,
    end: usize,
    label_location: HashMap<ValueLabel, LabelValueLoc>,
}

struct BuiltRangeSummary<'a> {
    range: &'a CachedValueLabelRange,
    isa: &'a dyn TargetIsa,
}

impl<'a> fmt::Debug for BuiltRangeSummary<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let range = self.range;
        write!(f, "[")?;
        let mut is_first = true;
        for (value, value_loc) in &range.label_location {
            if !is_first {
                write!(f, ", ")?;
            } else {
                is_first = false;
            }
            write!(
                f,
                "{:?}:{:?}",
                log_get_value_name(*value),
                log_get_value_loc(*value_loc, self.isa)
            )?;
        }
        write!(f, "]@[{}..{})", range.start, range.end)?;
        Ok(())
    }
}

struct ValueLabelRangesBuilder<'a, 'b> {
    isa: &'a dyn TargetIsa,
    ranges: Vec<CachedValueLabelRange>,
    frame_info: Option<&'a FunctionFrameInfo<'b>>,
    processed_labels: HashSet<ValueLabel>,
    covers_entire_scope: bool,
}

struct BuiltValueLabelRanges<TIter> {
    ranges: TIter,
    covers_entire_scope: bool,
}

impl<'a, 'b> ValueLabelRangesBuilder<'a, 'b> {
    pub fn new(
        scope: &[(u64, u64)], // wasm ranges
        addr_tr: &'a AddressTransform,
        frame_info: Option<&'a FunctionFrameInfo<'b>>,
        isa: &'a dyn TargetIsa,
    ) -> Self {
        let mut ranges = Vec::new();
        for (wasm_start, wasm_end) in scope {
            if let Some((func_index, tr)) = addr_tr.translate_ranges_raw(*wasm_start, *wasm_end) {
                ranges.extend(tr.into_iter().map(|(start, end)| CachedValueLabelRange {
                    func_index,
                    start,
                    end,
                    label_location: HashMap::new(),
                }));
            }
        }
        ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));

        dbi_log!(
            "Building ranges for values in scope: {}\n{:?}",
            ranges
                .iter()
                .map(|r| format!("[{}..{})", r.start, r.end))
                .join(" "),
            log_get_value_ranges(frame_info.map(|f| f.value_ranges), isa)
        );
        ValueLabelRangesBuilder {
            isa,
            ranges,
            frame_info,
            processed_labels: HashSet::new(),
            covers_entire_scope: true,
        }
    }

    fn process_label(&mut self, label: ValueLabel) {
        if self.processed_labels.contains(&label) {
            return;
        }
        dbi_log!("Intersecting with {:?}", log_get_value_name(label));
        self.processed_labels.insert(label);

        let value_ranges = match self.frame_info.and_then(|fi| fi.value_ranges.get(&label)) {
            Some(value_ranges) => value_ranges,
            None => {
                return;
            }
        };

        let ranges = &mut self.ranges;
        for value_range in value_ranges {
            let range_start = value_range.start as usize;
            let range_end = value_range.end as usize;
            let loc = value_range.loc;
            if range_start == range_end {
                continue;
            }
            assert!(range_start < range_end);

            // Find acceptable scope of ranges to intersect with.
            let i = match ranges.binary_search_by(|s| s.start.cmp(&range_start)) {
                Ok(i) => i,
                Err(i) => {
                    if i > 0 && range_start < ranges[i - 1].end {
                        i - 1
                    } else {
                        i
                    }
                }
            };
            let j = match ranges.binary_search_by(|s| s.start.cmp(&range_end)) {
                Ok(i) | Err(i) => i,
            };
            // Starting from the end, intersect (range_start..range_end) with
            // self.ranges array.
            for i in (i..j).rev() {
                if range_end <= ranges[i].start || ranges[i].end <= range_start {
                    continue;
                }
                if range_end < ranges[i].end {
                    // Cutting some of the range from the end.
                    let mut tail = ranges[i].clone();
                    ranges[i].end = range_end;
                    tail.start = range_end;
                    ranges.insert(i + 1, tail);
                    self.covers_entire_scope = false;
                }
                assert!(ranges[i].end <= range_end);
                if range_start <= ranges[i].start {
                    ranges[i].label_location.insert(label, loc);
                    continue;
                }
                // Cutting some of the range from the start.
                let mut tail = ranges[i].clone();
                ranges[i].end = range_start;
                tail.start = range_start;
                tail.label_location.insert(label, loc);
                ranges.insert(i + 1, tail);
                self.covers_entire_scope = false;
            }
        }
    }

    pub fn into_ranges(
        self,
    ) -> BuiltValueLabelRanges<impl Iterator<Item = CachedValueLabelRange> + use<>> {
        // Ranges with not-enough labels are discarded.
        let processed_labels_len = self.processed_labels.len();
        let is_valid_range =
            move |r: &CachedValueLabelRange| r.label_location.len() == processed_labels_len;

        if dbi_log_enabled!() {
            dbi_log!("Built ranges:");
            for range in self.ranges.iter().filter(|r| is_valid_range(*r)) {
                dbi_log!(
                    "{:?}",
                    BuiltRangeSummary {
                        range,
                        isa: self.isa
                    }
                );
            }
            dbi_log!("");
        }

        BuiltValueLabelRanges {
            ranges: self.ranges.into_iter().filter(is_valid_range),
            covers_entire_scope: self.covers_entire_scope,
        }
    }
}

/// Marker for tracking incoming jumps.
/// Different when created new, and the same when cloned.
#[derive(Clone, Eq)]
struct JumpTargetMarker(Rc<u32>);

impl JumpTargetMarker {
    fn new() -> JumpTargetMarker {
        // Create somewhat unique hash data -- using part of
        // the pointer of the RcBox.
        let mut rc = Rc::new(0);
        let hash_data = rc.as_ref() as *const u32 as usize as u32;
        *Rc::get_mut(&mut rc).unwrap() = hash_data;
        JumpTargetMarker(rc)
    }
}

impl PartialEq for JumpTargetMarker {
    fn eq(&self, other: &JumpTargetMarker) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Hash for JumpTargetMarker {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u32(*self.0);
    }
}
impl std::fmt::Debug for JumpTargetMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "JumpMarker<{:08x}>",
            self.0.as_ref() as *const u32 as usize
        )
    }
}

#[cfg(test)]
#[expect(trivial_numeric_casts, reason = "macro-generated code")]
mod tests {
    use super::{
        AddressTransform, CompiledExpression, CompiledExpressionPart, FunctionFrameInfo,
        JumpTargetMarker, ValueLabel, ValueLabelsRanges, compile_expression,
    };
    use crate::CompiledFunctionMetadata;
    use cranelift_codegen::{isa::lookup, settings::Flags};
    use gimli::{Encoding, EndianSlice, Expression, RunTimeEndian, constants};
    use target_lexicon::triple;
    use wasmtime_environ::FilePos;

    macro_rules! dw_op {
        (DW_OP_WASM_location) => {
            0xed
        };
        ($i:literal) => {
            $i
        };
        ($d:ident) => {
            constants::$d.0 as u8
        };
        ($e:expr) => {
            $e as u8
        };
    }

    macro_rules! expression {
        ($($t:tt),*) => {
            Expression(EndianSlice::new(
                &[$(dw_op!($t)),*],
                RunTimeEndian::Little,
            ))
        }
    }

    fn find_jump_targets<'a>(ce: &'a CompiledExpression) -> Vec<&'a JumpTargetMarker> {
        ce.parts
            .iter()
            .filter_map(|p| {
                if let CompiledExpressionPart::LandingPad(t) = p {
                    Some(t)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    static DWARF_ENCODING: Encoding = Encoding {
        address_size: 4,
        format: gimli::Format::Dwarf32,
        version: 4,
    };

    #[test]
    fn test_debug_expression_jump_target() {
        let m1 = JumpTargetMarker::new();
        let m2 = JumpTargetMarker::new();
        assert!(m1 != m2);
        assert!(m1 == m1.clone());

        // Internal hash_data test (theoretically can fail intermittently).
        assert!(m1.0 != m2.0);
    }

    #[test]
    fn test_debug_parse_expressions() {
        use cranelift_entity::EntityRef;

        let (val1, val3, val20) = (ValueLabel::new(1), ValueLabel::new(3), ValueLabel::new(20));

        let e = expression!(DW_OP_WASM_location, 0x0, 20, DW_OP_stack_value);
        let ce = compile_expression(&e, DWARF_ENCODING, None)
            .expect("non-error")
            .expect("expression");
        assert_eq!(
            ce,
            CompiledExpression {
                parts: vec![CompiledExpressionPart::Local {
                    label: val20,
                    trailing: true
                }],
                need_deref: false,
            }
        );

        let e = expression!(
            DW_OP_WASM_location,
            0x0,
            1,
            DW_OP_plus_uconst,
            0x10,
            DW_OP_stack_value
        );
        let ce = compile_expression(&e, DWARF_ENCODING, None)
            .expect("non-error")
            .expect("expression");
        assert_eq!(
            ce,
            CompiledExpression {
                parts: vec![
                    CompiledExpressionPart::Local {
                        label: val1,
                        trailing: false
                    },
                    CompiledExpressionPart::Code(vec![35, 16, 159])
                ],
                need_deref: false,
            }
        );

        let e = expression!(DW_OP_WASM_location, 0x0, 3, DW_OP_stack_value);
        let fe = compile_expression(&e, DWARF_ENCODING, None).expect("non-error");
        let e = expression!(DW_OP_fbreg, 0x12);
        let ce = compile_expression(&e, DWARF_ENCODING, fe.as_ref())
            .expect("non-error")
            .expect("expression");
        assert_eq!(
            ce,
            CompiledExpression {
                parts: vec![
                    CompiledExpressionPart::Local {
                        label: val3,
                        trailing: false
                    },
                    CompiledExpressionPart::Code(vec![35, 18])
                ],
                need_deref: true,
            }
        );

        let e = expression!(
            DW_OP_WASM_location,
            0x0,
            1,
            DW_OP_plus_uconst,
            5,
            DW_OP_deref,
            DW_OP_stack_value
        );
        let ce = compile_expression(&e, DWARF_ENCODING, None)
            .expect("non-error")
            .expect("expression");
        assert_eq!(
            ce,
            CompiledExpression {
                parts: vec![
                    CompiledExpressionPart::Local {
                        label: val1,
                        trailing: false
                    },
                    CompiledExpressionPart::Code(vec![35, 5]),
                    CompiledExpressionPart::Deref,
                    CompiledExpressionPart::Code(vec![6, 159])
                ],
                need_deref: false,
            }
        );

        let e = expression!(
            DW_OP_WASM_location,
            0x0,
            1,
            DW_OP_lit16,
            DW_OP_shra,
            DW_OP_stack_value
        );
        let ce = compile_expression(&e, DWARF_ENCODING, None)
            .expect("non-error")
            .expect("expression");
        assert_eq!(
            ce,
            CompiledExpression {
                parts: vec![
                    CompiledExpressionPart::Local {
                        label: val1,
                        trailing: false
                    },
                    CompiledExpressionPart::Code(vec![64, 35, 32, 22, 8, 32, 36, 22, 38, 159])
                ],
                need_deref: false,
            }
        );

        let e = expression!(
            DW_OP_lit1,
            DW_OP_dup,
            DW_OP_WASM_location,
            0x0,
            1,
            DW_OP_and,
            DW_OP_bra,
            5,
            0, // --> pointer
            DW_OP_swap,
            DW_OP_shr,
            DW_OP_skip,
            2,
            0, // --> done
            // pointer:
            DW_OP_plus,
            DW_OP_deref,
            // done:
            DW_OP_stack_value
        );
        let ce = compile_expression(&e, DWARF_ENCODING, None)
            .expect("non-error")
            .expect("expression");
        let targets = find_jump_targets(&ce);
        assert_eq!(targets.len(), 2);
        assert_eq!(
            ce,
            CompiledExpression {
                parts: vec![
                    CompiledExpressionPart::Code(vec![49, 18]),
                    CompiledExpressionPart::Local {
                        label: val1,
                        trailing: false
                    },
                    CompiledExpressionPart::Code(vec![26]),
                    CompiledExpressionPart::Jump {
                        conditionally: true,
                        target: targets[0].clone(),
                    },
                    CompiledExpressionPart::Code(vec![22, 35, 32, 22, 8, 32, 36, 22, 37]),
                    CompiledExpressionPart::Jump {
                        conditionally: false,
                        target: targets[1].clone(),
                    },
                    CompiledExpressionPart::LandingPad(targets[0].clone()), // capture from
                    CompiledExpressionPart::Code(vec![34]),
                    CompiledExpressionPart::Deref,
                    CompiledExpressionPart::Code(vec![6]),
                    CompiledExpressionPart::LandingPad(targets[1].clone()), // capture to
                    CompiledExpressionPart::Code(vec![159])
                ],
                need_deref: false,
            }
        );

        let e = expression!(
            DW_OP_lit1,
            DW_OP_dup,
            DW_OP_bra,
            2,
            0, // --> target
            DW_OP_deref,
            DW_OP_lit0,
            // target:
            DW_OP_stack_value
        );
        let ce = compile_expression(&e, DWARF_ENCODING, None)
            .expect("non-error")
            .expect("expression");
        let targets = find_jump_targets(&ce);
        assert_eq!(targets.len(), 1);
        assert_eq!(
            ce,
            CompiledExpression {
                parts: vec![
                    CompiledExpressionPart::Code(vec![49, 18]),
                    CompiledExpressionPart::Jump {
                        conditionally: true,
                        target: targets[0].clone(),
                    },
                    CompiledExpressionPart::Deref,
                    CompiledExpressionPart::Code(vec![6, 48]),
                    CompiledExpressionPart::LandingPad(targets[0].clone()), // capture to
                    CompiledExpressionPart::Code(vec![159])
                ],
                need_deref: false,
            }
        );

        let e = expression!(
            DW_OP_lit1,
            /* loop */ DW_OP_dup,
            DW_OP_lit25,
            DW_OP_ge,
            DW_OP_bra,
            5,
            0, // --> done
            DW_OP_plus_uconst,
            1,
            DW_OP_skip,
            (-11_i8),
            (!0), // --> loop
            /* done */ DW_OP_stack_value
        );
        let ce = compile_expression(&e, DWARF_ENCODING, None)
            .expect("non-error")
            .expect("expression");
        let targets = find_jump_targets(&ce);
        assert_eq!(targets.len(), 2);
        assert_eq!(
            ce,
            CompiledExpression {
                parts: vec![
                    CompiledExpressionPart::Code(vec![49]),
                    CompiledExpressionPart::LandingPad(targets[0].clone()),
                    CompiledExpressionPart::Code(vec![18, 73, 42]),
                    CompiledExpressionPart::Jump {
                        conditionally: true,
                        target: targets[1].clone(),
                    },
                    CompiledExpressionPart::Code(vec![35, 1]),
                    CompiledExpressionPart::Jump {
                        conditionally: false,
                        target: targets[0].clone(),
                    },
                    CompiledExpressionPart::LandingPad(targets[1].clone()),
                    CompiledExpressionPart::Code(vec![159])
                ],
                need_deref: false,
            }
        );

        let e = expression!(DW_OP_WASM_location, 0x0, 1, DW_OP_plus_uconst, 5);
        let ce = compile_expression(&e, DWARF_ENCODING, None)
            .expect("non-error")
            .expect("expression");
        assert_eq!(
            ce,
            CompiledExpression {
                parts: vec![
                    CompiledExpressionPart::Local {
                        label: val1,
                        trailing: false
                    },
                    CompiledExpressionPart::Code(vec![35, 5])
                ],
                need_deref: true,
            }
        );
    }

    fn create_mock_address_transform() -> AddressTransform {
        use crate::FunctionAddressMap;
        use cranelift_entity::PrimaryMap;
        use wasmtime_environ::InstructionAddressMap;
        use wasmtime_environ::WasmFileInfo;

        let mut module_map = PrimaryMap::new();
        let code_section_offset: u32 = 100;
        let func = CompiledFunctionMetadata {
            address_map: FunctionAddressMap {
                instructions: vec![
                    InstructionAddressMap {
                        srcloc: FilePos::new(code_section_offset + 12),
                        code_offset: 5,
                    },
                    InstructionAddressMap {
                        srcloc: FilePos::default(),
                        code_offset: 8,
                    },
                    InstructionAddressMap {
                        srcloc: FilePos::new(code_section_offset + 17),
                        code_offset: 15,
                    },
                    InstructionAddressMap {
                        srcloc: FilePos::default(),
                        code_offset: 23,
                    },
                ]
                .into(),
                start_srcloc: FilePos::new(code_section_offset + 10),
                end_srcloc: FilePos::new(code_section_offset + 20),
                body_offset: 0,
                body_len: 30,
            },
            ..Default::default()
        };
        module_map.push(&func);
        let fi = WasmFileInfo {
            code_section_offset: code_section_offset.into(),
            funcs: Vec::new(),
            imported_func_count: 0,
            path: None,
        };
        AddressTransform::mock(&module_map, fi)
    }

    fn create_mock_value_ranges() -> (ValueLabelsRanges, (ValueLabel, ValueLabel, ValueLabel)) {
        use cranelift_codegen::{LabelValueLoc, ValueLocRange};
        use cranelift_entity::EntityRef;
        use std::collections::HashMap;
        let mut value_ranges = HashMap::new();
        let value_0 = ValueLabel::new(0);
        let value_1 = ValueLabel::new(1);
        let value_2 = ValueLabel::new(2);
        value_ranges.insert(
            value_0,
            vec![ValueLocRange {
                loc: LabelValueLoc::CFAOffset(0),
                start: 0,
                end: 25,
            }],
        );
        value_ranges.insert(
            value_1,
            vec![ValueLocRange {
                loc: LabelValueLoc::CFAOffset(0),
                start: 5,
                end: 30,
            }],
        );
        value_ranges.insert(
            value_2,
            vec![
                ValueLocRange {
                    loc: LabelValueLoc::CFAOffset(0),
                    start: 0,
                    end: 10,
                },
                ValueLocRange {
                    loc: LabelValueLoc::CFAOffset(0),
                    start: 20,
                    end: 30,
                },
            ],
        );
        (value_ranges, (value_0, value_1, value_2))
    }

    #[test]
    fn test_debug_value_range_builder() {
        use super::ValueLabelRangesBuilder;
        use crate::debug::ModuleMemoryOffset;

        // Ignore this test if cranelift doesn't support the native platform.
        if cranelift_native::builder().is_err() {
            return;
        }

        let isa = lookup(triple!("x86_64"))
            .expect("expect x86_64 ISA")
            .finish(Flags::new(cranelift_codegen::settings::builder()))
            .expect("Creating ISA");

        let addr_tr = create_mock_address_transform();
        let (value_ranges, value_labels) = create_mock_value_ranges();
        let fi = FunctionFrameInfo {
            memory_offset: ModuleMemoryOffset::None,
            value_ranges: &value_ranges,
        };

        // No value labels, testing if entire function range coming through.
        let builder = ValueLabelRangesBuilder::new(&[(10, 20)], &addr_tr, Some(&fi), isa.as_ref());
        let ranges = builder.into_ranges().ranges.collect::<Vec<_>>();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].func_index, 0);
        assert_eq!(ranges[0].start, 0);
        assert_eq!(ranges[0].end, 30);

        // Two labels (val0@0..25 and val1@5..30), their common lifetime intersect at 5..25.
        let mut builder =
            ValueLabelRangesBuilder::new(&[(10, 20)], &addr_tr, Some(&fi), isa.as_ref());
        builder.process_label(value_labels.0);
        builder.process_label(value_labels.1);
        let ranges = builder.into_ranges().ranges.collect::<Vec<_>>();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].start, 5);
        assert_eq!(ranges[0].end, 25);

        // Adds val2 with complex lifetime @0..10 and @20..30 to the previous test, and
        // also narrows range.
        let mut builder =
            ValueLabelRangesBuilder::new(&[(11, 17)], &addr_tr, Some(&fi), isa.as_ref());
        builder.process_label(value_labels.0);
        builder.process_label(value_labels.1);
        builder.process_label(value_labels.2);
        let ranges = builder.into_ranges().ranges.collect::<Vec<_>>();
        // Result is two ranges @5..10 and @20..23
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].start, 5);
        assert_eq!(ranges[0].end, 10);
        assert_eq!(ranges[1].start, 20);
        assert_eq!(ranges[1].end, 23);
    }
}
