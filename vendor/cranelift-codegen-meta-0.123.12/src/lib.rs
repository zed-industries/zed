//! This crate generates Rust sources for use by
//! [`cranelift_codegen`](../cranelift_codegen/index.html).

use cranelift_srcgen::{Formatter, Language, error};
use shared::Definitions;

#[macro_use]
mod cdsl;

pub mod isa;
pub mod isle;

mod gen_asm;
mod gen_inst;
mod gen_isle;
mod gen_settings;
mod gen_types;

mod constant_hash;
mod shared;
mod unique_table;

#[cfg(feature = "pulley")]
mod pulley;

/// Generate an ISA from an architecture string (e.g. "x86_64").
pub fn isa_from_arch(arch: &str) -> Result<isa::Isa, String> {
    isa::Isa::from_arch(arch).ok_or_else(|| format!("no supported isa found for arch `{arch}`"))
}

/// Generates all the Rust source files used in Cranelift from the meta-language.
pub fn generate_rust(isas: &[isa::Isa], out_dir: &std::path::Path) -> Result<(), error::Error> {
    let shared_defs = shared::define();
    generate_rust_for_shared_defs(&shared_defs, isas, out_dir)
}

fn generate_rust_for_shared_defs(
    shared_defs: &Definitions,
    isas: &[isa::Isa],
    out_dir: &std::path::Path,
) -> Result<(), error::Error> {
    gen_settings::generate(
        &shared_defs.settings,
        gen_settings::ParentGroup::None,
        "settings.rs",
        out_dir,
    )?;

    gen_types::generate("types.rs", out_dir)?;

    gen_inst::generate(
        &shared_defs.all_formats,
        &shared_defs.all_instructions,
        "opcodes.rs",
        "inst_builder.rs",
        out_dir,
    )?;

    // Per ISA definitions.
    for isa in isa::define(isas) {
        gen_settings::generate(
            &isa.settings,
            gen_settings::ParentGroup::Shared,
            &format!("settings-{}.rs", isa.name),
            out_dir,
        )?;
    }

    #[cfg(feature = "pulley")]
    if isas.contains(&isa::Isa::Pulley32) || isas.contains(&isa::Isa::Pulley64) {
        pulley::generate_rust("pulley_inst_gen.rs", out_dir)?;
    }

    Ok(())
}

/// Generates all the ISLE source files used in Cranelift from the meta-language.
pub fn generate_isle(isle_dir: &std::path::Path) -> Result<(), error::Error> {
    let shared_defs = shared::define();
    generate_isle_for_shared_defs(&shared_defs, isle_dir)
}

fn generate_isle_for_shared_defs(
    shared_defs: &Definitions,
    isle_dir: &std::path::Path,
) -> Result<(), error::Error> {
    gen_isle::generate(
        &shared_defs.all_formats,
        &shared_defs.all_instructions,
        "numerics.isle",
        "isle_numerics.rs",
        "clif_opt.isle",
        "clif_lower.isle",
        isle_dir,
    )?;

    #[cfg(feature = "pulley")]
    pulley::generate_isle("pulley_gen.isle", isle_dir)?;

    Ok(())
}

/// Generate the ISLE definitions; this provides ISLE glue to access the
/// assembler instructions in [cranelift_assembler_x64_meta].
fn generate_isle_for_assembler(
    insts: &[cranelift_assembler_x64_meta::dsl::Inst],
    isle_dir: &std::path::Path,
) -> Result<(), error::Error> {
    let mut fmt = Formatter::new(Language::Isle);
    gen_asm::generate_isle(&mut fmt, insts);
    fmt.write("assembler.isle", isle_dir)
}

/// Generate a macro containing builder functions for the assembler's ISLE
/// constructors; this provides Rust implementations backing up the ISLE
/// definitions in [generate_isle_for_assembler].
fn generate_rust_macro_for_assembler(
    insts: &[cranelift_assembler_x64_meta::dsl::Inst],
    out_dir: &std::path::Path,
) -> Result<(), error::Error> {
    let mut fmt = Formatter::new(Language::Rust);
    gen_asm::generate_rust_macro(&mut fmt, insts);
    fmt.write("assembler-isle-macro.rs", out_dir)
}

/// Generates all the source files used in Cranelift from the meta-language.
pub fn generate(
    isas: &[isa::Isa],
    out_dir: &std::path::Path,
    isle_dir: &std::path::Path,
) -> Result<(), error::Error> {
    let shared_defs = shared::define();
    generate_rust_for_shared_defs(&shared_defs, isas, out_dir)?;
    generate_isle_for_shared_defs(&shared_defs, isle_dir)?;

    let insts = cranelift_assembler_x64_meta::instructions::list();
    generate_isle_for_assembler(&insts, isle_dir)?;
    generate_rust_macro_for_assembler(&insts, out_dir)?;

    Ok(())
}
