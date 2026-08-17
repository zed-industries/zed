//! Utility routines for pretty-printing error messages.

use crate::entity::SecondaryMap;
use crate::ir;
use crate::ir::entities::{AnyEntity, Block, Inst, Value};
use crate::ir::function::Function;
use crate::ir::pcc::Fact;
use crate::result::CodegenError;
use crate::verifier::{VerifierError, VerifierErrors};
use crate::write::{FuncWriter, PlainWriter, decorate_function};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use core::fmt::Write;

/// Pretty-print a verifier error.
pub fn pretty_verifier_error<'a>(
    func: &ir::Function,
    func_w: Option<Box<dyn FuncWriter + 'a>>,
    errors: VerifierErrors,
) -> String {
    let mut errors = errors.0;
    let mut w = String::new();
    let num_errors = errors.len();

    decorate_function(
        &mut PrettyVerifierError(func_w.unwrap_or_else(|| Box::new(PlainWriter)), &mut errors),
        &mut w,
        func,
    )
    .unwrap();

    writeln!(
        w,
        "\n; {} verifier error{} detected (see above). Compilation aborted.",
        num_errors,
        if num_errors == 1 { "" } else { "s" }
    )
    .unwrap();

    w
}

struct PrettyVerifierError<'a>(Box<dyn FuncWriter + 'a>, &'a mut Vec<VerifierError>);

impl<'a> FuncWriter for PrettyVerifierError<'a> {
    fn write_block_header(
        &mut self,
        w: &mut dyn Write,
        func: &Function,
        block: Block,
        indent: usize,
    ) -> fmt::Result {
        pretty_block_header_error(w, func, block, indent, &mut *self.0, self.1)
    }

    fn write_instruction(
        &mut self,
        w: &mut dyn Write,
        func: &Function,
        aliases: &SecondaryMap<Value, Vec<Value>>,
        inst: Inst,
        indent: usize,
    ) -> fmt::Result {
        pretty_instruction_error(w, func, aliases, inst, indent, &mut *self.0, self.1)
    }

    fn write_entity_definition(
        &mut self,
        w: &mut dyn Write,
        func: &Function,
        entity: AnyEntity,
        value: &dyn fmt::Display,
        maybe_fact: Option<&Fact>,
    ) -> fmt::Result {
        pretty_preamble_error(w, func, entity, value, maybe_fact, &mut *self.0, self.1)
    }
}

/// Pretty-print a function verifier error for a given block.
fn pretty_block_header_error(
    w: &mut dyn Write,
    func: &Function,
    cur_block: Block,
    indent: usize,
    func_w: &mut dyn FuncWriter,
    errors: &mut Vec<VerifierError>,
) -> fmt::Result {
    let mut s = String::new();
    func_w.write_block_header(&mut s, func, cur_block, indent)?;
    write!(w, "{s}")?;

    // TODO: Use drain_filter here when it gets stabilized
    let mut i = 0;
    let mut printed_error = false;
    while i != errors.len() {
        match errors[i].location {
            ir::entities::AnyEntity::Block(block) if block == cur_block => {
                if !printed_error {
                    print_arrow(w, &s)?;
                    printed_error = true;
                }
                let err = errors.remove(i);
                print_error(w, err)?;
            }
            _ => i += 1,
        }
    }

    if printed_error {
        w.write_char('\n')?;
    }

    Ok(())
}

/// Pretty-print a function verifier error for a given instruction.
fn pretty_instruction_error(
    w: &mut dyn Write,
    func: &Function,
    aliases: &SecondaryMap<Value, Vec<Value>>,
    cur_inst: Inst,
    indent: usize,
    func_w: &mut dyn FuncWriter,
    errors: &mut Vec<VerifierError>,
) -> fmt::Result {
    let mut s = String::new();
    func_w.write_instruction(&mut s, func, aliases, cur_inst, indent)?;
    write!(w, "{s}")?;

    // TODO: Use drain_filter here when it gets stabilized
    let mut i = 0;
    let mut printed_error = false;
    while i != errors.len() {
        match errors[i].location {
            ir::entities::AnyEntity::Inst(inst) if inst == cur_inst => {
                if !printed_error {
                    print_arrow(w, &s)?;
                    printed_error = true;
                }
                let err = errors.remove(i);
                print_error(w, err)?;
            }
            _ => i += 1,
        }
    }

    if printed_error {
        w.write_char('\n')?;
    }

    Ok(())
}

fn pretty_preamble_error(
    w: &mut dyn Write,
    func: &Function,
    entity: AnyEntity,
    value: &dyn fmt::Display,
    maybe_fact: Option<&Fact>,
    func_w: &mut dyn FuncWriter,
    errors: &mut Vec<VerifierError>,
) -> fmt::Result {
    let mut s = String::new();
    func_w.write_entity_definition(&mut s, func, entity, value, maybe_fact)?;
    write!(w, "{s}")?;

    // TODO: Use drain_filter here when it gets stabilized
    let mut i = 0;
    let mut printed_error = false;
    while i != errors.len() {
        if entity == errors[i].location {
            if !printed_error {
                print_arrow(w, &s)?;
                printed_error = true;
            }
            let err = errors.remove(i);
            print_error(w, err)?;
        } else {
            i += 1
        }
    }

    if printed_error {
        w.write_char('\n')?;
    }

    Ok(())
}

/// Prints:
///    ;   ^~~~~~
fn print_arrow(w: &mut dyn Write, entity: &str) -> fmt::Result {
    write!(w, ";")?;

    let indent = entity.len() - entity.trim_start().len();
    if indent != 0 {
        write!(w, "{1:0$}^", indent - 1, "")?;
    }

    for _ in 0..entity.trim().len() - 1 {
        write!(w, "~")?;
    }

    writeln!(w)
}

/// Prints:
///    ; error: [ERROR BODY]
fn print_error(w: &mut dyn Write, err: VerifierError) -> fmt::Result {
    writeln!(w, "; error: {}", err.to_string())?;
    Ok(())
}

/// Pretty-print a Cranelift error.
pub fn pretty_error(func: &ir::Function, err: CodegenError) -> String {
    if let CodegenError::Verifier(e) = err {
        pretty_verifier_error(func, None, e)
    } else {
        err.to_string()
    }
}
