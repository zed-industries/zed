use super::{Config, Print, PrintTermcolor, Printer, State};
use anyhow::{Result, anyhow, bail};
use termcolor::{Ansi, NoColor};
use wasmparser::VisitSimdOperator;
use wasmparser::{
    BlockType, BrTable, Catch, CompositeInnerType, ContType, FrameKind, FuncType, Handle, MemArg,
    ModuleArity, Operator, OperatorsReader, Ordering, RefType, ResumeTable, SubType, TryTable,
    VisitOperator,
};

pub struct OperatorState {
    op_offset: usize,
    nesting_start: u32,
    label: u32,
    label_indices: Vec<u32>,
    sep: OperatorSeparator,
}

impl OperatorState {
    pub fn new(printer: &Printer, sep: OperatorSeparator) -> Self {
        OperatorState {
            op_offset: 0,
            nesting_start: printer.nesting,
            label: 0,
            label_indices: Vec::new(),
            sep,
        }
    }
}

pub struct PrintOperator<'printer, 'state, 'a, 'b> {
    pub(super) printer: &'printer mut Printer<'a, 'b>,
    state: &'state mut State,
    operator_state: &'printer mut OperatorState,
}

struct FoldedInstruction {
    plain: String,
    folded: Vec<FoldedInstruction>,
    results: u32,
    offset: usize,
}

struct Block {
    ty: BlockType,
    kind: FrameKind,
    plain: String,
    folded: Vec<FoldedInstruction>,
    predicate: Option<Vec<FoldedInstruction>>,
    consequent: Option<(Vec<FoldedInstruction>, usize)>,
    offset: usize,
}

pub struct PrintOperatorFolded<'printer, 'state, 'a, 'b> {
    pub(super) printer: &'printer mut Printer<'a, 'b>,
    state: &'state mut State,
    operator_state: &'printer mut OperatorState,
    control: Vec<Block>,
    branch_hint: Option<FoldedInstruction>,
    original_separator: OperatorSeparator,
}

#[derive(Copy, Clone)]
pub enum OperatorSeparator {
    Newline,
    None,
    NoneThenSpace,
    Space,
}

impl<'printer, 'state, 'a, 'b> PrintOperator<'printer, 'state, 'a, 'b> {
    pub(super) fn new(
        printer: &'printer mut Printer<'a, 'b>,
        state: &'state mut State,
        operator_state: &'printer mut OperatorState,
    ) -> Self {
        PrintOperator {
            printer,
            state,
            operator_state,
        }
    }

    fn push_str(&mut self, s: &str) -> Result<()> {
        self.printer.result.write_str(s)?;
        Ok(())
    }

    fn result(&mut self) -> &mut dyn Print {
        self.printer.result
    }

    fn separator(&mut self) -> Result<()> {
        match self.operator_state.sep {
            OperatorSeparator::Newline => self.printer.newline(self.operator_state.op_offset),
            OperatorSeparator::None => Ok(()),
            OperatorSeparator::NoneThenSpace => {
                self.operator_state.sep = OperatorSeparator::Space;
                Ok(())
            }
            OperatorSeparator::Space => self.push_str(" "),
        }
    }

    /// Called just before an instruction that introduces a block such as
    /// `block`, `if`, `loop`, etc.
    fn block_start(&mut self) -> Result<()> {
        self.separator()?;
        self.printer.nesting += 1;
        self.operator_state
            .label_indices
            .push(self.operator_state.label);
        Ok(())
    }

    /// Used for `else` and `delegate`
    fn block_mid(&mut self) -> Result<()> {
        self.printer.nesting -= 1;
        self.separator()?;
        self.printer.nesting += 1;
        Ok(())
    }

    /// Used for `end` to terminate the prior block.
    fn block_end(&mut self) -> Result<()> {
        if self.printer.nesting > self.operator_state.nesting_start {
            self.printer.nesting -= 1;
        }
        self.separator()?;
        Ok(())
    }

    fn blockty(&mut self, ty: BlockType) -> Result<()> {
        let has_name = self.blockty_without_label_comment(ty)?;
        self.maybe_blockty_label_comment(has_name)
    }

    fn blockty_without_label_comment(&mut self, ty: BlockType) -> Result<bool> {
        let key = (self.state.core.funcs, self.operator_state.label);
        let has_name = match self.state.core.label_names.index_to_name.get(&key) {
            Some(name) => {
                write!(self.printer.result, " ")?;
                name.write(self.printer)?;
                true
            }
            None if self.printer.config.name_unnamed => {
                // Subtract one from the depth here because the label was
                // already pushed onto our stack when the instruction was
                // entered so its own label is one less.
                let depth = self.cur_depth() - 1;
                write!(self.result(), " $#label{depth}")?;
                true
            }
            None => false,
        };
        match ty {
            BlockType::Empty => {}
            BlockType::Type(t) => {
                self.push_str(" ")?;
                self.printer.start_group("result ")?;
                self.printer.print_valtype(self.state, t)?;
                self.printer.end_group()?;
            }
            BlockType::FuncType(idx) => {
                self.push_str(" ")?;
                self.printer
                    .print_core_functype_idx(self.state, idx, None)?;
            }
        }
        Ok(has_name)
    }

    fn maybe_blockty_label_comment(&mut self, has_name: bool) -> Result<()> {
        if !has_name {
            let depth = self.cur_depth();
            self.push_str(" ")?;
            self.result().start_comment()?;
            match self.operator_state.sep {
                OperatorSeparator::Newline | OperatorSeparator::None => {
                    write!(self.result(), ";; label = @{depth}")
                }
                _ => write!(self.result(), " (; label = @{depth} ;)"),
            }?;
            self.result().reset_color()?;
        }

        self.operator_state.label += 1;
        Ok(())
    }

    fn cur_depth(&self) -> u32 {
        self.printer.nesting - self.operator_state.nesting_start
    }

    fn tag_index(&mut self, index: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.tag_names, index)?;
        Ok(())
    }

    fn relative_depth(&mut self, depth: u32) -> Result<()> {
        self.push_str(" ")?;
        match self.cur_depth().checked_sub(depth) {
            // If this relative depth is in-range relative to the current depth,
            // then try to print a name for this label. Label names are tracked
            // as a stack where the depth matches `cur_depth` roughly, but label
            // names don't account for the function name so offset by one more
            // here.
            Some(i) => {
                let name = i
                    .checked_sub(1)
                    .and_then(|idx| self.operator_state.label_indices.get(idx as usize).copied())
                    .and_then(|label_idx| {
                        let key = (self.state.core.funcs, label_idx);
                        self.state.core.label_names.index_to_name.get(&key)
                    });

                // This is a bit tricky, but if there's a shallower label than
                // this target which shares the same name then we can't print
                // the name-based version. Names resolve to the nearest label
                // in the case of shadowing, which would be the wrong behavior
                // here. All that can be done is to print the index down below
                // instead.
                let name_conflict = name.is_some()
                    && self.operator_state.label_indices[i as usize..]
                        .iter()
                        .any(|other_label| {
                            let key = (self.state.core.funcs, *other_label);
                            if let Some(other) = self.state.core.label_names.index_to_name.get(&key)
                            {
                                if name.unwrap().name == other.name {
                                    return true;
                                }
                            }
                            false
                        });

                match name {
                    // Only print the name if one is found and there's also no
                    // name conflict.
                    Some(name) if !name_conflict => name.write(self.printer)?,

                    // If there's no name conflict, and we're synthesizing
                    // names, and this isn't targeting the function itself then
                    // print a synthesized names.
                    //
                    // Note that synthesized label names don't handle the
                    // function itself, so i==0, branching to a function label,
                    // is not supported and otherwise labels are offset by 1.
                    None if !name_conflict && self.printer.config.name_unnamed && i > 0 => {
                        self.result().start_name()?;
                        write!(self.result(), "$#label{}", i - 1)?;
                        self.result().reset_color()?;
                    }

                    _ => {
                        // Last-ditch resort, we gotta print the index.
                        self.result().start_name()?;
                        write!(self.result(), "{depth}")?;
                        self.result().reset_color()?;

                        // Unnamed labels have helpful `@N` labels printed for
                        // them so also try to print where this index is going
                        // (label-wise). Don't do this for a name conflict
                        // though because we wouldn't have printed the numbered
                        // label, and also don't do it for the function itself
                        // since the function has no label we can synthesize.
                        if !name_conflict && i > 0 {
                            self.result().start_comment()?;
                            write!(self.result(), " (;@{i};)")?;
                            self.result().reset_color()?;
                        }
                    }
                }
            }

            // This branch is out of range. Print the raw integer and then leave
            // a hopefully-helpful comment indicating that it's going nowhere.
            None => write!(self.result(), "{depth} (; INVALID ;)")?,
        }
        Ok(())
    }

    fn targets(&mut self, targets: BrTable<'_>) -> Result<()> {
        for item in targets.targets().chain([Ok(targets.default())]) {
            self.relative_depth(item?)?;
        }
        Ok(())
    }

    fn function_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.func_names, idx)
    }

    fn local_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer
            .print_local_idx(self.state, self.state.core.funcs, idx)
    }

    fn global_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.global_names, idx)
    }

    fn table_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.table_names, idx)
    }

    fn table(&mut self, idx: u32) -> Result<()> {
        self.table_index(idx)
    }

    fn memory_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.memory_names, idx)
    }

    fn type_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_core_type_ref(self.state, idx)
    }

    fn cont_type_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.type_names, idx)
    }

    fn argument_index(&mut self, idx: u32) -> Result<()> {
        self.cont_type_index(idx)
    }

    fn result_index(&mut self, idx: u32) -> Result<()> {
        self.cont_type_index(idx)
    }

    fn array_type_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.type_names, idx)
    }

    fn array_type_index_dst(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.type_names, idx)
    }

    fn array_type_index_src(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.type_names, idx)
    }

    fn array_size(&mut self, array_size: u32) -> Result<()> {
        write!(&mut self.printer.result, " {array_size}")?;
        Ok(())
    }

    fn struct_type_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.type_names, idx)
    }

    fn from_ref_type(&mut self, ref_ty: RefType) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_reftype(self.state, ref_ty)
    }

    fn to_ref_type(&mut self, ref_ty: RefType) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_reftype(self.state, ref_ty)
    }

    fn data_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.data_names, idx)
    }

    fn array_data_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.data_names, idx)
    }

    fn elem_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.element_names, idx)
    }

    fn array_elem_index(&mut self, idx: u32) -> Result<()> {
        self.push_str(" ")?;
        self.printer.print_idx(&self.state.core.element_names, idx)
    }

    fn lane(&mut self, lane: u8) -> Result<()> {
        write!(self.result(), " {lane}")?;
        Ok(())
    }

    fn lanes(&mut self, lanes: [u8; 16]) -> Result<()> {
        for lane in lanes.iter() {
            write!(self.result(), " {lane}")?;
        }
        Ok(())
    }

    fn memarg(&mut self, memarg: MemArg) -> Result<()> {
        if memarg.memory != 0 {
            self.memory_index(memarg.memory)?;
        }
        if memarg.offset != 0 {
            write!(self.result(), " offset={}", memarg.offset)?;
        }
        if memarg.align != memarg.max_align {
            let align = 1_u64 << memarg.align;
            write!(self.result(), " align={align}")?;
        }
        Ok(())
    }

    fn ordering(&mut self, ordering: Ordering) -> Result<()> {
        write!(
            self.result(),
            " {}",
            match ordering {
                Ordering::SeqCst => "seq_cst",
                Ordering::AcqRel => "acq_rel",
            }
        )?;
        Ok(())
    }

    fn try_table(&mut self, table: TryTable) -> Result<()> {
        let has_name = self.blockty_without_label_comment(table.ty)?;

        // Nesting has already been incremented but labels for catch start above
        // this `try_table` not at the `try_table`. Temporarily decrement this
        // nesting count and increase it below after printing catch clauses.
        self.printer.nesting -= 2;
        let try_table_label = self.operator_state.label_indices.pop().unwrap();

        for catch in table.catches {
            self.result().write_str(" ")?;
            match catch {
                Catch::One { tag, label } => {
                    self.printer.start_group("catch")?;
                    self.tag_index(tag)?;
                    self.relative_depth(label)?;
                    self.printer.end_group()?;
                }
                Catch::OneRef { tag, label } => {
                    self.printer.start_group("catch_ref")?;
                    self.tag_index(tag)?;
                    self.relative_depth(label)?;
                    self.printer.end_group()?;
                }
                Catch::All { label } => {
                    self.printer.start_group("catch_all")?;
                    self.relative_depth(label)?;
                    self.printer.end_group()?;
                }
                Catch::AllRef { label } => {
                    self.printer.start_group("catch_all_ref")?;
                    self.relative_depth(label)?;
                    self.printer.end_group()?;
                }
            }
        }
        self.operator_state.label_indices.push(try_table_label);
        self.printer.nesting += 2;
        self.maybe_blockty_label_comment(has_name)?;
        Ok(())
    }

    fn resume_table(&mut self, table: ResumeTable) -> Result<()> {
        // The start_group("resume/resume_throw") have already
        // increased the nesting depth, but the labels are defined
        // above this `resume` or `resume_throw`. Therefore we
        // temporarily decrement this nesting count and increase it
        // below after printing the on clauses.
        self.printer.nesting -= 1;
        for handle in table.handlers {
            self.result().write_str(" ")?;
            self.printer.start_group("on")?;
            match handle {
                Handle::OnLabel { tag, label } => {
                    self.tag_index(tag)?;
                    self.relative_depth(label)?;
                }
                Handle::OnSwitch { tag } => {
                    self.tag_index(tag)?;
                    self.result().write_str(" switch")?;
                }
            }
            self.printer.end_group()?;
        }
        self.printer.nesting += 1;
        Ok(())
    }
}

macro_rules! define_visit {
    // General structure of all the operator printer methods:
    //
    // * Print the name of the insruction as defined in this macro
    // * Print any payload, as necessary
    ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*) )*) => ($(
        fn $visit(&mut self $( , $($arg: $argty),* )?) -> Self::Output {
            define_visit!(before_op self $op);
            self.push_str(define_visit!(name $op))?;
            $(
                define_visit!(payload self $op $($arg)*);
            )?

            define_visit!(after_op self $op);
            Ok(())
        }
    )*);

    // Control-flow related opcodes have special handling to manage nested
    // depth as well as the stack of labels.
    //
    // The catch-all for "before an op" is "print an newline"
    (before_op $self:ident Loop) => ($self.block_start()?;);
    (before_op $self:ident Block) => ($self.block_start()?;);
    (before_op $self:ident If) => ($self.block_start()?;);
    (before_op $self:ident Try) => ($self.block_start()?;);
    (before_op $self:ident TryTable) => ($self.block_start()?;);
    (before_op $self:ident Catch) => ($self.block_mid()?;);
    (before_op $self:ident CatchAll) => ($self.block_mid()?;);
    (before_op $self:ident Delegate) => ($self.block_end()?;);
    (before_op $self:ident Else) => ($self.block_mid()?;);
    (before_op $self:ident End) => ($self.block_end()?;);
    (before_op $self:ident $op:ident) => ($self.separator()?;);

    // After some opcodes the label stack is popped.
    // (after_op $self:ident Delegate) => ($self.label_indices.pop(););
    (after_op $self:ident End) => ($self.operator_state.label_indices.pop(););
    (after_op $self:ident $op:ident) => ();

    // How to print the payload of an instruction. There are a number of
    // instructions that have special cases such as avoiding printing anything
    // when an index is 0 or similar. The final case in this list is the
    // catch-all which prints each payload individually based on the name of the
    // payload field.
    (payload $self:ident CallIndirect $ty:ident $table:ident) => (
        if $table != 0 {
            $self.table_index($table)?;
        }
        $self.type_index($ty)?;
    );
    (payload $self:ident ReturnCallIndirect $ty:ident $table:ident) => (
        if $table != 0 {
            $self.table_index($table)?;
        }
        $self.type_index($ty)?;
    );
    (payload $self:ident CallRef $ty:ident) => (
        $self.push_str(" ")?;
        $self.printer.print_idx(&$self.state.core.type_names, $ty)?;
    );
    (payload $self:ident ReturnCallRef $ty:ident) => (
        $self.push_str(" ")?;
        $self.printer.print_idx(&$self.state.core.type_names, $ty)?;
    );
    (payload $self:ident TypedSelect $select_ty:ident) => (
        $self.push_str(" ")?;
        $self.printer.start_group("result ")?;
        $self.printer.print_valtype($self.state, $select_ty)?;
        $self.printer.end_group()?;
    );
    (payload $self:ident TypedSelectMulti $select_tys:ident) => (
        $self.push_str(" ")?;
        $self.printer.start_group("result")?;
        $self.printer.print_valtypes($self.state, $select_tys)?;
        $self.printer.end_group()?;
    );
    (payload $self:ident RefNull $hty:ident) => (
        $self.push_str(" ")?;
        $self.printer.print_heaptype($self.state, $hty)?;
    );
    (payload $self:ident TableInit $segment:ident $table:ident) => (
        if $table != 0 {
            $self.table_index($table)?;
        }
        $self.elem_index($segment)?;
    );
    (payload $self:ident TableCopy $dst:ident $src:ident) => (
        if $src != 0 || $dst != 0 {
            $self.table_index($dst)?;
            $self.table_index($src)?;
        }
    );
    (payload $self:ident MemoryGrow $mem:ident) => (
        if $mem != 0 {
            $self.memory_index($mem)?;
        }
    );
    (payload $self:ident MemorySize $mem:ident) => (
        if $mem != 0 {
            $self.memory_index($mem)?;
        }
    );
    (payload $self:ident MemoryInit $segment:ident $mem:ident) => (
        if $mem != 0 {
            $self.memory_index($mem)?;
        }
        $self.data_index($segment)?;
    );
    (payload $self:ident MemoryCopy $dst:ident $src:ident) => (
        if $src != 0 || $dst != 0 {
            $self.memory_index($dst)?;
            $self.memory_index($src)?;
        }
    );
    (payload $self:ident MemoryFill $mem:ident) => (
        if $mem != 0 {
            $self.memory_index($mem)?;
        }
    );
    (payload $self:ident MemoryDiscard $mem:ident) => (
        if $mem != 0 {
            $self.memory_index($mem)?;
        }
    );
    (payload $self:ident I32Const $val:ident) => (
        $self.result().start_literal()?;
        write!($self.result(), " {}", $val)?;
        $self.result().reset_color()?;
    );
    (payload $self:ident I64Const $val:ident) => (
        $self.result().start_literal()?;
        write!($self.result(), " {}", $val)?;
        $self.result().reset_color()?;
    );
    (payload $self:ident F32Const $val:ident) => (
        $self.push_str(" ")?;
        $self.printer.print_f32($val.bits())?;
    );
    (payload $self:ident F64Const $val:ident) => (
        $self.push_str(" ")?;
        $self.printer.print_f64($val.bits())?;
    );
    (payload $self:ident V128Const $val:ident) => (
        $self.printer.print_type_keyword(" i32x4")?;
        $self.result().start_literal()?;
        for chunk in $val.bytes().chunks(4) {
            write!(
                $self.result(),
                " 0x{:02x}{:02x}{:02x}{:02x}",
                chunk[3],
                chunk[2],
                chunk[1],
                chunk[0],
            )?;
        }
        $self.result().reset_color()?;
    );
    (payload $self:ident RefTestNonNull $hty:ident) => (
        $self.push_str(" ")?;
        let rty = RefType::new(false, $hty)
            .ok_or_else(|| anyhow!("implementation limit: type index too large"))?;
        $self.printer.print_reftype($self.state, rty)?;
    );
    (payload $self:ident RefTestNullable $hty:ident) => (
        $self.push_str(" ")?;
        let rty = RefType::new(true, $hty)
            .ok_or_else(|| anyhow!("implementation limit: type index too large"))?;
        $self.printer.print_reftype($self.state, rty)?;
    );
    (payload $self:ident RefCastNonNull $hty:ident) => (
        $self.push_str(" ")?;
        let rty = RefType::new(false, $hty)
            .ok_or_else(|| anyhow!("implementation limit: type index too large"))?;
        $self.printer.print_reftype($self.state, rty)?;
    );
    (payload $self:ident RefCastNullable $hty:ident) => (
        $self.push_str(" ")?;
        let rty = RefType::new(true, $hty)
            .ok_or_else(|| anyhow!("implementation limit: type index too large"))?;
        $self.printer.print_reftype($self.state, rty)?;
    );
    (payload $self:ident StructGet $ty:ident $field:ident) => (
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructGetS $ty:ident $field:ident) => (
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructGetU $ty:ident $field:ident) => (
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructSet $ty:ident $field:ident) => (
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructAtomicGet $order:ident $ty:ident $field:ident) => (
        $self.ordering($order)?;
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructAtomicGetS $order:ident $ty:ident $field:ident) => (
        $self.ordering($order)?;
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructAtomicGetU $order:ident $ty:ident $field:ident) => (
        $self.ordering($order)?;
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructAtomicSet $order:ident $ty:ident $field:ident) => (
        $self.ordering($order)?;
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructAtomicRmwAdd $order:ident $ty:ident $field:ident) => (
        $self.ordering($order)?;
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructAtomicRmwSub $order:ident $ty:ident $field:ident) => (
        $self.ordering($order)?;
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructAtomicRmwAnd $order:ident $ty:ident $field:ident) => (
        $self.ordering($order)?;
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructAtomicRmwOr $order:ident $ty:ident $field:ident) => (
        $self.ordering($order)?;
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructAtomicRmwXor $order:ident $ty:ident $field:ident) => (
        $self.ordering($order)?;
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructAtomicRmwXchg $order:ident $ty:ident $field:ident) => (
        $self.ordering($order)?;
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident StructAtomicRmwCmpxchg $order:ident $ty:ident $field:ident) => (
        $self.ordering($order)?;
        $self.struct_type_index($ty)?;
        $self.push_str(" ")?;
        $self.printer.print_field_idx($self.state, $ty, $field)?;
    );
    (payload $self:ident $op:ident $($arg:ident)*) => (
        $($self.$arg($arg)?;)*
    );

    (name Block) => ("block");
    (name If) => ("if");
    (name Else) => ("else");
    (name Loop) => ("loop");
    (name End) => ("end");
    (name Unreachable) => ("unreachable");
    (name Nop) => ("nop");
    (name Br) => ("br");
    (name BrIf) => ("br_if");
    (name BrOnNull) => ("br_on_null");
    (name BrOnNonNull) => ("br_on_non_null");
    (name BrTable) => ("br_table");
    (name Return) => ("return");
    (name Call) => ("call");
    (name CallIndirect) => ("call_indirect");
    (name CallRef) => ("call_ref");
    (name ReturnCall) => ("return_call");
    (name ReturnCallIndirect) => ("return_call_indirect");
    (name ReturnCallRef) => ("return_call_ref");
    (name Drop) => ("drop");
    (name Select) => ("select");
    (name TypedSelect) => ("select");
    (name TypedSelectMulti) => ("select");
    (name LocalGet) => ("local.get");
    (name LocalSet) => ("local.set");
    (name LocalTee) => ("local.tee");
    (name GlobalGet) => ("global.get");
    (name GlobalSet) => ("global.set");
    (name TableGet) => ("table.get");
    (name TableSet) => ("table.set");
    (name I32Load) => ("i32.load");
    (name I64Load) => ("i64.load");
    (name F32Load) => ("f32.load");
    (name F64Load) => ("f64.load");
    (name I32Load8S) => ("i32.load8_s");
    (name I32Load8U) => ("i32.load8_u");
    (name I32Load16S) => ("i32.load16_s");
    (name I32Load16U) => ("i32.load16_u");
    (name I64Load8S) => ("i64.load8_s");
    (name I64Load8U) => ("i64.load8_u");
    (name I64Load16S) => ("i64.load16_s");
    (name I64Load16U) => ("i64.load16_u");
    (name I64Load32S) => ("i64.load32_s");
    (name I64Load32U) => ("i64.load32_u");
    (name I32Store) => ("i32.store");
    (name I64Store) => ("i64.store");
    (name F32Store) => ("f32.store");
    (name F64Store) => ("f64.store");
    (name I32Store8) => ("i32.store8");
    (name I32Store16) => ("i32.store16");
    (name I64Store8) => ("i64.store8");
    (name I64Store16) => ("i64.store16");
    (name I64Store32) => ("i64.store32");
    (name MemorySize) => ("memory.size");
    (name MemoryGrow) => ("memory.grow");
    (name MemoryInit) => ("memory.init");
    (name MemoryCopy) => ("memory.copy");
    (name MemoryFill) => ("memory.fill");
    (name MemoryDiscard) => ("memory.discard");
    (name DataDrop) => ("data.drop");
    (name ElemDrop) => ("elem.drop");
    (name TableInit) => ("table.init");
    (name TableCopy) => ("table.copy");
    (name TableFill) => ("table.fill");
    (name TableSize) => ("table.size");
    (name TableGrow) => ("table.grow");
    (name RefAsNonNull) => ("ref.as_non_null");
    (name RefNull) => ("ref.null");
    (name RefEq) => ("ref.eq");
    (name RefIsNull) => ("ref.is_null");
    (name RefFunc) => ("ref.func");
    (name I32Const) => ("i32.const");
    (name I64Const) => ("i64.const");
    (name F32Const) => ("f32.const");
    (name F64Const) => ("f64.const");
    (name I32Clz) => ("i32.clz");
    (name I32Ctz) => ("i32.ctz");
    (name I32Popcnt) => ("i32.popcnt");
    (name I32Add) => ("i32.add");
    (name I32Sub) => ("i32.sub");
    (name I32Mul) => ("i32.mul");
    (name I32DivS) => ("i32.div_s");
    (name I32DivU) => ("i32.div_u");
    (name I32RemS) => ("i32.rem_s");
    (name I32RemU) => ("i32.rem_u");
    (name I32And) => ("i32.and");
    (name I32Or) => ("i32.or");
    (name I32Xor) => ("i32.xor");
    (name I32Shl) => ("i32.shl");
    (name I32ShrS) => ("i32.shr_s");
    (name I32ShrU) => ("i32.shr_u");
    (name I32Rotl) => ("i32.rotl");
    (name I32Rotr) => ("i32.rotr");
    (name I64Clz) => ("i64.clz");
    (name I64Ctz) => ("i64.ctz");
    (name I64Popcnt) => ("i64.popcnt");
    (name I64Add) => ("i64.add");
    (name I64Sub) => ("i64.sub");
    (name I64Mul) => ("i64.mul");
    (name I64DivS) => ("i64.div_s");
    (name I64DivU) => ("i64.div_u");
    (name I64RemS) => ("i64.rem_s");
    (name I64RemU) => ("i64.rem_u");
    (name I64And) => ("i64.and");
    (name I64Or) => ("i64.or");
    (name I64Xor) => ("i64.xor");
    (name I64Shl) => ("i64.shl");
    (name I64ShrS) => ("i64.shr_s");
    (name I64ShrU) => ("i64.shr_u");
    (name I64Rotl) => ("i64.rotl");
    (name I64Rotr) => ("i64.rotr");
    (name F32Abs) => ("f32.abs");
    (name F32Neg) => ("f32.neg");
    (name F32Ceil) => ("f32.ceil");
    (name F32Floor) => ("f32.floor");
    (name F32Trunc) => ("f32.trunc");
    (name F32Nearest) => ("f32.nearest");
    (name F32Sqrt) => ("f32.sqrt");
    (name F32Add) => ("f32.add");
    (name F32Sub) => ("f32.sub");
    (name F32Mul) => ("f32.mul");
    (name F32Div) => ("f32.div");
    (name F32Min) => ("f32.min");
    (name F32Max) => ("f32.max");
    (name F32Copysign) => ("f32.copysign");
    (name F64Abs) => ("f64.abs");
    (name F64Neg) => ("f64.neg");
    (name F64Ceil) => ("f64.ceil");
    (name F64Floor) => ("f64.floor");
    (name F64Trunc) => ("f64.trunc");
    (name F64Nearest) => ("f64.nearest");
    (name F64Sqrt) => ("f64.sqrt");
    (name F64Add) => ("f64.add");
    (name F64Sub) => ("f64.sub");
    (name F64Mul) => ("f64.mul");
    (name F64Div) => ("f64.div");
    (name F64Min) => ("f64.min");
    (name F64Max) => ("f64.max");
    (name F64Copysign) => ("f64.copysign");
    (name I32Eqz) => ("i32.eqz");
    (name I32Eq) => ("i32.eq");
    (name I32Ne) => ("i32.ne");
    (name I32LtS) => ("i32.lt_s");
    (name I32LtU) => ("i32.lt_u");
    (name I32GtS) => ("i32.gt_s");
    (name I32GtU) => ("i32.gt_u");
    (name I32LeS) => ("i32.le_s");
    (name I32LeU) => ("i32.le_u");
    (name I32GeS) => ("i32.ge_s");
    (name I32GeU) => ("i32.ge_u");
    (name I64Eqz) => ("i64.eqz");
    (name I64Eq) => ("i64.eq");
    (name I64Ne) => ("i64.ne");
    (name I64LtS) => ("i64.lt_s");
    (name I64LtU) => ("i64.lt_u");
    (name I64GtS) => ("i64.gt_s");
    (name I64GtU) => ("i64.gt_u");
    (name I64LeS) => ("i64.le_s");
    (name I64LeU) => ("i64.le_u");
    (name I64GeS) => ("i64.ge_s");
    (name I64GeU) => ("i64.ge_u");
    (name F32Eq) => ("f32.eq");
    (name F32Ne) => ("f32.ne");
    (name F32Lt) => ("f32.lt");
    (name F32Gt) => ("f32.gt");
    (name F32Le) => ("f32.le");
    (name F32Ge) => ("f32.ge");
    (name F64Eq) => ("f64.eq");
    (name F64Ne) => ("f64.ne");
    (name F64Lt) => ("f64.lt");
    (name F64Gt) => ("f64.gt");
    (name F64Le) => ("f64.le");
    (name F64Ge) => ("f64.ge");
    (name I32WrapI64) => ("i32.wrap_i64");
    (name I32TruncF32S) => ("i32.trunc_f32_s");
    (name I32TruncF32U) => ("i32.trunc_f32_u");
    (name I32TruncF64S) => ("i32.trunc_f64_s");
    (name I32TruncF64U) => ("i32.trunc_f64_u");
    (name I64ExtendI32S) => ("i64.extend_i32_s");
    (name I64ExtendI32U) => ("i64.extend_i32_u");
    (name I64TruncF32S) => ("i64.trunc_f32_s");
    (name I64TruncF32U) => ("i64.trunc_f32_u");
    (name I64TruncF64S) => ("i64.trunc_f64_s");
    (name I64TruncF64U) => ("i64.trunc_f64_u");
    (name F32ConvertI32S) => ("f32.convert_i32_s");
    (name F32ConvertI32U) => ("f32.convert_i32_u");
    (name F32ConvertI64S) => ("f32.convert_i64_s");
    (name F32ConvertI64U) => ("f32.convert_i64_u");
    (name F32DemoteF64) => ("f32.demote_f64");
    (name F64ConvertI32S) => ("f64.convert_i32_s");
    (name F64ConvertI32U) => ("f64.convert_i32_u");
    (name F64ConvertI64S) => ("f64.convert_i64_s");
    (name F64ConvertI64U) => ("f64.convert_i64_u");
    (name F64PromoteF32) => ("f64.promote_f32");
    (name I32ReinterpretF32) => ("i32.reinterpret_f32");
    (name I64ReinterpretF64) => ("i64.reinterpret_f64");
    (name F32ReinterpretI32) => ("f32.reinterpret_i32");
    (name F64ReinterpretI64) => ("f64.reinterpret_i64");
    (name I32TruncSatF32S) => ("i32.trunc_sat_f32_s");
    (name I32TruncSatF32U) => ("i32.trunc_sat_f32_u");
    (name I32TruncSatF64S) => ("i32.trunc_sat_f64_s");
    (name I32TruncSatF64U) => ("i32.trunc_sat_f64_u");
    (name I64TruncSatF32S) => ("i64.trunc_sat_f32_s");
    (name I64TruncSatF32U) => ("i64.trunc_sat_f32_u");
    (name I64TruncSatF64S) => ("i64.trunc_sat_f64_s");
    (name I64TruncSatF64U) => ("i64.trunc_sat_f64_u");
    (name I32Extend8S) => ("i32.extend8_s");
    (name I32Extend16S) => ("i32.extend16_s");
    (name I64Extend8S) => ("i64.extend8_s");
    (name I64Extend16S) => ("i64.extend16_s");
    (name I64Extend32S) => ("i64.extend32_s");
    (name MemoryAtomicNotify) => ("memory.atomic.notify");
    (name MemoryAtomicWait32) => ("memory.atomic.wait32");
    (name MemoryAtomicWait64) => ("memory.atomic.wait64");
    (name AtomicFence) => ("atomic.fence");
    (name I32AtomicLoad) => ("i32.atomic.load");
    (name I64AtomicLoad) => ("i64.atomic.load");
    (name I32AtomicLoad8U) => ("i32.atomic.load8_u");
    (name I32AtomicLoad16U) => ("i32.atomic.load16_u");
    (name I64AtomicLoad8U) => ("i64.atomic.load8_u");
    (name I64AtomicLoad16U) => ("i64.atomic.load16_u");
    (name I64AtomicLoad32U) => ("i64.atomic.load32_u");
    (name I32AtomicStore) => ("i32.atomic.store");
    (name I64AtomicStore) => ("i64.atomic.store");
    (name I32AtomicStore8) => ("i32.atomic.store8");
    (name I32AtomicStore16) => ("i32.atomic.store16");
    (name I64AtomicStore8) => ("i64.atomic.store8");
    (name I64AtomicStore16) => ("i64.atomic.store16");
    (name I64AtomicStore32) => ("i64.atomic.store32");
    (name I32AtomicRmwAdd) => ("i32.atomic.rmw.add");
    (name I64AtomicRmwAdd) => ("i64.atomic.rmw.add");
    (name I32AtomicRmw8AddU) => ("i32.atomic.rmw8.add_u");
    (name I32AtomicRmw16AddU) => ("i32.atomic.rmw16.add_u");
    (name I64AtomicRmw8AddU) => ("i64.atomic.rmw8.add_u");
    (name I64AtomicRmw16AddU) => ("i64.atomic.rmw16.add_u");
    (name I64AtomicRmw32AddU) => ("i64.atomic.rmw32.add_u");
    (name I32AtomicRmwSub) => ("i32.atomic.rmw.sub");
    (name I64AtomicRmwSub) => ("i64.atomic.rmw.sub");
    (name I32AtomicRmw8SubU) => ("i32.atomic.rmw8.sub_u");
    (name I32AtomicRmw16SubU) => ("i32.atomic.rmw16.sub_u");
    (name I64AtomicRmw8SubU) => ("i64.atomic.rmw8.sub_u");
    (name I64AtomicRmw16SubU) => ("i64.atomic.rmw16.sub_u");
    (name I64AtomicRmw32SubU) => ("i64.atomic.rmw32.sub_u");
    (name I32AtomicRmwAnd) => ("i32.atomic.rmw.and");
    (name I64AtomicRmwAnd) => ("i64.atomic.rmw.and");
    (name I32AtomicRmw8AndU) => ("i32.atomic.rmw8.and_u");
    (name I32AtomicRmw16AndU) => ("i32.atomic.rmw16.and_u");
    (name I64AtomicRmw8AndU) => ("i64.atomic.rmw8.and_u");
    (name I64AtomicRmw16AndU) => ("i64.atomic.rmw16.and_u");
    (name I64AtomicRmw32AndU) => ("i64.atomic.rmw32.and_u");
    (name I32AtomicRmwOr) => ("i32.atomic.rmw.or");
    (name I64AtomicRmwOr) => ("i64.atomic.rmw.or");
    (name I32AtomicRmw8OrU) => ("i32.atomic.rmw8.or_u");
    (name I32AtomicRmw16OrU) => ("i32.atomic.rmw16.or_u");
    (name I64AtomicRmw8OrU) => ("i64.atomic.rmw8.or_u");
    (name I64AtomicRmw16OrU) => ("i64.atomic.rmw16.or_u");
    (name I64AtomicRmw32OrU) => ("i64.atomic.rmw32.or_u");
    (name I32AtomicRmwXor) => ("i32.atomic.rmw.xor");
    (name I64AtomicRmwXor) => ("i64.atomic.rmw.xor");
    (name I32AtomicRmw8XorU) => ("i32.atomic.rmw8.xor_u");
    (name I32AtomicRmw16XorU) => ("i32.atomic.rmw16.xor_u");
    (name I64AtomicRmw8XorU) => ("i64.atomic.rmw8.xor_u");
    (name I64AtomicRmw16XorU) => ("i64.atomic.rmw16.xor_u");
    (name I64AtomicRmw32XorU) => ("i64.atomic.rmw32.xor_u");
    (name I32AtomicRmwXchg) => ("i32.atomic.rmw.xchg");
    (name I64AtomicRmwXchg) => ("i64.atomic.rmw.xchg");
    (name I32AtomicRmw8XchgU) => ("i32.atomic.rmw8.xchg_u");
    (name I32AtomicRmw16XchgU) => ("i32.atomic.rmw16.xchg_u");
    (name I64AtomicRmw8XchgU) => ("i64.atomic.rmw8.xchg_u");
    (name I64AtomicRmw16XchgU) => ("i64.atomic.rmw16.xchg_u");
    (name I64AtomicRmw32XchgU) => ("i64.atomic.rmw32.xchg_u");
    (name I32AtomicRmwCmpxchg) => ("i32.atomic.rmw.cmpxchg");
    (name I64AtomicRmwCmpxchg) => ("i64.atomic.rmw.cmpxchg");
    (name I32AtomicRmw8CmpxchgU) => ("i32.atomic.rmw8.cmpxchg_u");
    (name I32AtomicRmw16CmpxchgU) => ("i32.atomic.rmw16.cmpxchg_u");
    (name I64AtomicRmw8CmpxchgU) => ("i64.atomic.rmw8.cmpxchg_u");
    (name I64AtomicRmw16CmpxchgU) => ("i64.atomic.rmw16.cmpxchg_u");
    (name I64AtomicRmw32CmpxchgU) => ("i64.atomic.rmw32.cmpxchg_u");
    (name V128Load) => ("v128.load");
    (name V128Load8x8S) => ("v128.load8x8_s");
    (name V128Load8x8U) => ("v128.load8x8_u");
    (name V128Load16x4S) => ("v128.load16x4_s");
    (name V128Load16x4U) => ("v128.load16x4_u");
    (name V128Load32x2S) => ("v128.load32x2_s");
    (name V128Load32x2U) => ("v128.load32x2_u");
    (name V128Load8Splat) => ("v128.load8_splat");
    (name V128Load16Splat) => ("v128.load16_splat");
    (name V128Load32Splat) => ("v128.load32_splat");
    (name V128Load64Splat) => ("v128.load64_splat");
    (name V128Load32Zero) => ("v128.load32_zero");
    (name V128Load64Zero) => ("v128.load64_zero");
    (name V128Store) => ("v128.store");
    (name V128Load8Lane) => ("v128.load8_lane");
    (name V128Load16Lane) => ("v128.load16_lane");
    (name V128Load32Lane) => ("v128.load32_lane");
    (name V128Load64Lane) => ("v128.load64_lane");
    (name V128Store8Lane) => ("v128.store8_lane");
    (name V128Store16Lane) => ("v128.store16_lane");
    (name V128Store32Lane) => ("v128.store32_lane");
    (name V128Store64Lane) => ("v128.store64_lane");
    (name V128Const) => ("v128.const");
    (name I8x16Shuffle) => ("i8x16.shuffle");
    (name I8x16ExtractLaneS) => ("i8x16.extract_lane_s");
    (name I8x16ExtractLaneU) => ("i8x16.extract_lane_u");
    (name I8x16ReplaceLane) => ("i8x16.replace_lane");
    (name I16x8ExtractLaneS) => ("i16x8.extract_lane_s");
    (name I16x8ExtractLaneU) => ("i16x8.extract_lane_u");
    (name I16x8ReplaceLane) => ("i16x8.replace_lane");
    (name I32x4ExtractLane) => ("i32x4.extract_lane");
    (name I32x4ReplaceLane) => ("i32x4.replace_lane");
    (name I64x2ExtractLane) => ("i64x2.extract_lane");
    (name I64x2ReplaceLane) => ("i64x2.replace_lane");
    (name F32x4ExtractLane) => ("f32x4.extract_lane");
    (name F32x4ReplaceLane) => ("f32x4.replace_lane");
    (name F64x2ExtractLane) => ("f64x2.extract_lane");
    (name F64x2ReplaceLane) => ("f64x2.replace_lane");
    (name I8x16Swizzle) => ("i8x16.swizzle");
    (name I8x16Splat) => ("i8x16.splat");
    (name I16x8Splat) => ("i16x8.splat");
    (name I32x4Splat) => ("i32x4.splat");
    (name I64x2Splat) => ("i64x2.splat");
    (name F32x4Splat) => ("f32x4.splat");
    (name F64x2Splat) => ("f64x2.splat");
    (name I8x16Eq) => ("i8x16.eq");
    (name I8x16Ne) => ("i8x16.ne");
    (name I8x16LtS) => ("i8x16.lt_s");
    (name I8x16LtU) => ("i8x16.lt_u");
    (name I8x16GtS) => ("i8x16.gt_s");
    (name I8x16GtU) => ("i8x16.gt_u");
    (name I8x16LeS) => ("i8x16.le_s");
    (name I8x16LeU) => ("i8x16.le_u");
    (name I8x16GeS) => ("i8x16.ge_s");
    (name I8x16GeU) => ("i8x16.ge_u");
    (name I16x8Eq) => ("i16x8.eq");
    (name I16x8Ne) => ("i16x8.ne");
    (name I16x8LtS) => ("i16x8.lt_s");
    (name I16x8LtU) => ("i16x8.lt_u");
    (name I16x8GtS) => ("i16x8.gt_s");
    (name I16x8GtU) => ("i16x8.gt_u");
    (name I16x8LeS) => ("i16x8.le_s");
    (name I16x8LeU) => ("i16x8.le_u");
    (name I16x8GeS) => ("i16x8.ge_s");
    (name I16x8GeU) => ("i16x8.ge_u");
    (name I32x4Eq) => ("i32x4.eq");
    (name I32x4Ne) => ("i32x4.ne");
    (name I32x4LtS) => ("i32x4.lt_s");
    (name I32x4LtU) => ("i32x4.lt_u");
    (name I32x4GtS) => ("i32x4.gt_s");
    (name I32x4GtU) => ("i32x4.gt_u");
    (name I32x4LeS) => ("i32x4.le_s");
    (name I32x4LeU) => ("i32x4.le_u");
    (name I32x4GeS) => ("i32x4.ge_s");
    (name I32x4GeU) => ("i32x4.ge_u");
    (name I64x2Eq) => ("i64x2.eq");
    (name I64x2Ne) => ("i64x2.ne");
    (name I64x2LtS) => ("i64x2.lt_s");
    (name I64x2GtS) => ("i64x2.gt_s");
    (name I64x2LeS) => ("i64x2.le_s");
    (name I64x2GeS) => ("i64x2.ge_s");
    (name F32x4Eq) => ("f32x4.eq");
    (name F32x4Ne) => ("f32x4.ne");
    (name F32x4Lt) => ("f32x4.lt");
    (name F32x4Gt) => ("f32x4.gt");
    (name F32x4Le) => ("f32x4.le");
    (name F32x4Ge) => ("f32x4.ge");
    (name F64x2Eq) => ("f64x2.eq");
    (name F64x2Ne) => ("f64x2.ne");
    (name F64x2Lt) => ("f64x2.lt");
    (name F64x2Gt) => ("f64x2.gt");
    (name F64x2Le) => ("f64x2.le");
    (name F64x2Ge) => ("f64x2.ge");
    (name V128Not) => ("v128.not");
    (name V128And) => ("v128.and");
    (name V128AndNot) => ("v128.andnot");
    (name V128Or) => ("v128.or");
    (name V128Xor) => ("v128.xor");
    (name V128Bitselect) => ("v128.bitselect");
    (name V128AnyTrue) => ("v128.any_true");
    (name I8x16Abs) => ("i8x16.abs");
    (name I8x16Neg) => ("i8x16.neg");
    (name I8x16Popcnt) => ("i8x16.popcnt");
    (name I8x16AllTrue) => ("i8x16.all_true");
    (name I8x16Bitmask) => ("i8x16.bitmask");
    (name I8x16NarrowI16x8S) => ("i8x16.narrow_i16x8_s");
    (name I8x16NarrowI16x8U) => ("i8x16.narrow_i16x8_u");
    (name I8x16Shl) => ("i8x16.shl");
    (name I8x16ShrS) => ("i8x16.shr_s");
    (name I8x16ShrU) => ("i8x16.shr_u");
    (name I8x16Add) => ("i8x16.add");
    (name I8x16AddSatS) => ("i8x16.add_sat_s");
    (name I8x16AddSatU) => ("i8x16.add_sat_u");
    (name I8x16Sub) => ("i8x16.sub");
    (name I8x16SubSatS) => ("i8x16.sub_sat_s");
    (name I8x16SubSatU) => ("i8x16.sub_sat_u");
    (name I8x16MinS) => ("i8x16.min_s");
    (name I8x16MinU) => ("i8x16.min_u");
    (name I8x16MaxS) => ("i8x16.max_s");
    (name I8x16MaxU) => ("i8x16.max_u");
    (name I8x16AvgrU) => ("i8x16.avgr_u");
    (name I16x8ExtAddPairwiseI8x16S) => ("i16x8.extadd_pairwise_i8x16_s");
    (name I16x8ExtAddPairwiseI8x16U) => ("i16x8.extadd_pairwise_i8x16_u");
    (name I16x8Abs) => ("i16x8.abs");
    (name I16x8Neg) => ("i16x8.neg");
    (name I16x8Q15MulrSatS) => ("i16x8.q15mulr_sat_s");
    (name I16x8AllTrue) => ("i16x8.all_true");
    (name I16x8Bitmask) => ("i16x8.bitmask");
    (name I16x8NarrowI32x4S) => ("i16x8.narrow_i32x4_s");
    (name I16x8NarrowI32x4U) => ("i16x8.narrow_i32x4_u");
    (name I16x8ExtendLowI8x16S) => ("i16x8.extend_low_i8x16_s");
    (name I16x8ExtendHighI8x16S) => ("i16x8.extend_high_i8x16_s");
    (name I16x8ExtendLowI8x16U) => ("i16x8.extend_low_i8x16_u");
    (name I16x8ExtendHighI8x16U) => ("i16x8.extend_high_i8x16_u");
    (name I16x8Shl) => ("i16x8.shl");
    (name I16x8ShrS) => ("i16x8.shr_s");
    (name I16x8ShrU) => ("i16x8.shr_u");
    (name I16x8Add) => ("i16x8.add");
    (name I16x8AddSatS) => ("i16x8.add_sat_s");
    (name I16x8AddSatU) => ("i16x8.add_sat_u");
    (name I16x8Sub) => ("i16x8.sub");
    (name I16x8SubSatS) => ("i16x8.sub_sat_s");
    (name I16x8SubSatU) => ("i16x8.sub_sat_u");
    (name I16x8Mul) => ("i16x8.mul");
    (name I16x8MinS) => ("i16x8.min_s");
    (name I16x8MinU) => ("i16x8.min_u");
    (name I16x8MaxS) => ("i16x8.max_s");
    (name I16x8MaxU) => ("i16x8.max_u");
    (name I16x8AvgrU) => ("i16x8.avgr_u");
    (name I16x8ExtMulLowI8x16S) => ("i16x8.extmul_low_i8x16_s");
    (name I16x8ExtMulHighI8x16S) => ("i16x8.extmul_high_i8x16_s");
    (name I16x8ExtMulLowI8x16U) => ("i16x8.extmul_low_i8x16_u");
    (name I16x8ExtMulHighI8x16U) => ("i16x8.extmul_high_i8x16_u");
    (name I32x4ExtAddPairwiseI16x8S) => ("i32x4.extadd_pairwise_i16x8_s");
    (name I32x4ExtAddPairwiseI16x8U) => ("i32x4.extadd_pairwise_i16x8_u");
    (name I32x4Abs) => ("i32x4.abs");
    (name I32x4Neg) => ("i32x4.neg");
    (name I32x4AllTrue) => ("i32x4.all_true");
    (name I32x4Bitmask) => ("i32x4.bitmask");
    (name I32x4ExtendLowI16x8S) => ("i32x4.extend_low_i16x8_s");
    (name I32x4ExtendHighI16x8S) => ("i32x4.extend_high_i16x8_s");
    (name I32x4ExtendLowI16x8U) => ("i32x4.extend_low_i16x8_u");
    (name I32x4ExtendHighI16x8U) => ("i32x4.extend_high_i16x8_u");
    (name I32x4Shl) => ("i32x4.shl");
    (name I32x4ShrS) => ("i32x4.shr_s");
    (name I32x4ShrU) => ("i32x4.shr_u");
    (name I32x4Add) => ("i32x4.add");
    (name I32x4Sub) => ("i32x4.sub");
    (name I32x4Mul) => ("i32x4.mul");
    (name I32x4MinS) => ("i32x4.min_s");
    (name I32x4MinU) => ("i32x4.min_u");
    (name I32x4MaxS) => ("i32x4.max_s");
    (name I32x4MaxU) => ("i32x4.max_u");
    (name I32x4DotI16x8S) => ("i32x4.dot_i16x8_s");
    (name I32x4ExtMulLowI16x8S) => ("i32x4.extmul_low_i16x8_s");
    (name I32x4ExtMulHighI16x8S) => ("i32x4.extmul_high_i16x8_s");
    (name I32x4ExtMulLowI16x8U) => ("i32x4.extmul_low_i16x8_u");
    (name I32x4ExtMulHighI16x8U) => ("i32x4.extmul_high_i16x8_u");
    (name I64x2Abs) => ("i64x2.abs");
    (name I64x2Neg) => ("i64x2.neg");
    (name I64x2AllTrue) => ("i64x2.all_true");
    (name I64x2Bitmask) => ("i64x2.bitmask");
    (name I64x2ExtendLowI32x4S) => ("i64x2.extend_low_i32x4_s");
    (name I64x2ExtendHighI32x4S) => ("i64x2.extend_high_i32x4_s");
    (name I64x2ExtendLowI32x4U) => ("i64x2.extend_low_i32x4_u");
    (name I64x2ExtendHighI32x4U) => ("i64x2.extend_high_i32x4_u");
    (name I64x2Shl) => ("i64x2.shl");
    (name I64x2ShrS) => ("i64x2.shr_s");
    (name I64x2ShrU) => ("i64x2.shr_u");
    (name I64x2Add) => ("i64x2.add");
    (name I64x2Sub) => ("i64x2.sub");
    (name I64x2Mul) => ("i64x2.mul");
    (name I64x2ExtMulLowI32x4S) => ("i64x2.extmul_low_i32x4_s");
    (name I64x2ExtMulHighI32x4S) => ("i64x2.extmul_high_i32x4_s");
    (name I64x2ExtMulLowI32x4U) => ("i64x2.extmul_low_i32x4_u");
    (name I64x2ExtMulHighI32x4U) => ("i64x2.extmul_high_i32x4_u");
    (name F32x4Ceil) => ("f32x4.ceil");
    (name F32x4Floor) => ("f32x4.floor");
    (name F32x4Trunc) => ("f32x4.trunc");
    (name F32x4Nearest) => ("f32x4.nearest");
    (name F32x4Abs) => ("f32x4.abs");
    (name F32x4Neg) => ("f32x4.neg");
    (name F32x4Sqrt) => ("f32x4.sqrt");
    (name F32x4Add) => ("f32x4.add");
    (name F32x4Sub) => ("f32x4.sub");
    (name F32x4Mul) => ("f32x4.mul");
    (name F32x4Div) => ("f32x4.div");
    (name F32x4Min) => ("f32x4.min");
    (name F32x4Max) => ("f32x4.max");
    (name F32x4PMin) => ("f32x4.pmin");
    (name F32x4PMax) => ("f32x4.pmax");
    (name F64x2Ceil) => ("f64x2.ceil");
    (name F64x2Floor) => ("f64x2.floor");
    (name F64x2Trunc) => ("f64x2.trunc");
    (name F64x2Nearest) => ("f64x2.nearest");
    (name F64x2Abs) => ("f64x2.abs");
    (name F64x2Neg) => ("f64x2.neg");
    (name F64x2Sqrt) => ("f64x2.sqrt");
    (name F64x2Add) => ("f64x2.add");
    (name F64x2Sub) => ("f64x2.sub");
    (name F64x2Mul) => ("f64x2.mul");
    (name F64x2Div) => ("f64x2.div");
    (name F64x2Min) => ("f64x2.min");
    (name F64x2Max) => ("f64x2.max");
    (name F64x2PMin) => ("f64x2.pmin");
    (name F64x2PMax) => ("f64x2.pmax");
    (name I32x4TruncSatF32x4S) => ("i32x4.trunc_sat_f32x4_s");
    (name I32x4TruncSatF32x4U) => ("i32x4.trunc_sat_f32x4_u");
    (name F32x4ConvertI32x4S) => ("f32x4.convert_i32x4_s");
    (name F32x4ConvertI32x4U) => ("f32x4.convert_i32x4_u");
    (name I32x4TruncSatF64x2SZero) => ("i32x4.trunc_sat_f64x2_s_zero");
    (name I32x4TruncSatF64x2UZero) => ("i32x4.trunc_sat_f64x2_u_zero");
    (name F64x2ConvertLowI32x4S) => ("f64x2.convert_low_i32x4_s");
    (name F64x2ConvertLowI32x4U) => ("f64x2.convert_low_i32x4_u");
    (name F32x4DemoteF64x2Zero) => ("f32x4.demote_f64x2_zero");
    (name F64x2PromoteLowF32x4) => ("f64x2.promote_low_f32x4");
    (name I8x16RelaxedSwizzle) => ("i8x16.relaxed_swizzle");
    (name I32x4RelaxedTruncF32x4S) => ("i32x4.relaxed_trunc_f32x4_s");
    (name I32x4RelaxedTruncF32x4U) => ("i32x4.relaxed_trunc_f32x4_u");
    (name I32x4RelaxedTruncF64x2SZero) => ("i32x4.relaxed_trunc_f64x2_s_zero");
    (name I32x4RelaxedTruncF64x2UZero) => ("i32x4.relaxed_trunc_f64x2_u_zero");
    (name F32x4RelaxedMadd) => ("f32x4.relaxed_madd");
    (name F32x4RelaxedNmadd) => ("f32x4.relaxed_nmadd");
    (name F64x2RelaxedMadd) => ("f64x2.relaxed_madd");
    (name F64x2RelaxedNmadd) => ("f64x2.relaxed_nmadd");
    (name I8x16RelaxedLaneselect) => ("i8x16.relaxed_laneselect");
    (name I16x8RelaxedLaneselect) => ("i16x8.relaxed_laneselect");
    (name I32x4RelaxedLaneselect) => ("i32x4.relaxed_laneselect");
    (name I64x2RelaxedLaneselect) => ("i64x2.relaxed_laneselect");
    (name F32x4RelaxedMin) => ("f32x4.relaxed_min");
    (name F32x4RelaxedMax) => ("f32x4.relaxed_max");
    (name F64x2RelaxedMin) => ("f64x2.relaxed_min");
    (name F64x2RelaxedMax) => ("f64x2.relaxed_max");
    (name I16x8RelaxedQ15mulrS) => ("i16x8.relaxed_q15mulr_s");
    (name I16x8RelaxedDotI8x16I7x16S) => ("i16x8.relaxed_dot_i8x16_i7x16_s");
    (name I32x4RelaxedDotI8x16I7x16AddS) => ("i32x4.relaxed_dot_i8x16_i7x16_add_s");
    (name StructNew) => ("struct.new");
    (name StructNewDefault) => ("struct.new_default");
    (name StructGet) => ("struct.get");
    (name StructGetS) => ("struct.get_s");
    (name StructGetU) => ("struct.get_u");
    (name StructSet) => ("struct.set");
    (name ArrayNew) => ("array.new");
    (name ArrayNewDefault) => ("array.new_default");
    (name ArrayNewFixed) => ("array.new_fixed");
    (name ArrayNewData) => ("array.new_data");
    (name ArrayNewElem) => ("array.new_elem");
    (name ArrayGet) => ("array.get");
    (name ArrayGetS) => ("array.get_s");
    (name ArrayGetU) => ("array.get_u");
    (name ArraySet) => ("array.set");
    (name ArrayLen) => ("array.len");
    (name ArrayFill) => ("array.fill");
    (name ArrayCopy) => ("array.copy");
    (name ArrayInitData) => ("array.init_data");
    (name ArrayInitElem) => ("array.init_elem");
    (name AnyConvertExtern) => ("any.convert_extern");
    (name ExternConvertAny) => ("extern.convert_any");
    (name RefTestNonNull) => ("ref.test");
    (name RefTestNullable) => ("ref.test");
    (name RefCastNonNull) => ("ref.cast");
    (name RefCastNullable) => ("ref.cast");
    (name BrOnCast) => ("br_on_cast");
    (name BrOnCastFail) => ("br_on_cast_fail");
    (name RefI31) => ("ref.i31");
    (name I31GetS) => ("i31.get_s");
    (name I31GetU) => ("i31.get_u");
    (name TryTable) => ("try_table");
    (name Throw) => ("throw");
    (name ThrowRef) => ("throw_ref");
    (name Rethrow) => ("rethrow");
    (name Try) => ("try");
    (name Catch) => ("catch");
    (name CatchAll) => ("catch_all");
    (name Delegate) => ("delegate");
    (name GlobalAtomicGet) => ("global.atomic.get");
    (name GlobalAtomicSet) => ("global.atomic.set");
    (name GlobalAtomicRmwAdd) => ("global.atomic.rmw.add");
    (name GlobalAtomicRmwSub) => ("global.atomic.rmw.sub");
    (name GlobalAtomicRmwAnd) => ("global.atomic.rmw.and");
    (name GlobalAtomicRmwOr) => ("global.atomic.rmw.or");
    (name GlobalAtomicRmwXor) => ("global.atomic.rmw.xor");
    (name GlobalAtomicRmwXchg) => ("global.atomic.rmw.xchg");
    (name GlobalAtomicRmwCmpxchg) => ("global.atomic.rmw.cmpxchg");
    (name TableAtomicGet) => ("table.atomic.get");
    (name TableAtomicSet) => ("table.atomic.set");
    (name TableAtomicRmwXchg) => ("table.atomic.rmw.xchg");
    (name TableAtomicRmwCmpxchg) => ("table.atomic.rmw.cmpxchg");
    (name StructAtomicGet) => ("struct.atomic.get");
    (name StructAtomicGetS) => ("struct.atomic.get_s");
    (name StructAtomicGetU) => ("struct.atomic.get_u");
    (name StructAtomicSet) => ("struct.atomic.set");
    (name StructAtomicRmwAdd) => ("struct.atomic.rmw.add");
    (name StructAtomicRmwSub) => ("struct.atomic.rmw.sub");
    (name StructAtomicRmwAnd) => ("struct.atomic.rmw.and");
    (name StructAtomicRmwOr) => ("struct.atomic.rmw.or");
    (name StructAtomicRmwXor) => ("struct.atomic.rmw.xor");
    (name StructAtomicRmwXchg) => ("struct.atomic.rmw.xchg");
    (name StructAtomicRmwCmpxchg) => ("struct.atomic.rmw.cmpxchg");
    (name ArrayAtomicGet) => ("array.atomic.get");
    (name ArrayAtomicGetS) => ("array.atomic.get_s");
    (name ArrayAtomicGetU) => ("array.atomic.get_u");
    (name ArrayAtomicSet) => ("array.atomic.set");
    (name ArrayAtomicRmwAdd) => ("array.atomic.rmw.add");
    (name ArrayAtomicRmwSub) => ("array.atomic.rmw.sub");
    (name ArrayAtomicRmwAnd) => ("array.atomic.rmw.and");
    (name ArrayAtomicRmwOr) => ("array.atomic.rmw.or");
    (name ArrayAtomicRmwXor) => ("array.atomic.rmw.xor");
    (name ArrayAtomicRmwXchg) => ("array.atomic.rmw.xchg");
    (name ArrayAtomicRmwCmpxchg) => ("array.atomic.rmw.cmpxchg");
    (name RefI31Shared) => ("ref.i31_shared");
    (name ContNew) => ("cont.new");
    (name ContBind) => ("cont.bind");
    (name Suspend) => ("suspend");
    (name Resume) => ("resume");
    (name ResumeThrow) => ("resume_throw");
    (name Switch) => ("switch");
    (name I64Add128) => ("i64.add128");
    (name I64Sub128) => ("i64.sub128");
    (name I64MulWideS) => ("i64.mul_wide_s");
    (name I64MulWideU) => ("i64.mul_wide_u");
}

impl<'a> VisitOperator<'a> for PrintOperator<'_, '_, '_, '_> {
    type Output = Result<()>;

    fn simd_visitor(&mut self) -> Option<&mut dyn VisitSimdOperator<'a, Output = Self::Output>> {
        Some(self)
    }

    wasmparser::for_each_visit_operator!(define_visit);
}

impl<'a> VisitSimdOperator<'a> for PrintOperator<'_, '_, '_, '_> {
    wasmparser::for_each_visit_simd_operator!(define_visit);
}

pub trait OpPrinter {
    fn branch_hint(&mut self, offset: usize, taken: bool) -> Result<()>;
    fn set_offset(&mut self, offset: usize);
    fn visit_operator(
        &mut self,
        reader: &mut OperatorsReader<'_>,
        annotation: Option<&str>,
    ) -> Result<()>;
    fn finalize(&mut self, annotation: Option<&str>) -> Result<()>;
    fn use_color(&self) -> bool;
}

impl OpPrinter for PrintOperator<'_, '_, '_, '_> {
    fn branch_hint(&mut self, offset: usize, taken: bool) -> Result<()> {
        self.printer.newline(offset)?;
        let desc = if taken { "\"\\01\"" } else { "\"\\00\"" };
        self.printer.result.start_comment()?;
        write!(self.printer.result, "(@metadata.code.branch_hint {desc})")?;
        self.printer.result.reset_color()?;
        Ok(())
    }

    fn set_offset(&mut self, offset: usize) {
        self.operator_state.op_offset = offset;
    }

    fn visit_operator(
        &mut self,
        reader: &mut OperatorsReader<'_>,
        annotation: Option<&str>,
    ) -> Result<()> {
        reader.visit_operator(self)??;
        if let Some(s) = annotation {
            self.printer.newline_unknown_pos()?;
            self.result().start_comment()?;
            write!(self.result(), ";; {s}")?;
            self.result().reset_color()?;
        }
        Ok(())
    }

    fn finalize(&mut self, annotation: Option<&str>) -> Result<()> {
        if let Some(s) = annotation {
            self.printer.newline_unknown_pos()?;
            self.result().start_comment()?;
            write!(self.printer.result, ";; {s}")?;
            self.result().reset_color()?;
        }
        Ok(())
    }

    fn use_color(&self) -> bool {
        self.printer.result.supports_async_color()
    }
}

impl OpPrinter for PrintOperatorFolded<'_, '_, '_, '_> {
    fn branch_hint(&mut self, offset: usize, taken: bool) -> Result<()> {
        let mut hint = String::new();
        hint.push_str("@metadata.code.branch_hint ");
        hint.push_str(if taken { "\"\\01\"" } else { "\"\\00\"" });
        self.branch_hint = Some(FoldedInstruction {
            plain: hint,
            folded: Vec::new(),
            results: 0,
            offset,
        });
        Ok(())
    }

    fn set_offset(&mut self, offset: usize) {
        self.operator_state.op_offset = offset;
    }

    fn visit_operator(
        &mut self,
        reader: &mut OperatorsReader<'_>,
        annotation: Option<&str>,
    ) -> Result<()> {
        let operator = reader.clone().read()?;
        let (params, results) = operator.operator_arity(self).unwrap_or((0, 0));
        let mut buf_color = PrintTermcolor(Ansi::new(Vec::new()));
        let mut buf_nocolor = PrintTermcolor(NoColor::new(Vec::new()));
        let internal_config = Config {
            name_unnamed: self.printer.config.name_unnamed,
            ..Default::default()
        };
        let mut internal_printer = Printer {
            config: &internal_config,
            result: if self.use_color() {
                &mut buf_color
            } else {
                &mut buf_nocolor
            },
            nesting: self.printer.nesting,
            line: self.printer.line,
            group_lines: Vec::new(),
            code_section_hints: Vec::new(),
        };

        let mut op_printer =
            PrintOperator::new(&mut internal_printer, self.state, self.operator_state);
        reader.visit_operator(&mut op_printer)??;
        if let Some(s) = annotation {
            internal_printer.result.start_comment()?;
            write!(internal_printer.result, " (; {s}")?;
            internal_printer.result.start_comment()?;
            write!(internal_printer.result, " ;)")?;
            internal_printer.result.reset_color()?;
        }

        self.printer.nesting = internal_printer.nesting;
        self.printer.line = internal_printer.line;

        let inst = String::from_utf8(if self.use_color() {
            buf_color.0.into_inner()
        } else {
            buf_nocolor.0.into_inner()
        })
        .expect("invalid UTF-8");

        match operator {
            Operator::Loop { blockty } => self.push_block(blockty, FrameKind::Loop, inst),
            Operator::Block { blockty } => self.push_block(blockty, FrameKind::Block, inst),
            Operator::TryTable { try_table } => {
                self.push_block(try_table.ty, FrameKind::TryTable, inst)
            }
            Operator::If { blockty } => self.push_if(blockty, inst),
            Operator::Else => self.handle_else(),
            Operator::End => self.handle_end(results),

            Operator::Try { .. }
            | Operator::Catch { .. }
            | Operator::CatchAll { .. }
            | Operator::Delegate { .. } => {
                bail!("legacy-exceptions not supported")
            }
            _ => self.handle_plain(inst, params, results),
        }
    }

    // Recurse through the stack and print each folded instruction.
    fn finalize(&mut self, annotation: Option<&str>) -> Result<()> {
        if self.control.len() != 1 {
            bail!("instruction sequence not closed");
        }
        for inst in &self.control.last().unwrap().folded {
            PrintOperatorFolded::print(&mut self.printer, &mut self.original_separator, &inst)?;
        }
        if let Some(s) = annotation {
            self.printer.newline_unknown_pos()?;
            self.printer.result.start_comment()?;
            write!(self.printer.result, ";; {s}")?;
            self.printer.result.reset_color()?;
        }
        Ok(())
    }

    fn use_color(&self) -> bool {
        self.printer.result.supports_async_color()
    }
}

impl ModuleArity for PrintOperatorFolded<'_, '_, '_, '_> {
    fn tag_type_arity(&self, tag_idx: u32) -> Option<(u32, u32)> {
        self.sub_type_arity(
            self.sub_type_at(
                *self
                    .state
                    .core
                    .tag_to_type
                    .get(tag_idx as usize)?
                    .as_ref()?,
            )?,
        )
    }

    fn type_index_of_function(&self, func_idx: u32) -> Option<u32> {
        *self.state.core.func_to_type.get(func_idx as usize)?
    }

    fn sub_type_at(&self, type_idx: u32) -> Option<&SubType> {
        self.state.core.types.get(type_idx as usize)?.as_ref()
    }

    fn func_type_of_cont_type(&self, c: &ContType) -> Option<&FuncType> {
        let st = self.sub_type_at(c.0.unpack().as_module_index()?)?;
        if let CompositeInnerType::Func(ft) = &st.composite_type.inner {
            Some(ft)
        } else {
            None
        }
    }

    fn sub_type_of_ref_type(&self, rt: &RefType) -> Option<&SubType> {
        self.sub_type_at(rt.type_index()?.as_module_index()?)
    }

    fn control_stack_height(&self) -> u32 {
        self.control.len() as u32
    }

    fn label_block(&self, depth: u32) -> Option<(BlockType, FrameKind)> {
        let cur_depth = self.printer.nesting - self.operator_state.nesting_start;
        if self.control.len() != cur_depth as usize + 1 {
            return None;
        }
        match (self.control.len() - 1).checked_sub(depth as usize) {
            Some(i) => Some((self.control[i].ty, self.control[i].kind)),
            None => None,
        }
    }
}

impl<'printer, 'state, 'a, 'b> PrintOperatorFolded<'printer, 'state, 'a, 'b> {
    pub(super) fn new(
        printer: &'printer mut Printer<'a, 'b>,
        state: &'state mut State,
        operator_state: &'printer mut OperatorState,
    ) -> Self {
        let original_separator = operator_state.sep;
        operator_state.sep = OperatorSeparator::None;

        PrintOperatorFolded {
            printer,
            state,
            operator_state,
            control: Vec::new(),
            branch_hint: None,
            original_separator,
        }
    }

    // Set up the outermost block, representing the unnamed function label.
    pub fn begin_function(&mut self, func_idx: u32) -> Result<()> {
        match self.state.core.func_to_type.get(func_idx as usize) {
            Some(Some(type_idx)) => self.control.push(Block {
                ty: BlockType::FuncType(*type_idx),
                kind: FrameKind::Block,
                plain: String::new(),
                folded: Vec::new(),
                predicate: None,
                consequent: None,
                offset: self.operator_state.op_offset,
            }),
            _ => bail!("invalid func_idx"),
        }

        Ok(())
    }

    // Set up a catch-all block to represent the constant expression's operand stack.
    pub fn begin_const_expr(&mut self) {
        self.control.push(Block {
            ty: BlockType::Empty,
            kind: FrameKind::Block,
            plain: String::new(),
            folded: Vec::new(),
            predicate: None,
            consequent: None,
            offset: 0,
        });
    }

    // Handle a "plain" (non-block) instruction. The behavior resembles WABT's WatWriter::PushExpr().
    // Each instruction will pop some number of operands off the stack to become the "children"
    // of the foldedinst phrase. In the presence of multi-value instructions with more than 1 result,
    // it may not be possible to hit the params target exactly. This doesn't necessarily mean the
    // Wasm is invalid, but it can't be represented sensibly in folded form.
    fn handle_plain(&mut self, plain: String, params: u32, mut results: u32) -> Result<()> {
        let stack = match self.control.last_mut() {
            Some(stack) => stack,
            None => bail!("instruction without enclosing block"),
        };

        let mut first_param = stack.folded.len();
        let mut param_count: u32 = 0;
        if params > 0 {
            for (pos, inst) in stack.folded.iter().enumerate().rev() {
                param_count = param_count.saturating_add(inst.results);
                if param_count == params {
                    first_param = pos;
                    break;
                } else if param_count > params {
                    // unfoldable instruction
                    results = u32::MAX;
                    break;
                }
            }
        }

        let mut inst = FoldedInstruction {
            plain,
            folded: stack.folded.drain(first_param..).collect(),
            results,
            offset: self.operator_state.op_offset,
        };
        if let Some(hint) = self.branch_hint.take() {
            inst.folded.push(hint);
        }
        stack.folded.push(inst);

        Ok(())
    }

    // Print a folded instruction to the "real" printer. First print the "plain"
    // instruction, then recursively print each instruction that was folded in to the
    // foldedinst phrase.
    fn print(
        printer: &mut Printer,
        sep: &mut OperatorSeparator,
        inst: &FoldedInstruction,
    ) -> Result<()> {
        match sep {
            OperatorSeparator::Newline => printer.newline(inst.offset)?,
            OperatorSeparator::None => (),
            OperatorSeparator::NoneThenSpace => *sep = OperatorSeparator::Space,
            OperatorSeparator::Space => printer.result.write_str(" ")?,
        }

        printer.result.write_str("(")?;
        printer.result.write_str(&inst.plain)?;
        if inst.folded.is_empty() && inst.plain.contains(";;") {
            // Wasm line comment (e.g. label annotation) shouldn't comment out the closing parenthesis
            printer.newline(inst.offset)?;
        }
        printer.nesting += 1;
        for fi in &inst.folded {
            PrintOperatorFolded::print(printer, sep, &fi)?;
        }
        printer.nesting -= 1;
        printer.result.write_str(")")?;
        Ok(())
    }

    // The folding printer doesn't try to handle branch hints attached to blocks other than `if`.
    fn reject_branch_hint(&mut self) -> Result<()> {
        if self.branch_hint.is_some() {
            bail!("branch hints are only supported on an `if` or a plain instructions");
        }
        Ok(())
    }

    fn push_block(&mut self, ty: BlockType, kind: FrameKind, plain: String) -> Result<()> {
        self.reject_branch_hint()?;
        self.control.push(Block {
            ty,
            kind,
            plain,
            folded: Vec::new(),
            predicate: None,
            consequent: None,
            offset: self.operator_state.op_offset,
        });
        Ok(())
    }

    fn push_if(&mut self, ty: BlockType, plain: String) -> Result<()> {
        let mut predicate = Vec::new();
        if let Some(phrase) = self
            .control
            .last_mut()
            .ok_or_else(|| anyhow!("no enclosing block"))?
            .folded
            .pop()
        {
            predicate.push(phrase)
        }
        if let Some(hint) = self.branch_hint.take() {
            predicate.push(hint);
        }
        self.control.push(Block {
            ty,
            kind: FrameKind::If,
            plain,
            folded: Vec::new(),
            predicate: Some(predicate),
            consequent: None,
            offset: self.operator_state.op_offset,
        });
        Ok(())
    }

    fn handle_else(&mut self) -> Result<()> {
        self.reject_branch_hint()?;
        match self.control.pop() {
            Some(Block {
                ty,
                kind: FrameKind::If,
                plain,
                predicate,
                folded,
                offset,
                ..
            }) => self.control.push(Block {
                ty,
                kind: FrameKind::Else,
                plain,
                folded: Vec::new(),
                predicate,
                consequent: Some((folded, offset)),
                offset: self.operator_state.op_offset,
            }),
            _ => bail!("no enclosing if block"),
        }

        Ok(())
    }

    // The end instruction closes the current block and transforms it to the
    // corresponding form of foldedinst belonging to the parent block. This reuses/abuses
    // the "plain" nomenclature to also represent the opening delimiters
    // of block instructions and other block-like clauses (e.g. "then", "else").
    fn handle_end(&mut self, results: u32) -> Result<()> {
        self.reject_branch_hint()?;
        let frame = self.control.pop();
        let inst = match frame {
            Some(Block {
                kind: FrameKind::Block | FrameKind::Loop | FrameKind::TryTable,
                plain,
                folded,
                offset,
                ..
            }) => FoldedInstruction {
                plain,
                folded,
                results,
                offset,
            },
            Some(Block {
                kind: FrameKind::If,
                plain,
                folded,
                predicate: Some(predicate),
                offset,
                ..
            }) => {
                let then_clause = FoldedInstruction {
                    plain: String::from("then"),
                    folded,
                    results,
                    offset,
                };
                let mut folded = predicate;
                folded.push(then_clause);
                FoldedInstruction {
                    plain,
                    folded,
                    results,
                    offset,
                }
            }
            Some(Block {
                kind: FrameKind::Else,
                plain,
                folded,
                predicate: Some(predicate),
                consequent: Some((consequent, if_offset)),
                offset,
                ..
            }) => {
                let then_clause = FoldedInstruction {
                    plain: String::from("then"),
                    folded: consequent,
                    results,
                    offset: if_offset,
                };
                let else_clause = FoldedInstruction {
                    plain: String::from("else"),
                    folded,
                    results,
                    offset,
                };
                let mut folded = predicate;
                folded.push(then_clause);
                folded.push(else_clause);
                FoldedInstruction {
                    plain,
                    folded,
                    results,
                    offset: if_offset,
                }
            }
            _ => bail!("unhandled frame kind"),
        };

        self.control
            .last_mut()
            .ok_or_else(|| anyhow!("end without outer block"))?
            .folded
            .push(inst);
        Ok(())
    }
}
