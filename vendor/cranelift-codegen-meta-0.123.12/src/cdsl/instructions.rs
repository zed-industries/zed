use std::fmt;
use std::rc::Rc;

use crate::cdsl::camel_case;
use crate::cdsl::formats::InstructionFormat;
use crate::cdsl::operands::Operand;
use crate::cdsl::typevar::TypeVar;

pub(crate) type AllInstructions = Vec<Instruction>;

pub(crate) struct InstructionGroupBuilder<'all_inst> {
    all_instructions: &'all_inst mut AllInstructions,
}

impl<'all_inst> InstructionGroupBuilder<'all_inst> {
    pub fn new(all_instructions: &'all_inst mut AllInstructions) -> Self {
        Self { all_instructions }
    }

    pub fn push(&mut self, builder: InstructionBuilder) {
        let inst = builder.build();
        self.all_instructions.push(inst);
    }
}

#[derive(Debug)]
pub(crate) struct PolymorphicInfo {
    pub use_typevar_operand: bool,
    pub ctrl_typevar: TypeVar,
}

#[derive(Debug)]
pub(crate) struct InstructionContent {
    /// Instruction mnemonic, also becomes opcode name.
    pub name: String,
    pub camel_name: String,

    /// Documentation string.
    pub doc: String,

    /// Input operands. This can be a mix of SSA value operands and other operand kinds.
    pub operands_in: Vec<Operand>,
    /// Output operands. The output operands must be SSA values or `variable_args`.
    pub operands_out: Vec<Operand>,

    /// Instruction format.
    pub format: Rc<InstructionFormat>,

    /// One of the input or output operands is a free type variable. None if the instruction is not
    /// polymorphic, set otherwise.
    pub polymorphic_info: Option<PolymorphicInfo>,

    /// Indices in operands_in of input operands that are values.
    pub value_opnums: Vec<usize>,
    /// Indices in operands_in of input operands that are immediates or entities.
    pub imm_opnums: Vec<usize>,
    /// Indices in operands_out of output operands that are values.
    pub value_results: Vec<usize>,

    /// True for instructions that terminate the block.
    pub is_terminator: bool,
    /// True for all branch or jump instructions.
    pub is_branch: bool,
    /// Is this a call instruction?
    pub is_call: bool,
    /// Is this a return instruction?
    pub is_return: bool,
    /// Can this instruction read from memory?
    pub can_load: bool,
    /// Can this instruction write to memory?
    pub can_store: bool,
    /// Can this instruction cause a trap?
    pub can_trap: bool,
    /// Does this instruction have other side effects besides can_* flags?
    pub other_side_effects: bool,
    /// Despite having other side effects, is this instruction okay to GVN?
    pub side_effects_idempotent: bool,
}

impl InstructionContent {
    pub fn snake_name(&self) -> &str {
        if &self.name == "return" {
            "return_"
        } else {
            &self.name
        }
    }
}

pub(crate) type Instruction = Rc<InstructionContent>;

impl fmt::Display for InstructionContent {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if !self.operands_out.is_empty() {
            let operands_out = self
                .operands_out
                .iter()
                .map(|op| op.name)
                .collect::<Vec<_>>()
                .join(", ");
            fmt.write_str(&operands_out)?;
            fmt.write_str(" = ")?;
        }

        fmt.write_str(&self.name)?;

        if !self.operands_in.is_empty() {
            let operands_in = self
                .operands_in
                .iter()
                .map(|op| op.name)
                .collect::<Vec<_>>()
                .join(", ");
            fmt.write_str(" ")?;
            fmt.write_str(&operands_in)?;
        }

        Ok(())
    }
}

pub(crate) struct InstructionBuilder {
    name: String,
    doc: String,
    format: Rc<InstructionFormat>,
    operands_in: Option<Vec<Operand>>,
    operands_out: Option<Vec<Operand>>,

    // See Instruction comments for the meaning of these fields.
    is_terminator: bool,
    is_branch: bool,
    is_call: bool,
    is_return: bool,
    can_load: bool,
    can_store: bool,
    can_trap: bool,
    other_side_effects: bool,
    side_effects_idempotent: bool,
}

impl InstructionBuilder {
    pub fn new<S: Into<String>>(name: S, doc: S, format: &Rc<InstructionFormat>) -> Self {
        Self {
            name: name.into(),
            doc: doc.into(),
            format: format.clone(),
            operands_in: None,
            operands_out: None,

            is_terminator: false,
            is_branch: false,
            is_call: false,
            is_return: false,
            can_load: false,
            can_store: false,
            can_trap: false,
            other_side_effects: false,
            side_effects_idempotent: false,
        }
    }

    pub fn operands_in(mut self, operands: Vec<Operand>) -> Self {
        assert!(self.operands_in.is_none());
        self.operands_in = Some(operands);
        self
    }

    pub fn operands_out(mut self, operands: Vec<Operand>) -> Self {
        assert!(self.operands_out.is_none());
        self.operands_out = Some(operands);
        self
    }

    /// Mark this instruction as a block terminator.
    pub fn terminates_block(mut self) -> Self {
        self.is_terminator = true;
        self
    }

    /// Mark this instruction as a branch instruction. This also implies that the instruction is a
    /// block terminator.
    pub fn branches(mut self) -> Self {
        self.is_branch = true;
        self.terminates_block()
    }

    /// Mark this instruction as a call instruction.
    pub fn call(mut self) -> Self {
        self.is_call = true;
        self
    }

    /// Mark this instruction as a return instruction. This also implies that the instruction is a
    /// block terminator.
    pub fn returns(mut self) -> Self {
        self.is_return = true;
        self.terminates_block()
    }

    /// Mark this instruction as one that can load from memory.
    pub fn can_load(mut self) -> Self {
        self.can_load = true;
        self
    }

    /// Mark this instruction as one that can store to memory.
    pub fn can_store(mut self) -> Self {
        self.can_store = true;
        self
    }

    /// Mark this instruction as possibly trapping.
    pub fn can_trap(mut self) -> Self {
        self.can_trap = true;
        self
    }

    /// Mark this instruction as one that has side-effects.
    pub fn other_side_effects(mut self) -> Self {
        self.other_side_effects = true;
        self
    }

    /// Mark this instruction as one whose side-effects may be de-duplicated.
    pub fn side_effects_idempotent(mut self) -> Self {
        self.side_effects_idempotent = true;
        self
    }

    fn build(self) -> Instruction {
        let operands_in = self.operands_in.unwrap_or_default();
        let operands_out = self.operands_out.unwrap_or_default();

        let mut value_opnums = Vec::new();
        let mut imm_opnums = Vec::new();
        for (i, op) in operands_in.iter().enumerate() {
            if op.is_value() {
                value_opnums.push(i);
            } else if op.is_immediate_or_entityref() {
                imm_opnums.push(i);
            } else {
                assert!(op.is_varargs());
            }
        }

        let value_results = operands_out
            .iter()
            .enumerate()
            .filter_map(|(i, op)| if op.is_value() { Some(i) } else { None })
            .collect();

        verify_format(&self.name, &operands_in, &self.format);

        let polymorphic_info =
            verify_polymorphic(&operands_in, &operands_out, &self.format, &value_opnums);

        let camel_name = camel_case(&self.name);

        Rc::new(InstructionContent {
            name: self.name,
            camel_name,
            doc: self.doc,
            operands_in,
            operands_out,
            format: self.format,
            polymorphic_info,
            value_opnums,
            value_results,
            imm_opnums,
            is_terminator: self.is_terminator,
            is_branch: self.is_branch,
            is_call: self.is_call,
            is_return: self.is_return,
            can_load: self.can_load,
            can_store: self.can_store,
            can_trap: self.can_trap,
            other_side_effects: self.other_side_effects,
            side_effects_idempotent: self.side_effects_idempotent,
        })
    }
}

/// Checks that the input operands actually match the given format.
fn verify_format(inst_name: &str, operands_in: &[Operand], format: &InstructionFormat) {
    // A format is defined by:
    // - its number of input value operands,
    // - its number and names of input immediate operands,
    // - whether it has a value list or not.
    let mut num_values = 0;
    let mut num_blocks = 0;
    let mut num_immediates = 0;

    for operand in operands_in.iter() {
        if operand.is_varargs() {
            assert!(
                format.has_value_list,
                "instruction {} has varargs, but its format {} doesn't have a value list; you may \
                 need to use a different format.",
                inst_name, format.name
            );
        }
        if operand.is_value() {
            num_values += 1;
        }
        if operand.kind.is_block() {
            num_blocks += 1;
        } else if operand.is_immediate_or_entityref() {
            if let Some(format_field) = format.imm_fields.get(num_immediates) {
                assert_eq!(
                    format_field.kind.rust_field_name,
                    operand.kind.rust_field_name,
                    "{}th operand of {} should be {} (according to format), not {} (according to \
                     inst definition). You may need to use a different format.",
                    num_immediates,
                    inst_name,
                    format_field.kind.rust_field_name,
                    operand.kind.rust_field_name
                );
                num_immediates += 1;
            }
        }
    }

    assert_eq!(
        num_values, format.num_value_operands,
        "inst {} doesn't have as many value input operands as its format {} declares; you may need \
         to use a different format.",
        inst_name, format.name
    );

    assert_eq!(
        num_blocks, format.num_block_operands,
        "inst {} doesn't have as many block input operands as its format {} declares; you may need \
        to use a different format.",
        inst_name, format.name,
    );

    assert_eq!(
        num_immediates,
        format.imm_fields.len(),
        "inst {} doesn't have as many immediate input \
         operands as its format {} declares; you may need to use a different format.",
        inst_name,
        format.name
    );
}

/// Check if this instruction is polymorphic, and verify its use of type variables.
fn verify_polymorphic(
    operands_in: &[Operand],
    operands_out: &[Operand],
    format: &InstructionFormat,
    value_opnums: &[usize],
) -> Option<PolymorphicInfo> {
    // The instruction is polymorphic if it has one free input or output operand.
    let is_polymorphic = operands_in
        .iter()
        .any(|op| op.is_value() && op.type_var().unwrap().free_typevar().is_some())
        || operands_out
            .iter()
            .any(|op| op.is_value() && op.type_var().unwrap().free_typevar().is_some());

    if !is_polymorphic {
        return None;
    }

    // Verify the use of type variables.
    let tv_op = format.typevar_operand;
    let mut maybe_error_message = None;
    if let Some(tv_op) = tv_op {
        if tv_op < value_opnums.len() {
            let op_num = value_opnums[tv_op];
            let tv = operands_in[op_num].type_var().unwrap();
            let free_typevar = tv.free_typevar();
            if (free_typevar.is_some() && tv == &free_typevar.unwrap())
                || tv.singleton_type().is_some()
            {
                match is_ctrl_typevar_candidate(tv, operands_in, operands_out) {
                    Ok(_other_typevars) => {
                        return Some(PolymorphicInfo {
                            use_typevar_operand: true,
                            ctrl_typevar: tv.clone(),
                        });
                    }
                    Err(error_message) => {
                        maybe_error_message = Some(error_message);
                    }
                }
            }
        }
    };

    // If we reached here, it means the type variable indicated as the typevar operand couldn't
    // control every other input and output type variable. We need to look at the result type
    // variables.
    if operands_out.is_empty() {
        // No result means no other possible type variable, so it's a type inference failure.
        match maybe_error_message {
            Some(msg) => panic!("{}", msg),
            None => panic!("typevar_operand must be a free type variable"),
        }
    }

    // Otherwise, try to infer the controlling type variable by looking at the first result.
    let tv = operands_out[0].type_var().unwrap();
    let free_typevar = tv.free_typevar();
    if free_typevar.is_some() && tv != &free_typevar.unwrap() {
        panic!("first result must be a free type variable");
    }

    // At this point, if the next unwrap() fails, it means the output type couldn't be used as a
    // controlling type variable either; panicking is the right behavior.
    is_ctrl_typevar_candidate(tv, operands_in, operands_out).unwrap();

    Some(PolymorphicInfo {
        use_typevar_operand: false,
        ctrl_typevar: tv.clone(),
    })
}

/// Verify that the use of TypeVars is consistent with `ctrl_typevar` as the controlling type
/// variable.
///
/// All polymorphic inputs must either be derived from `ctrl_typevar` or be independent free type
/// variables only used once.
///
/// All polymorphic results must be derived from `ctrl_typevar`.
///
/// Return a vector of other type variables used, or a string explaining what went wrong.
fn is_ctrl_typevar_candidate(
    ctrl_typevar: &TypeVar,
    operands_in: &[Operand],
    operands_out: &[Operand],
) -> Result<Vec<TypeVar>, String> {
    let mut other_typevars = Vec::new();

    // Check value inputs.
    for input in operands_in {
        if !input.is_value() {
            continue;
        }

        let typ = input.type_var().unwrap();
        let free_typevar = typ.free_typevar();

        // Non-polymorphic or derived from ctrl_typevar is OK.
        if free_typevar.is_none() {
            continue;
        }
        let free_typevar = free_typevar.unwrap();
        if &free_typevar == ctrl_typevar {
            continue;
        }

        // No other derived typevars allowed.
        if typ != &free_typevar {
            return Err(format!(
                "{:?}: type variable {} must be derived from {:?} while it is derived from {:?}",
                input, typ.name, ctrl_typevar, free_typevar
            ));
        }

        // Other free type variables can only be used once each.
        for other_tv in &other_typevars {
            if &free_typevar == other_tv {
                return Err(format!(
                    "non-controlling type variable {} can't be used more than once",
                    free_typevar.name
                ));
            }
        }

        other_typevars.push(free_typevar);
    }

    // Check outputs.
    for result in operands_out {
        if !result.is_value() {
            continue;
        }

        let typ = result.type_var().unwrap();
        let free_typevar = typ.free_typevar();

        // Non-polymorphic or derived from ctrl_typevar is OK.
        if free_typevar.is_none() || &free_typevar.unwrap() == ctrl_typevar {
            continue;
        }

        return Err("type variable in output not derived from ctrl_typevar".into());
    }

    Ok(other_typevars)
}
