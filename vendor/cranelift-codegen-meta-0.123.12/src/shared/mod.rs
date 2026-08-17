//! Shared definitions for the Cranelift intermediate language.

pub mod entities;
pub mod formats;
pub mod immediates;
pub mod instructions;
pub mod settings;
pub mod types;

use crate::cdsl::formats::{FormatStructure, InstructionFormat};
use crate::cdsl::instructions::AllInstructions;
use crate::cdsl::settings::SettingGroup;

use crate::shared::entities::EntityRefs;
use crate::shared::formats::Formats;
use crate::shared::immediates::Immediates;

use std::collections::HashMap;
use std::rc::Rc;

pub(crate) struct Definitions {
    pub settings: SettingGroup,
    pub all_instructions: AllInstructions,
    pub all_formats: Vec<Rc<InstructionFormat>>,
}

pub(crate) fn define() -> Definitions {
    let mut all_instructions = AllInstructions::new();

    let immediates = Immediates::new();
    let entities = EntityRefs::new();
    let formats = Formats::new(&immediates, &entities);
    instructions::define(&mut all_instructions, &formats, &immediates, &entities);
    let all_formats = verify_instruction_formats(&all_instructions);

    Definitions {
        settings: settings::define(),
        all_instructions,
        all_formats,
    }
}

/// Verifies certain properties of formats.
///
/// - Formats must be uniquely named: if two formats have the same name, they must refer to the
///   same data. Otherwise, two format variants in the codegen crate would have the same name.
/// - Formats must be structurally different from each other. Otherwise, this would lead to
///   code duplicate in the codegen crate.
///
/// Returns a list of all the instruction formats effectively used.
fn verify_instruction_formats(all_instructions: &AllInstructions) -> Vec<Rc<InstructionFormat>> {
    let mut format_names: HashMap<&'static str, &Rc<InstructionFormat>> = HashMap::new();

    // A structure is: number of input value operands / whether there's varargs or not / names
    // of immediate fields.
    let mut format_structures: HashMap<FormatStructure, Rc<InstructionFormat>> = HashMap::new();

    for inst in all_instructions {
        // Check name.
        if let Some(existing_format) = format_names.get(&inst.format.name) {
            assert!(
                Rc::ptr_eq(existing_format, &inst.format),
                "formats must uniquely named; there's a\
                     conflict on the name '{}', please make sure it is used only once.",
                existing_format.name
            );
        } else {
            format_names.insert(inst.format.name, &inst.format);
        }

        // Check structure.
        let key = inst.format.structure();
        if let Some(existing_format) = format_structures.get(&key) {
            assert_eq!(
                existing_format.name, inst.format.name,
                "duplicate instruction formats {} and {}; please remove one.",
                existing_format.name, inst.format.name
            );
        } else {
            format_structures.insert(key, inst.format.clone());
        }
    }

    let mut result = Vec::from_iter(format_structures.into_values());
    result.sort_by_key(|format| format.name);
    result
}
