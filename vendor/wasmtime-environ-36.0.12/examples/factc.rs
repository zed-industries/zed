use anyhow::{Context, Result, bail};
use clap::Parser;
use std::io::{IsTerminal, Write};
use std::path::{Path, PathBuf};
use wasmparser::{Validator, WasmFeatures};
use wasmtime_environ::{ScopeVec, Tunables, component::*};

/// A small helper utility to explore generated adapter modules from Wasmtime's
/// adapter fusion compiler.
///
/// This utility takes a `*.wat` file as input which is expected to be a valid
/// WebAssembly component. The component is parsed and any type definition for a
/// component function gets a generated adapter for it as if the caller/callee
/// used that type as the adapter.
///
/// For example with an input that looks like:
///
///     (component
///         (type (func (param u32) (result (list u8))))
///     )
///
/// This tool can be used to generate an adapter for that signature.
#[derive(Parser)]
struct Factc {
    /// Whether or not debug code is inserted into the generated adapter.
    #[arg(long)]
    debug: bool,

    /// Whether or not to skip validation of the generated adapter module.
    #[arg(long)]
    skip_validate: bool,

    /// Where to place the generated adapter module. Standard output is used if
    /// this is not specified.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output the text format for WebAssembly instead of the binary format.
    #[arg(short, long)]
    text: bool,

    /// The input component to generate adapters for.
    input: PathBuf,
}

fn main() -> Result<()> {
    Factc::parse().execute()
}

impl Factc {
    fn execute(self) -> Result<()> {
        env_logger::init();

        let input = wat::parse_file(&self.input)?;

        let tunables = Tunables::default_host();
        let mut validator =
            wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all());
        let mut component_types = ComponentTypesBuilder::new(&validator);
        let adapters = ScopeVec::new();

        Translator::new(&tunables, &mut validator, &mut component_types, &adapters)
            .translate(&input)?;

        let (out_name, mut out_file): (_, Box<dyn std::io::Write>) = match &self.output {
            Some(file) => (
                file.as_path(),
                Box::new(std::io::BufWriter::new(
                    std::fs::File::create(file)
                        .with_context(|| format!("failed to create {}", file.display()))?,
                )),
            ),
            None => (Path::new("stdout"), Box::new(std::io::stdout())),
        };

        for wasm in adapters.into_iter() {
            let output = if self.text {
                wasmprinter::print_bytes(&wasm)
                    .context("failed to convert binary wasm to text")?
                    .into_bytes()
            } else if self.output.is_none() && std::io::stdout().is_terminal() {
                bail!("cannot print binary wasm output to a terminal unless `-t` flag is passed")
            } else {
                wasm.to_vec()
            };

            out_file
                .write_all(&output)
                .with_context(|| format!("failed to write to {}", out_name.display()))?;

            if !self.skip_validate {
                Validator::new_with_features(WasmFeatures::default() | WasmFeatures::MEMORY64)
                    .validate_all(&wasm)
                    .context("failed to validate generated module")?;
            }
        }

        Ok(())
    }
}
