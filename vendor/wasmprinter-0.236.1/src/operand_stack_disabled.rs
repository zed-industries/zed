use wasmparser::{BinaryReader, OperatorsReader, Payload, Result};

pub enum Validator {}

impl Validator {
    pub fn new() -> Option<Validator> {
        None
    }

    pub fn payload(&mut self, _payload: &Payload<'_>) -> Result<()> {
        match *self {}
    }

    pub fn next_func(&mut self) -> Option<FuncValidator> {
        match *self {}
    }
}

pub enum FuncValidator {}

impl FuncValidator {
    pub fn read_locals(&mut self, _reader: BinaryReader<'_>) -> Result<()> {
        match *self {}
    }

    pub fn visit_operator(&mut self, _reader: &OperatorsReader<'_>, _is_end: bool) -> Result<()> {
        match *self {}
    }

    pub fn visualize_operand_stack(&self, _use_color: bool) -> anyhow::Result<String> {
        match *self {}
    }
}
