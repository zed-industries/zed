use super::HashMap;
use crate::frontend::FunctionBuilder;
use alloc::vec::Vec;
use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::*;

type EntryIndex = u128;

/// Unlike with `br_table`, `Switch` cases may be sparse or non-0-based.
/// They emit efficient code using branches, jump tables, or a combination of both.
///
/// # Example
///
/// ```rust
/// # use cranelift_codegen::ir::types::*;
/// # use cranelift_codegen::ir::{UserFuncName, Function, Signature, InstBuilder};
/// # use cranelift_codegen::isa::CallConv;
/// # use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Switch};
/// #
/// # let mut sig = Signature::new(CallConv::SystemV);
/// # let mut fn_builder_ctx = FunctionBuilderContext::new();
/// # let mut func = Function::with_name_signature(UserFuncName::user(0, 0), sig);
/// # let mut builder = FunctionBuilder::new(&mut func, &mut fn_builder_ctx);
/// #
/// # let entry = builder.create_block();
/// # builder.switch_to_block(entry);
/// #
/// let block0 = builder.create_block();
/// let block1 = builder.create_block();
/// let block2 = builder.create_block();
/// let fallback = builder.create_block();
///
/// let val = builder.ins().iconst(I32, 1);
///
/// let mut switch = Switch::new();
/// switch.set_entry(0, block0);
/// switch.set_entry(1, block1);
/// switch.set_entry(7, block2);
/// switch.emit(&mut builder, val, fallback);
/// ```
#[derive(Debug, Default)]
pub struct Switch {
    cases: HashMap<EntryIndex, Block>,
}

impl Switch {
    /// Create a new empty switch
    pub fn new() -> Self {
        Self {
            cases: HashMap::new(),
        }
    }

    /// Set a switch entry
    pub fn set_entry(&mut self, index: EntryIndex, block: Block) {
        let prev = self.cases.insert(index, block);
        assert!(prev.is_none(), "Tried to set the same entry {index} twice");
    }

    /// Get a reference to all existing entries
    pub fn entries(&self) -> &HashMap<EntryIndex, Block> {
        &self.cases
    }

    /// Turn the `cases` `HashMap` into a list of `ContiguousCaseRange`s.
    ///
    /// # Postconditions
    ///
    /// * Every entry will be represented.
    /// * The `ContiguousCaseRange`s will not overlap.
    /// * Between two `ContiguousCaseRange`s there will be at least one entry index.
    /// * No `ContiguousCaseRange`s will be empty.
    fn collect_contiguous_case_ranges(self) -> Vec<ContiguousCaseRange> {
        log::trace!("build_contiguous_case_ranges before: {:#?}", self.cases);
        let mut cases = self.cases.into_iter().collect::<Vec<(_, _)>>();
        cases.sort_by_key(|&(index, _)| index);

        let mut contiguous_case_ranges: Vec<ContiguousCaseRange> = vec![];
        let mut last_index = None;
        for (index, block) in cases {
            match last_index {
                None => contiguous_case_ranges.push(ContiguousCaseRange::new(index)),
                Some(last_index) => {
                    if index > last_index + 1 {
                        contiguous_case_ranges.push(ContiguousCaseRange::new(index));
                    }
                }
            }
            contiguous_case_ranges
                .last_mut()
                .unwrap()
                .blocks
                .push(block);
            last_index = Some(index);
        }

        log::trace!("build_contiguous_case_ranges after: {contiguous_case_ranges:#?}");

        contiguous_case_ranges
    }

    /// Binary search for the right `ContiguousCaseRange`.
    fn build_search_tree<'a>(
        bx: &mut FunctionBuilder,
        val: Value,
        otherwise: Block,
        contiguous_case_ranges: &'a [ContiguousCaseRange],
    ) {
        // If no switch cases were added to begin with, we can just emit `jump otherwise`.
        if contiguous_case_ranges.is_empty() {
            bx.ins().jump(otherwise, &[]);
            return;
        }

        // Avoid allocation in the common case
        if contiguous_case_ranges.len() <= 3 {
            Self::build_search_branches(bx, val, otherwise, contiguous_case_ranges);
            return;
        }

        let mut stack = Vec::new();
        stack.push((None, contiguous_case_ranges));

        while let Some((block, contiguous_case_ranges)) = stack.pop() {
            if let Some(block) = block {
                bx.switch_to_block(block);
            }

            if contiguous_case_ranges.len() <= 3 {
                Self::build_search_branches(bx, val, otherwise, contiguous_case_ranges);
            } else {
                let split_point = contiguous_case_ranges.len() / 2;
                let (left, right) = contiguous_case_ranges.split_at(split_point);

                let left_block = bx.create_block();
                let right_block = bx.create_block();

                let first_index = right[0].first_index;
                let should_take_right_side =
                    icmp_imm_u128(bx, IntCC::UnsignedGreaterThanOrEqual, val, first_index);
                bx.ins()
                    .brif(should_take_right_side, right_block, &[], left_block, &[]);

                bx.seal_block(left_block);
                bx.seal_block(right_block);

                stack.push((Some(left_block), left));
                stack.push((Some(right_block), right));
            }
        }
    }

    /// Linear search for the right `ContiguousCaseRange`.
    fn build_search_branches<'a>(
        bx: &mut FunctionBuilder,
        val: Value,
        otherwise: Block,
        contiguous_case_ranges: &'a [ContiguousCaseRange],
    ) {
        for (ix, range) in contiguous_case_ranges.iter().enumerate().rev() {
            let alternate = if ix == 0 {
                otherwise
            } else {
                bx.create_block()
            };

            if range.first_index == 0 {
                assert_eq!(alternate, otherwise);

                if let Some(block) = range.single_block() {
                    bx.ins().brif(val, otherwise, &[], block, &[]);
                } else {
                    Self::build_jump_table(bx, val, otherwise, 0, &range.blocks);
                }
            } else {
                if let Some(block) = range.single_block() {
                    let is_good_val = icmp_imm_u128(bx, IntCC::Equal, val, range.first_index);
                    bx.ins().brif(is_good_val, block, &[], alternate, &[]);
                } else {
                    let is_good_val = icmp_imm_u128(
                        bx,
                        IntCC::UnsignedGreaterThanOrEqual,
                        val,
                        range.first_index,
                    );
                    let jt_block = bx.create_block();
                    bx.ins().brif(is_good_val, jt_block, &[], alternate, &[]);
                    bx.seal_block(jt_block);
                    bx.switch_to_block(jt_block);
                    Self::build_jump_table(bx, val, otherwise, range.first_index, &range.blocks);
                }
            }

            if alternate != otherwise {
                bx.seal_block(alternate);
                bx.switch_to_block(alternate);
            }
        }
    }

    fn build_jump_table(
        bx: &mut FunctionBuilder,
        val: Value,
        otherwise: Block,
        first_index: EntryIndex,
        blocks: &[Block],
    ) {
        // There are currently no 128bit systems supported by rustc, but once we do ensure that
        // we don't silently ignore a part of the jump table for 128bit integers on 128bit systems.
        assert!(
            u32::try_from(blocks.len()).is_ok(),
            "Jump tables bigger than 2^32-1 are not yet supported"
        );

        let jt_data = JumpTableData::new(
            bx.func.dfg.block_call(otherwise, &[]),
            &blocks
                .iter()
                .map(|block| bx.func.dfg.block_call(*block, &[]))
                .collect::<Vec<_>>(),
        );
        let jump_table = bx.create_jump_table(jt_data);

        let discr = if first_index == 0 {
            val
        } else {
            if let Ok(first_index) = u64::try_from(first_index) {
                bx.ins().iadd_imm(val, (first_index as i64).wrapping_neg())
            } else {
                let (lsb, msb) = (first_index as u64, (first_index >> 64) as u64);
                let lsb = bx.ins().iconst(types::I64, lsb as i64);
                let msb = bx.ins().iconst(types::I64, msb as i64);
                let index = bx.ins().iconcat(lsb, msb);
                bx.ins().isub(val, index)
            }
        };

        let discr = match bx.func.dfg.value_type(discr).bits() {
            bits if bits > 32 => {
                // Check for overflow of cast to u32. This is the max supported jump table entries.
                let new_block = bx.create_block();
                let bigger_than_u32 =
                    bx.ins()
                        .icmp_imm(IntCC::UnsignedGreaterThan, discr, u32::MAX as i64);
                bx.ins()
                    .brif(bigger_than_u32, otherwise, &[], new_block, &[]);
                bx.seal_block(new_block);
                bx.switch_to_block(new_block);

                // Cast to i32, as br_table is not implemented for i64/i128
                bx.ins().ireduce(types::I32, discr)
            }
            bits if bits < 32 => bx.ins().uextend(types::I32, discr),
            _ => discr,
        };

        bx.ins().br_table(discr, jump_table);
    }

    /// Build the switch
    ///
    /// # Arguments
    ///
    /// * The function builder to emit to
    /// * The value to switch on
    /// * The default block
    pub fn emit(self, bx: &mut FunctionBuilder, val: Value, otherwise: Block) {
        // Validate that the type of `val` is sufficiently wide to address all cases.
        let max = self.cases.keys().max().copied().unwrap_or(0);
        let val_ty = bx.func.dfg.value_type(val);
        let val_ty_max = val_ty.bounds(false).1;
        if max > val_ty_max {
            panic!("The index type {val_ty} does not fit the maximum switch entry of {max}");
        }

        let contiguous_case_ranges = self.collect_contiguous_case_ranges();
        Self::build_search_tree(bx, val, otherwise, &contiguous_case_ranges);
    }
}

fn icmp_imm_u128(bx: &mut FunctionBuilder, cond: IntCC, x: Value, y: u128) -> Value {
    if bx.func.dfg.value_type(x) != types::I128 {
        assert!(u64::try_from(y).is_ok());
        bx.ins().icmp_imm(cond, x, y as i64)
    } else if let Ok(index) = i64::try_from(y) {
        bx.ins().icmp_imm(cond, x, index)
    } else {
        let (lsb, msb) = (y as u64, (y >> 64) as u64);
        let lsb = bx.ins().iconst(types::I64, lsb as i64);
        let msb = bx.ins().iconst(types::I64, msb as i64);
        let index = bx.ins().iconcat(lsb, msb);
        bx.ins().icmp(cond, x, index)
    }
}

/// This represents a contiguous range of cases to switch on.
///
/// For example 10 => block1, 11 => block2, 12 => block7 will be represented as:
///
/// ```plain
/// ContiguousCaseRange {
///     first_index: 10,
///     blocks: vec![Block::from_u32(1), Block::from_u32(2), Block::from_u32(7)]
/// }
/// ```
#[derive(Debug)]
struct ContiguousCaseRange {
    /// The entry index of the first case. Eg. 10 when the entry indexes are 10, 11, 12 and 13.
    first_index: EntryIndex,

    /// The blocks to jump to sorted in ascending order of entry index.
    blocks: Vec<Block>,
}

impl ContiguousCaseRange {
    fn new(first_index: EntryIndex) -> Self {
        Self {
            first_index,
            blocks: Vec::new(),
        }
    }

    /// Returns `Some` block when there is only a single block in this range.
    fn single_block(&self) -> Option<Block> {
        if self.blocks.len() == 1 {
            Some(self.blocks[0])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::FunctionBuilderContext;
    use alloc::string::ToString;

    macro_rules! setup {
        ($default:expr, [$($index:expr,)*]) => {{
            let mut func = Function::new();
            let mut func_ctx = FunctionBuilderContext::new();
            {
                let mut bx = FunctionBuilder::new(&mut func, &mut func_ctx);
                let block = bx.create_block();
                bx.switch_to_block(block);
                let val = bx.ins().iconst(types::I8, 0);
                let mut switch = Switch::new();
                let _ = &mut switch;
                $(
                    let block = bx.create_block();
                    switch.set_entry($index, block);
                )*
                switch.emit(&mut bx, val, Block::with_number($default).unwrap());
            }
            func
                .to_string()
                .trim_start_matches("function u0:0() fast {\n")
                .trim_end_matches("\n}\n")
                .to_string()
        }};
    }

    #[test]
    fn switch_empty() {
        let func = setup!(42, []);
        assert_eq_output!(
            func,
            "block0:
    v0 = iconst.i8 0
    jump block42"
        );
    }

    #[test]
    fn switch_zero() {
        let func = setup!(0, [0,]);
        assert_eq_output!(
            func,
            "block0:
    v0 = iconst.i8 0
    brif v0, block0, block1  ; v0 = 0"
        );
    }

    #[test]
    fn switch_single() {
        let func = setup!(0, [1,]);
        assert_eq_output!(
            func,
            "block0:
    v0 = iconst.i8 0
    v1 = icmp_imm eq v0, 1  ; v0 = 0
    brif v1, block1, block0"
        );
    }

    #[test]
    fn switch_bool() {
        let func = setup!(0, [0, 1,]);
        assert_eq_output!(
            func,
            "block0:
    v0 = iconst.i8 0
    v1 = uextend.i32 v0  ; v0 = 0
    br_table v1, block0, [block1, block2]"
        );
    }

    #[test]
    fn switch_two_gap() {
        let func = setup!(0, [0, 2,]);
        assert_eq_output!(
            func,
            "block0:
    v0 = iconst.i8 0
    v1 = icmp_imm eq v0, 2  ; v0 = 0
    brif v1, block2, block3

block3:
    brif.i8 v0, block0, block1  ; v0 = 0"
        );
    }

    #[test]
    fn switch_many() {
        let func = setup!(0, [0, 1, 5, 7, 10, 11, 12,]);
        assert_eq_output!(
            func,
            "block0:
    v0 = iconst.i8 0
    v1 = icmp_imm uge v0, 7  ; v0 = 0
    brif v1, block9, block8

block9:
    v2 = icmp_imm.i8 uge v0, 10  ; v0 = 0
    brif v2, block11, block10

block11:
    v3 = iadd_imm.i8 v0, -10  ; v0 = 0
    v4 = uextend.i32 v3
    br_table v4, block0, [block5, block6, block7]

block10:
    v5 = icmp_imm.i8 eq v0, 7  ; v0 = 0
    brif v5, block4, block0

block8:
    v6 = icmp_imm.i8 eq v0, 5  ; v0 = 0
    brif v6, block3, block12

block12:
    v7 = uextend.i32 v0  ; v0 = 0
    br_table v7, block0, [block1, block2]"
        );
    }

    #[test]
    fn switch_min_index_value() {
        let func = setup!(0, [i8::MIN as u8 as u128, 1,]);
        assert_eq_output!(
            func,
            "block0:
    v0 = iconst.i8 0
    v1 = icmp_imm eq v0, -128  ; v0 = 0
    brif v1, block1, block3

block3:
    v2 = icmp_imm.i8 eq v0, 1  ; v0 = 0
    brif v2, block2, block0"
        );
    }

    #[test]
    fn switch_max_index_value() {
        let func = setup!(0, [i8::MAX as u8 as u128, 1,]);
        assert_eq_output!(
            func,
            "block0:
    v0 = iconst.i8 0
    v1 = icmp_imm eq v0, 127  ; v0 = 0
    brif v1, block1, block3

block3:
    v2 = icmp_imm.i8 eq v0, 1  ; v0 = 0
    brif v2, block2, block0"
        )
    }

    #[test]
    fn switch_optimal_codegen() {
        let func = setup!(0, [-1i8 as u8 as u128, 0, 1,]);
        assert_eq_output!(
            func,
            "block0:
    v0 = iconst.i8 0
    v1 = icmp_imm eq v0, -1  ; v0 = 0
    brif v1, block1, block4

block4:
    v2 = uextend.i32 v0  ; v0 = 0
    br_table v2, block0, [block2, block3]"
        );
    }

    #[test]
    #[should_panic(
        expected = "The index type i8 does not fit the maximum switch entry of 4683743612477887600"
    )]
    fn switch_rejects_small_inputs() {
        // This is a regression test for a bug that we found where we would emit a cmp
        // with a type that was not able to fully represent a large index.
        //
        // See: https://github.com/bytecodealliance/wasmtime/pull/4502#issuecomment-1191961677
        setup!(1, [0x4100_0000_00bf_d470,]);
    }

    #[test]
    fn switch_seal_generated_blocks() {
        let cases = &[vec![0, 1, 2], vec![0, 1, 2, 10, 11, 12, 20, 30, 40, 50]];

        for case in cases {
            for typ in &[types::I8, types::I16, types::I32, types::I64, types::I128] {
                eprintln!("Testing {typ:?} with keys: {case:?}");
                do_case(case, *typ);
            }
        }

        fn do_case(keys: &[u128], typ: Type) {
            let mut func = Function::new();
            let mut builder_ctx = FunctionBuilderContext::new();
            let mut builder = FunctionBuilder::new(&mut func, &mut builder_ctx);

            let root_block = builder.create_block();
            let default_block = builder.create_block();
            let mut switch = Switch::new();

            let case_blocks = keys
                .iter()
                .map(|key| {
                    let block = builder.create_block();
                    switch.set_entry(*key, block);
                    block
                })
                .collect::<Vec<_>>();

            builder.seal_block(root_block);
            builder.switch_to_block(root_block);

            let val = builder.ins().iconst(typ, 1);
            switch.emit(&mut builder, val, default_block);

            for &block in case_blocks.iter().chain(std::iter::once(&default_block)) {
                builder.seal_block(block);
                builder.switch_to_block(block);
                builder.ins().return_(&[]);
            }

            builder.finalize(); // Will panic if some blocks are not sealed
        }
    }

    #[test]
    fn switch_64bit() {
        let mut func = Function::new();
        let mut func_ctx = FunctionBuilderContext::new();
        {
            let mut bx = FunctionBuilder::new(&mut func, &mut func_ctx);
            let block0 = bx.create_block();
            bx.switch_to_block(block0);
            let val = bx.ins().iconst(types::I64, 0);
            let mut switch = Switch::new();
            let block1 = bx.create_block();
            switch.set_entry(1, block1);
            let block2 = bx.create_block();
            switch.set_entry(0, block2);
            let block3 = bx.create_block();
            switch.emit(&mut bx, val, block3);
        }
        let func = func
            .to_string()
            .trim_start_matches("function u0:0() fast {\n")
            .trim_end_matches("\n}\n")
            .to_string();
        assert_eq_output!(
            func,
            "block0:
    v0 = iconst.i64 0
    v1 = icmp_imm ugt v0, 0xffff_ffff  ; v0 = 0
    brif v1, block3, block4

block4:
    v2 = ireduce.i32 v0  ; v0 = 0
    br_table v2, block3, [block2, block1]"
        );
    }

    #[test]
    fn switch_128bit() {
        let mut func = Function::new();
        let mut func_ctx = FunctionBuilderContext::new();
        {
            let mut bx = FunctionBuilder::new(&mut func, &mut func_ctx);
            let block0 = bx.create_block();
            bx.switch_to_block(block0);
            let val = bx.ins().iconst(types::I64, 0);
            let val = bx.ins().uextend(types::I128, val);
            let mut switch = Switch::new();
            let block1 = bx.create_block();
            switch.set_entry(1, block1);
            let block2 = bx.create_block();
            switch.set_entry(0, block2);
            let block3 = bx.create_block();
            switch.emit(&mut bx, val, block3);
        }
        let func = func
            .to_string()
            .trim_start_matches("function u0:0() fast {\n")
            .trim_end_matches("\n}\n")
            .to_string();
        assert_eq_output!(
            func,
            "block0:
    v0 = iconst.i64 0
    v1 = uextend.i128 v0  ; v0 = 0
    v2 = icmp_imm ugt v1, 0xffff_ffff
    brif v2, block3, block4

block4:
    v3 = ireduce.i32 v1
    br_table v3, block3, [block2, block1]"
        );
    }

    #[test]
    fn switch_128bit_max_u64() {
        let mut func = Function::new();
        let mut func_ctx = FunctionBuilderContext::new();
        {
            let mut bx = FunctionBuilder::new(&mut func, &mut func_ctx);
            let block0 = bx.create_block();
            bx.switch_to_block(block0);
            let val = bx.ins().iconst(types::I64, 0);
            let val = bx.ins().uextend(types::I128, val);
            let mut switch = Switch::new();
            let block1 = bx.create_block();
            switch.set_entry(u64::MAX.into(), block1);
            let block2 = bx.create_block();
            switch.set_entry(0, block2);
            let block3 = bx.create_block();
            switch.emit(&mut bx, val, block3);
        }
        let func = func
            .to_string()
            .trim_start_matches("function u0:0() fast {\n")
            .trim_end_matches("\n}\n")
            .to_string();
        assert_eq_output!(
            func,
            "block0:
    v0 = iconst.i64 0
    v1 = uextend.i128 v0  ; v0 = 0
    v2 = iconst.i64 -1
    v3 = iconst.i64 0
    v4 = iconcat v2, v3  ; v2 = -1, v3 = 0
    v5 = icmp eq v1, v4
    brif v5, block1, block4

block4:
    brif.i128 v1, block3, block2"
        );
    }
}
