//! A crate to convert a WebAssembly binary to its textual representation in the
//! WebAssembly Text Format (WAT).
//!
//! This crate is intended for developer toolchains and debugging, supporting
//! human-readable versions of a wasm binary. This can also be useful when
//! developing wasm toolchain support in Rust for various purposes like testing
//! and debugging and such.

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use anyhow::{Context, Result, anyhow, bail};
use operator::{OpPrinter, OperatorSeparator, OperatorState, PrintOperator, PrintOperatorFolded};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io;
use std::marker;
use std::mem;
use std::path::Path;
use wasmparser::*;

const MAX_LOCALS: u32 = 50000;
const MAX_NESTING_TO_PRINT: u32 = 50;
const MAX_WASM_FUNCTIONS: u32 = 1_000_000;
const MAX_WASM_FUNCTION_SIZE: u32 = 128 * 1024;

#[cfg(feature = "component-model")]
mod component;
#[cfg(feature = "validate")]
mod operand_stack;
#[cfg(not(feature = "validate"))]
mod operand_stack_disabled;
#[cfg(not(feature = "validate"))]
use operand_stack_disabled as operand_stack;
mod operator;
mod print;

pub use self::print::*;

/// Reads a WebAssembly `file` from the filesystem and then prints it into an
/// in-memory `String`.
pub fn print_file(file: impl AsRef<Path>) -> Result<String> {
    let file = file.as_ref();
    let contents = std::fs::read(file).context(format!("failed to read `{}`", file.display()))?;
    print_bytes(contents)
}

/// Prints an in-memory `wasm` binary blob into an in-memory `String` which is
/// its textual representation.
pub fn print_bytes(wasm: impl AsRef<[u8]>) -> Result<String> {
    let mut dst = String::new();
    Config::new().print(wasm.as_ref(), &mut PrintFmtWrite(&mut dst))?;
    Ok(dst)
}

/// Configuration used to print a WebAssembly binary.
///
/// This structure is used to control the overal structure of how wasm binaries
/// are printed and tweaks various ways that configures the output.
#[derive(Debug)]
pub struct Config {
    print_offsets: bool,
    print_skeleton: bool,
    name_unnamed: bool,
    fold_instructions: bool,
    indent_text: String,
    print_operand_stack: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            print_offsets: false,
            print_skeleton: false,
            name_unnamed: false,
            fold_instructions: false,
            indent_text: "  ".to_string(),
            print_operand_stack: false,
        }
    }
}

/// This structure is the actual structure that prints WebAssembly binaries.
struct Printer<'cfg, 'env> {
    config: &'cfg Config,
    result: &'cfg mut (dyn Print + 'env),
    nesting: u32,
    line: usize,
    group_lines: Vec<usize>,
    code_section_hints: Vec<(u32, Vec<(usize, BranchHint)>)>,
}

#[derive(Default)]
struct CoreState {
    types: Vec<Option<SubType>>,
    funcs: u32,
    func_to_type: Vec<Option<u32>>,
    memories: u32,
    tags: u32,
    tag_to_type: Vec<Option<u32>>,
    globals: u32,
    tables: u32,
    #[cfg(feature = "component-model")]
    modules: u32,
    #[cfg(feature = "component-model")]
    instances: u32,
    func_names: NamingMap<u32, NameFunc>,
    local_names: NamingMap<(u32, u32), NameLocal>,
    label_names: NamingMap<(u32, u32), NameLabel>,
    type_names: NamingMap<u32, NameType>,
    field_names: NamingMap<(u32, u32), NameField>,
    tag_names: NamingMap<u32, NameTag>,
    table_names: NamingMap<u32, NameTable>,
    memory_names: NamingMap<u32, NameMemory>,
    global_names: NamingMap<u32, NameGlobal>,
    element_names: NamingMap<u32, NameElem>,
    data_names: NamingMap<u32, NameData>,
    #[cfg(feature = "component-model")]
    module_names: NamingMap<u32, NameModule>,
    #[cfg(feature = "component-model")]
    instance_names: NamingMap<u32, NameInstance>,
}

/// A map of index-to-name for tracking what are the contents of the name
/// section.
///
/// The type parameter `T` is either `u32` for most index-based maps or a `(u32,
/// u32)` for label/local maps where there are two levels of indices.
///
/// The type parameter `K` is a static description/namespace for what kind of
/// item is contained within this map. That's used by some helper methods to
/// synthesize reasonable names automatically.
struct NamingMap<T, K> {
    index_to_name: HashMap<T, Naming>,
    _marker: marker::PhantomData<K>,
}

impl<T, K> Default for NamingMap<T, K> {
    fn default() -> NamingMap<T, K> {
        NamingMap {
            index_to_name: HashMap::new(),
            _marker: marker::PhantomData,
        }
    }
}

#[derive(Default)]
#[cfg(feature = "component-model")]
struct ComponentState {
    types: u32,
    funcs: u32,
    instances: u32,
    components: u32,
    values: u32,
    type_names: NamingMap<u32, NameType>,
    func_names: NamingMap<u32, NameFunc>,
    component_names: NamingMap<u32, NameComponent>,
    instance_names: NamingMap<u32, NameInstance>,
    value_names: NamingMap<u32, NameValue>,
}

struct State {
    encoding: Encoding,
    name: Option<Naming>,
    core: CoreState,
    #[cfg(feature = "component-model")]
    component: ComponentState,
    custom_section_place: Option<(&'static str, usize)>,
    // `custom_section_place` stores the text representation of the location where
    // a custom section should be serialized in the binary format.
    // The tuple elements are a str (e.g. "after elem") and the line number
    // where the custom section place was set. `update_custom_section_place` won't
    // update the custom section place unless the line number changes; this prevents
    // printing a place "after xxx" where the xxx section doesn't appear in the text format
    // (e.g. because it was present but empty in the binary format).
}

impl State {
    fn new(encoding: Encoding) -> Self {
        Self {
            encoding,
            name: None,
            core: CoreState::default(),
            #[cfg(feature = "component-model")]
            component: ComponentState::default(),
            custom_section_place: None,
        }
    }
}

struct Naming {
    name: String,
    kind: NamingKind,
}

enum NamingKind {
    DollarName,
    DollarQuotedName,
    SyntheticPrefix(String),
}

impl Config {
    /// Creates a new [`Config`] object that's ready to start printing wasm
    /// binaries to strings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether or not to print binary offsets of each item as comments in the
    /// text format whenever a newline is printed.
    pub fn print_offsets(&mut self, print: bool) -> &mut Self {
        self.print_offsets = print;
        self
    }

    /// Whether or not to print only a "skeleton" which skips function bodies,
    /// data segment contents, element segment contents, etc.
    pub fn print_skeleton(&mut self, print: bool) -> &mut Self {
        self.print_skeleton = print;
        self
    }

    /// Assign names to all unnamed items.
    ///
    /// If enabled then any previously unnamed item will have a name synthesized
    /// that looks like `$#func10` for example. The leading `#` indicates that
    /// it's `wasmprinter`-generated. The `func` is the namespace of the name
    /// and provides extra context about the item when referenced. The 10 is the
    /// local index of the item.
    ///
    /// Note that if the resulting text output is converted back to binary the
    /// resulting `name` custom section will not be the same as before.
    pub fn name_unnamed(&mut self, enable: bool) -> &mut Self {
        self.name_unnamed = enable;
        self
    }

    /// Print instructions in folded form where possible.
    ///
    /// This will cause printing to favor the s-expression (parenthesized) form
    /// of WebAssembly instructions. For example this output would be generated
    /// for a simple `add` function:
    ///
    /// ```wasm
    /// (module
    ///     (func $foo (param i32 i32) (result i32)
    ///         (i32.add
    ///             (local.get 0)
    ///             (local.get 1))
    ///     )
    /// )
    /// ```
    pub fn fold_instructions(&mut self, enable: bool) -> &mut Self {
        self.fold_instructions = enable;
        self
    }

    /// Print the operand stack types within function bodies,
    /// flagging newly pushed operands when color output is enabled. E.g.:
    ///
    /// ```wasm
    /// (module
    ///   (type (;0;) (func))
    ///   (func (;0;) (type 0)
    ///     i32.const 4
    ///     ;; [i32]
    ///     i32.const 5
    ///     ;; [i32 i32]
    ///     i32.add
    ///     ;; [i32]
    ///     drop
    ///     ;; []
    ///   )
    /// )
    /// ```
    #[cfg(feature = "validate")]
    pub fn print_operand_stack(&mut self, enable: bool) -> &mut Self {
        self.print_operand_stack = enable;
        self
    }

    /// Select the string to use when indenting.
    ///
    /// The indent allowed here are arbitrary and unchecked. You should enter
    /// blank text like `" "` or `"\t"`, rather than something like `"(;;)"`.
    ///
    /// The default setting is double spaces `" "`
    pub fn indent_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.indent_text = text.into();
        self
    }

    /// Print a WebAssembly binary.
    ///
    /// This function takes an entire `wasm` binary blob and prints it to the
    /// `result` in the WebAssembly Text Format.
    pub fn print(&self, wasm: &[u8], result: &mut impl Print) -> Result<()> {
        Printer {
            config: self,
            result,
            code_section_hints: Vec::new(),
            group_lines: Vec::new(),
            line: 0,
            nesting: 0,
        }
        .print_contents(wasm)
    }

    /// Get the line-by-line WAT disassembly for the given Wasm, along with the
    /// binary offsets for each line.
    pub fn offsets_and_lines<'a>(
        &self,
        wasm: &[u8],
        storage: &'a mut String,
    ) -> Result<impl Iterator<Item = (Option<usize>, &'a str)> + 'a> {
        struct TrackingPrint<'a> {
            dst: &'a mut String,
            lines: Vec<usize>,
            line_offsets: Vec<Option<usize>>,
        }

        impl Print for TrackingPrint<'_> {
            fn write_str(&mut self, s: &str) -> io::Result<()> {
                self.dst.push_str(s);
                Ok(())
            }
            fn start_line(&mut self, offset: Option<usize>) {
                self.lines.push(self.dst.len());
                self.line_offsets.push(offset);
            }
        }

        let mut output = TrackingPrint {
            dst: storage,
            lines: Vec::new(),
            line_offsets: Vec::new(),
        };
        self.print(wasm, &mut output)?;

        let TrackingPrint {
            dst,
            lines,
            line_offsets,
        } = output;
        let end = dst.len();
        let dst = &dst[..];
        let mut offsets = line_offsets.into_iter();
        let mut lines = lines.into_iter().peekable();

        Ok(std::iter::from_fn(move || {
            let offset = offsets.next()?;
            let i = lines.next()?;
            let j = lines.peek().copied().unwrap_or(end);
            let line = &dst[i..j];
            Some((offset, line))
        }))
    }
}

impl Printer<'_, '_> {
    fn read_names<'a>(
        &mut self,
        mut bytes: &'a [u8],
        mut parser: Parser,
        state: &mut State,
    ) -> Result<()> {
        loop {
            let payload = match parser.parse(bytes, true)? {
                Chunk::NeedMoreData(_) => unreachable!(),
                Chunk::Parsed { payload, consumed } => {
                    bytes = &bytes[consumed..];
                    payload
                }
            };

            match payload {
                Payload::CodeSectionStart { size, .. } => {
                    if size as usize > bytes.len() {
                        bail!("invalid code section size");
                    }
                    bytes = &bytes[size as usize..];
                    parser.skip_section();
                }
                #[cfg(feature = "component-model")]
                Payload::ModuleSection {
                    unchecked_range: range,
                    ..
                }
                | Payload::ComponentSection {
                    unchecked_range: range,
                    ..
                } => {
                    let offset = range.end - range.start;
                    if offset > bytes.len() {
                        bail!("invalid module or component section range");
                    }
                    bytes = &bytes[offset..];
                }

                Payload::CustomSection(c) => {
                    // Ignore any error associated with the name sections.
                    match c.as_known() {
                        KnownCustom::Name(reader) => {
                            drop(self.register_names(state, reader));
                        }
                        #[cfg(feature = "component-model")]
                        KnownCustom::ComponentName(reader) => {
                            drop(self.register_component_names(state, reader));
                        }
                        KnownCustom::BranchHints(reader) => {
                            drop(self.register_branch_hint_section(reader));
                        }
                        _ => {}
                    }
                }

                Payload::End(_) => break,
                _ => {}
            }
        }

        Ok(())
    }

    fn ensure_module(states: &[State]) -> Result<()> {
        if !matches!(states.last().unwrap().encoding, Encoding::Module) {
            bail!("a module section was encountered when parsing a component");
        }

        Ok(())
    }

    #[cfg(feature = "component-model")]
    fn ensure_component(states: &[State]) -> Result<()> {
        if !matches!(states.last().unwrap().encoding, Encoding::Component) {
            bail!("a component section was encountered when parsing a module");
        }

        Ok(())
    }

    fn print_contents(&mut self, mut bytes: &[u8]) -> Result<()> {
        self.result.start_line(Some(0));

        let mut expected = None;
        let mut states: Vec<State> = Vec::new();
        let mut parser = Parser::new(0);
        #[cfg(feature = "component-model")]
        let mut parsers = Vec::new();

        let mut validator = if self.config.print_operand_stack {
            operand_stack::Validator::new()
        } else {
            None
        };

        loop {
            let payload = match parser.parse(bytes, true)? {
                Chunk::NeedMoreData(_) => unreachable!(),
                Chunk::Parsed { payload, consumed } => {
                    bytes = &bytes[consumed..];
                    payload
                }
            };
            if let Some(validator) = &mut validator {
                match validator.payload(&payload) {
                    Ok(()) => {}
                    Err(e) => {
                        self.newline_unknown_pos()?;
                        write!(self.result, ";; module or component is invalid: {e}")?;
                    }
                }
            }
            match payload {
                Payload::Version { encoding, .. } => {
                    if let Some(e) = expected {
                        if encoding != e {
                            bail!("incorrect encoding for nested module or component");
                        }
                        expected = None;
                    }

                    assert!(states.last().map(|s| s.encoding) != Some(Encoding::Module));

                    match encoding {
                        Encoding::Module => {
                            states.push(State::new(Encoding::Module));
                            states.last_mut().unwrap().custom_section_place =
                                Some(("before first", self.line));
                            if states.len() > 1 {
                                self.start_group("core module")?;
                            } else {
                                self.start_group("module")?;
                            }

                            #[cfg(feature = "component-model")]
                            if states.len() > 1 {
                                let parent = &states[states.len() - 2];
                                self.result.write_str(" ")?;
                                self.print_name(&parent.core.module_names, parent.core.modules)?;
                            }
                        }
                        Encoding::Component => {
                            #[cfg(feature = "component-model")]
                            {
                                states.push(State::new(Encoding::Component));
                                self.start_group("component")?;

                                if states.len() > 1 {
                                    let parent = &states[states.len() - 2];
                                    self.result.write_str(" ")?;
                                    self.print_name(
                                        &parent.component.component_names,
                                        parent.component.components,
                                    )?;
                                }
                            }
                            #[cfg(not(feature = "component-model"))]
                            {
                                bail!(
                                    "support for printing components disabled \
                                     at compile-time"
                                );
                            }
                        }
                    }

                    let len = states.len();
                    let state = states.last_mut().unwrap();

                    // First up try to find the `name` subsection which we'll use to print
                    // pretty names everywhere.
                    self.read_names(bytes, parser.clone(), state)?;

                    if len == 1 {
                        if let Some(name) = state.name.as_ref() {
                            self.result.write_str(" ")?;
                            name.write(self)?;
                        }
                    }
                }
                Payload::CustomSection(c) => {
                    // If the custom printing trait handles this section, keep
                    // going after that.
                    let printed =
                        self.result
                            .print_custom_section(c.name(), c.data_offset(), c.data())?;
                    if printed {
                        self.update_custom_section_line(&mut states);
                        continue;
                    }

                    // If this wasn't handled specifically above then try to
                    // print the known custom builtin sections. If this fails
                    // because the custom section is malformed then print the
                    // raw contents instead.
                    let state = states.last().unwrap();
                    let start = self.nesting;
                    match c.as_known() {
                        KnownCustom::Unknown => self.print_raw_custom_section(state, c.clone())?,
                        _ => {
                            match (Printer {
                                config: self.config,
                                result: &mut PrintFmtWrite(String::new()),
                                nesting: 0,
                                line: 0,
                                group_lines: Vec::new(),
                                code_section_hints: Vec::new(),
                            })
                            .print_known_custom_section(c.clone())
                            {
                                Ok(true) => {
                                    self.print_known_custom_section(c.clone())?;
                                }
                                Ok(false) => self.print_raw_custom_section(state, c.clone())?,
                                Err(e) if !e.is::<BinaryReaderError>() => return Err(e),
                                Err(e) => {
                                    let msg = format!(
                                        "failed to parse custom section `{}`: {e}",
                                        c.name()
                                    );
                                    for line in msg.lines() {
                                        self.newline(c.data_offset())?;
                                        write!(self.result, ";; {line}")?;
                                    }
                                    self.print_raw_custom_section(state, c.clone())?
                                }
                            }
                        }
                    }
                    assert!(self.nesting == start);
                    self.update_custom_section_line(&mut states);
                }
                Payload::TypeSection(s) => {
                    self.print_types(states.last_mut().unwrap(), s)?;
                    self.update_custom_section_place(&mut states, "after type");
                }
                Payload::ImportSection(s) => {
                    Self::ensure_module(&states)?;
                    self.print_imports(states.last_mut().unwrap(), s)?;
                    self.update_custom_section_place(&mut states, "after import");
                }
                Payload::FunctionSection(reader) => {
                    Self::ensure_module(&states)?;
                    if reader.count() > MAX_WASM_FUNCTIONS {
                        bail!(
                            "module contains {} functions which exceeds the limit of {}",
                            reader.count(),
                            MAX_WASM_FUNCTIONS
                        );
                    }
                    for ty in reader {
                        states.last_mut().unwrap().core.func_to_type.push(Some(ty?))
                    }
                    self.update_custom_section_place(&mut states, "after func");
                }
                Payload::TableSection(s) => {
                    Self::ensure_module(&states)?;
                    self.print_tables(states.last_mut().unwrap(), s)?;
                    self.update_custom_section_place(&mut states, "after table");
                }
                Payload::MemorySection(s) => {
                    Self::ensure_module(&states)?;
                    self.print_memories(states.last_mut().unwrap(), s)?;
                    self.update_custom_section_place(&mut states, "after memory");
                }
                Payload::TagSection(s) => {
                    Self::ensure_module(&states)?;
                    self.print_tags(states.last_mut().unwrap(), s)?;
                    self.update_custom_section_place(&mut states, "after tag");
                }
                Payload::GlobalSection(s) => {
                    Self::ensure_module(&states)?;
                    self.print_globals(states.last_mut().unwrap(), s)?;
                    self.update_custom_section_place(&mut states, "after global");
                }
                Payload::ExportSection(s) => {
                    Self::ensure_module(&states)?;
                    self.print_exports(states.last().unwrap(), s)?;
                    self.update_custom_section_place(&mut states, "after export");
                }
                Payload::StartSection { func, range } => {
                    Self::ensure_module(&states)?;
                    self.newline(range.start)?;
                    self.start_group("start ")?;
                    self.print_idx(&states.last().unwrap().core.func_names, func)?;
                    self.end_group()?;
                    self.update_custom_section_place(&mut states, "after start");
                }
                Payload::ElementSection(s) => {
                    Self::ensure_module(&states)?;
                    self.print_elems(states.last_mut().unwrap(), s)?;
                    self.update_custom_section_place(&mut states, "after elem");
                }
                Payload::CodeSectionStart { .. } => {
                    Self::ensure_module(&states)?;
                }
                Payload::CodeSectionEntry(body) => {
                    self.print_code_section_entry(
                        states.last_mut().unwrap(),
                        &body,
                        validator.as_mut().and_then(|v| v.next_func()),
                    )?;
                    self.update_custom_section_place(&mut states, "after code");
                }
                Payload::DataCountSection { .. } => {
                    Self::ensure_module(&states)?;
                    // not part of the text format
                }
                Payload::DataSection(s) => {
                    Self::ensure_module(&states)?;
                    self.print_data(states.last_mut().unwrap(), s)?;
                    self.update_custom_section_place(&mut states, "after data");
                }

                #[cfg(feature = "component-model")]
                Payload::ModuleSection {
                    parser: inner,
                    unchecked_range: range,
                } => {
                    Self::ensure_component(&states)?;
                    expected = Some(Encoding::Module);
                    parsers.push(parser);
                    parser = inner;
                    self.newline(range.start)?;
                }
                #[cfg(feature = "component-model")]
                Payload::InstanceSection(s) => {
                    Self::ensure_component(&states)?;
                    self.print_instances(states.last_mut().unwrap(), s)?;
                }
                #[cfg(feature = "component-model")]
                Payload::CoreTypeSection(s) => self.print_core_types(&mut states, s)?,
                #[cfg(feature = "component-model")]
                Payload::ComponentSection {
                    parser: inner,
                    unchecked_range: range,
                } => {
                    Self::ensure_component(&states)?;
                    expected = Some(Encoding::Component);
                    parsers.push(parser);
                    parser = inner;
                    self.newline(range.start)?;
                }
                #[cfg(feature = "component-model")]
                Payload::ComponentInstanceSection(s) => {
                    Self::ensure_component(&states)?;
                    self.print_component_instances(states.last_mut().unwrap(), s)?;
                }
                #[cfg(feature = "component-model")]
                Payload::ComponentAliasSection(s) => {
                    Self::ensure_component(&states)?;
                    self.print_component_aliases(&mut states, s)?;
                }
                #[cfg(feature = "component-model")]
                Payload::ComponentTypeSection(s) => {
                    Self::ensure_component(&states)?;
                    self.print_component_types(&mut states, s)?;
                }
                #[cfg(feature = "component-model")]
                Payload::ComponentCanonicalSection(s) => {
                    Self::ensure_component(&states)?;
                    self.print_canonical_functions(states.last_mut().unwrap(), s)?;
                }
                #[cfg(feature = "component-model")]
                Payload::ComponentStartSection { start, range } => {
                    Self::ensure_component(&states)?;
                    self.print_component_start(states.last_mut().unwrap(), range.start, start)?;
                }
                #[cfg(feature = "component-model")]
                Payload::ComponentImportSection(s) => {
                    Self::ensure_component(&states)?;
                    self.print_component_imports(states.last_mut().unwrap(), s)?;
                }
                #[cfg(feature = "component-model")]
                Payload::ComponentExportSection(s) => {
                    Self::ensure_component(&states)?;
                    self.print_component_exports(states.last_mut().unwrap(), s)?;
                }

                Payload::End(offset) => {
                    self.end_group()?; // close the `module` or `component` group

                    #[cfg(feature = "component-model")]
                    {
                        let state = states.pop().unwrap();
                        if let Some(parent) = states.last_mut() {
                            match state.encoding {
                                Encoding::Module => {
                                    parent.core.modules += 1;
                                }
                                Encoding::Component => {
                                    parent.component.components += 1;
                                }
                            }
                            parser = parsers.pop().unwrap();

                            continue;
                        }
                    }
                    self.newline(offset)?;
                    if self.config.print_offsets {
                        self.result.newline()?;
                    }
                    break;
                }

                other => match other.as_section() {
                    Some((id, _)) => bail!("found unknown section `{}`", id),
                    None => bail!("found unknown payload"),
                },
            }
        }

        Ok(())
    }

    fn update_custom_section_place(&self, states: &mut Vec<State>, place: &'static str) {
        if let Some(last) = states.last_mut() {
            if let Some((prev, prev_line)) = &mut last.custom_section_place {
                if *prev_line != self.line {
                    *prev = place;
                    *prev_line = self.line;
                }
            }
        }
    }

    fn update_custom_section_line(&self, states: &mut Vec<State>) {
        if let Some(last) = states.last_mut() {
            if let Some((_, prev_line)) = &mut last.custom_section_place {
                *prev_line = self.line;
            }
        }
    }

    fn start_group(&mut self, name: &str) -> Result<()> {
        write!(self.result, "(")?;
        self.result.start_keyword()?;
        write!(self.result, "{name}")?;
        self.result.reset_color()?;
        self.nesting += 1;
        self.group_lines.push(self.line);
        Ok(())
    }

    fn end_group(&mut self) -> Result<()> {
        self.nesting -= 1;
        if let Some(line) = self.group_lines.pop() {
            if line != self.line {
                self.newline_unknown_pos()?;
            }
        }
        self.result.write_str(")")?;
        Ok(())
    }

    fn register_names(&mut self, state: &mut State, names: NameSectionReader<'_>) -> Result<()> {
        fn indirect_name_map<K>(
            into: &mut NamingMap<(u32, u32), K>,
            names: IndirectNameMap<'_>,
            name: &str,
        ) -> Result<()> {
            for indirect in names {
                let indirect = indirect?;
                let mut used = match name {
                    // labels can be shadowed, so maintaining the used names is not useful.
                    "label" => None,
                    "local" | "field" => Some(HashSet::new()),
                    _ => unimplemented!("{name} is an unknown type of indirect names"),
                };
                for naming in indirect.names {
                    let naming = naming?;
                    into.index_to_name.insert(
                        (indirect.index, naming.index),
                        Naming::new(naming.name, naming.index, name, used.as_mut()),
                    );
                }
            }
            Ok(())
        }

        for section in names {
            match section? {
                Name::Module { name, .. } => {
                    let name = Naming::new(name, 0, "module", None);
                    state.name = Some(name);
                }
                Name::Function(n) => name_map(&mut state.core.func_names, n, "func")?,
                Name::Local(n) => indirect_name_map(&mut state.core.local_names, n, "local")?,
                Name::Label(n) => indirect_name_map(&mut state.core.label_names, n, "label")?,
                Name::Type(n) => name_map(&mut state.core.type_names, n, "type")?,
                Name::Table(n) => name_map(&mut state.core.table_names, n, "table")?,
                Name::Memory(n) => name_map(&mut state.core.memory_names, n, "memory")?,
                Name::Global(n) => name_map(&mut state.core.global_names, n, "global")?,
                Name::Element(n) => name_map(&mut state.core.element_names, n, "elem")?,
                Name::Data(n) => name_map(&mut state.core.data_names, n, "data")?,
                Name::Field(n) => indirect_name_map(&mut state.core.field_names, n, "field")?,
                Name::Tag(n) => name_map(&mut state.core.tag_names, n, "tag")?,
                Name::Unknown { .. } => (),
            }
        }
        Ok(())
    }

    fn print_rec(
        &mut self,
        state: &mut State,
        offset: Option<usize>,
        rec: RecGroup,
        is_component: bool,
    ) -> Result<()> {
        if rec.is_explicit_rec_group() {
            if is_component {
                self.start_group("core rec")?;
            } else {
                self.start_group("rec")?;
            }
            for ty in rec.into_types() {
                match offset {
                    Some(offset) => self.newline(offset + 2)?,
                    None => self.newline_unknown_pos()?,
                }
                self.print_type(state, ty, false)?;
            }
            self.end_group()?; // `rec`
        } else {
            assert_eq!(rec.types().len(), 1);
            let ty = rec.into_types().next().unwrap();
            self.print_type(state, ty, is_component)?;
        }
        Ok(())
    }

    fn print_type(&mut self, state: &mut State, ty: SubType, is_component: bool) -> Result<()> {
        if is_component {
            self.start_group("core type ")?;
        } else {
            self.start_group("type ")?;
        }
        let ty_idx = state.core.types.len() as u32;
        self.print_name(&state.core.type_names, ty_idx)?;
        self.result.write_str(" ")?;
        self.print_sub(state, &ty, ty_idx)?;
        self.end_group()?; // `type`
        state.core.types.push(Some(ty));
        Ok(())
    }

    fn print_sub(&mut self, state: &State, ty: &SubType, ty_idx: u32) -> Result<u32> {
        let r = if !ty.is_final || !ty.supertype_idx.is_none() {
            self.start_group("sub")?;
            self.print_sub_type(state, ty)?;
            let r = self.print_composite(state, &ty.composite_type, ty_idx)?;
            self.end_group()?; // `sub`
            r
        } else {
            self.print_composite(state, &ty.composite_type, ty_idx)?
        };
        Ok(r)
    }

    fn print_composite(&mut self, state: &State, ty: &CompositeType, ty_idx: u32) -> Result<u32> {
        if ty.shared {
            self.start_group("shared")?;
            self.result.write_str(" ")?;
        }
        let r = match &ty.inner {
            CompositeInnerType::Func(ty) => {
                self.start_group("func")?;
                let r = self.print_func_type(state, ty, None)?;
                self.end_group()?; // `func`
                r
            }
            CompositeInnerType::Array(ty) => {
                self.start_group("array")?;
                let r = self.print_array_type(state, ty)?;
                self.end_group()?; // `array`
                r
            }
            CompositeInnerType::Struct(ty) => {
                self.start_group("struct")?;
                let r = self.print_struct_type(state, ty, ty_idx)?;
                self.end_group()?; // `struct`
                r
            }
            CompositeInnerType::Cont(ty) => {
                self.start_group("cont")?;
                let r = self.print_cont_type(state, ty)?;
                self.end_group()?; // `cont`
                r
            }
        };
        if ty.shared {
            self.end_group()?; // `shared`
        }
        Ok(r)
    }

    fn print_types(&mut self, state: &mut State, parser: TypeSectionReader<'_>) -> Result<()> {
        for ty in parser.into_iter_with_offsets() {
            let (offset, rec_group) = ty?;
            self.newline(offset)?;
            self.print_rec(state, Some(offset), rec_group, false)?;
        }
        Ok(())
    }

    fn print_core_functype_idx(
        &mut self,
        state: &State,
        idx: u32,
        names_for: Option<u32>,
    ) -> Result<Option<u32>> {
        self.print_core_type_ref(state, idx)?;

        match state.core.types.get(idx as usize) {
            Some(Some(SubType {
                composite_type:
                    CompositeType {
                        inner: CompositeInnerType::Func(ty),
                        shared: false,
                    },
                ..
            })) => self.print_func_type(state, ty, names_for).map(Some),
            Some(Some(_)) | Some(None) | None => Ok(None),
        }
    }

    /// Returns the number of parameters, useful for local index calculations
    /// later.
    fn print_func_type(
        &mut self,
        state: &State,
        ty: &FuncType,
        names_for: Option<u32>,
    ) -> Result<u32> {
        if !ty.params().is_empty() {
            self.result.write_str(" ")?;
        }

        let mut params = NamedLocalPrinter::new("param");
        // Note that named parameters must be alone in a `param` block, so
        // we need to be careful to terminate previous param blocks and open
        // a new one if that's the case with a named parameter.
        for (i, param) in ty.params().iter().enumerate() {
            params.start_local(names_for, i as u32, self, state)?;
            self.print_valtype(state, *param)?;
            params.end_local(self)?;
        }
        params.finish(self)?;
        if !ty.results().is_empty() {
            self.result.write_str(" ")?;
            self.start_group("result")?;
            for result in ty.results().iter() {
                self.result.write_str(" ")?;
                self.print_valtype(state, *result)?;
            }
            self.end_group()?;
        }
        Ok(ty.params().len() as u32)
    }

    fn print_field_type(
        &mut self,
        state: &State,
        ty: &FieldType,
        ty_field_idx: Option<(u32, u32)>,
    ) -> Result<u32> {
        self.result.write_str(" ")?;
        if let Some(idxs @ (_, field_idx)) = ty_field_idx {
            match state.core.field_names.index_to_name.get(&idxs) {
                Some(name) => {
                    name.write_identifier(self)?;
                    self.result.write_str(" ")?;
                }
                None if self.config.name_unnamed => write!(self.result, "$#field{field_idx} ")?,
                None => {}
            }
        }
        if ty.mutable {
            self.result.write_str("(mut ")?;
        }
        self.print_storage_type(state, ty.element_type)?;
        if ty.mutable {
            self.result.write_str(")")?;
        }
        Ok(0)
    }

    fn print_array_type(&mut self, state: &State, ty: &ArrayType) -> Result<u32> {
        self.print_field_type(state, &ty.0, None)
    }

    fn print_struct_type(&mut self, state: &State, ty: &StructType, ty_idx: u32) -> Result<u32> {
        for (field_index, field) in ty.fields.iter().enumerate() {
            self.result.write_str(" (field")?;
            self.print_field_type(state, field, Some((ty_idx, field_index as u32)))?;
            self.result.write_str(")")?;
        }
        Ok(0)
    }

    fn print_cont_type(&mut self, state: &State, ct: &ContType) -> Result<u32> {
        self.result.write_str(" ")?;
        self.print_idx(&state.core.type_names, ct.0.as_module_index().unwrap())?;
        Ok(0)
    }

    fn print_sub_type(&mut self, state: &State, ty: &SubType) -> Result<u32> {
        self.result.write_str(" ")?;
        if ty.is_final {
            self.result.write_str("final ")?;
        }
        if let Some(idx) = ty.supertype_idx {
            self.print_idx(&state.core.type_names, idx.as_module_index().unwrap())?;
            self.result.write_str(" ")?;
        }
        Ok(0)
    }

    fn print_storage_type(&mut self, state: &State, ty: StorageType) -> Result<()> {
        match ty {
            StorageType::I8 => self.result.write_str("i8")?,
            StorageType::I16 => self.result.write_str("i16")?,
            StorageType::Val(val_type) => self.print_valtype(state, val_type)?,
        }
        Ok(())
    }

    fn print_valtype(&mut self, state: &State, ty: ValType) -> Result<()> {
        match ty {
            ValType::I32 => self.print_type_keyword("i32")?,
            ValType::I64 => self.print_type_keyword("i64")?,
            ValType::F32 => self.print_type_keyword("f32")?,
            ValType::F64 => self.print_type_keyword("f64")?,
            ValType::V128 => self.print_type_keyword("v128")?,
            ValType::Ref(rt) => self.print_reftype(state, rt)?,
        }
        Ok(())
    }

    fn print_valtypes(&mut self, state: &State, tys: Vec<ValType>) -> Result<()> {
        for ty in tys {
            self.result.write_str(" ")?;
            self.print_valtype(state, ty)?;
        }
        Ok(())
    }

    fn print_reftype(&mut self, state: &State, ty: RefType) -> Result<()> {
        if ty.is_nullable() {
            match ty.as_non_null() {
                RefType::FUNC => self.print_type_keyword("funcref")?,
                RefType::EXTERN => self.print_type_keyword("externref")?,
                RefType::I31 => self.print_type_keyword("i31ref")?,
                RefType::ANY => self.print_type_keyword("anyref")?,
                RefType::NONE => self.print_type_keyword("nullref")?,
                RefType::NOEXTERN => self.print_type_keyword("nullexternref")?,
                RefType::NOFUNC => self.print_type_keyword("nullfuncref")?,
                RefType::EQ => self.print_type_keyword("eqref")?,
                RefType::STRUCT => self.print_type_keyword("structref")?,
                RefType::ARRAY => self.print_type_keyword("arrayref")?,
                RefType::EXN => self.print_type_keyword("exnref")?,
                RefType::NOEXN => self.print_type_keyword("nullexnref")?,
                _ => {
                    self.start_group("ref")?;
                    self.result.write_str(" null ")?;
                    self.print_heaptype(state, ty.heap_type())?;
                    self.end_group()?;
                }
            }
        } else {
            self.start_group("ref ")?;
            self.print_heaptype(state, ty.heap_type())?;
            self.end_group()?;
        }
        Ok(())
    }

    fn print_heaptype(&mut self, state: &State, ty: HeapType) -> Result<()> {
        match ty {
            HeapType::Concrete(i) => {
                self.print_idx(&state.core.type_names, i.as_module_index().unwrap())?;
            }
            HeapType::Abstract { shared, ty } => {
                use AbstractHeapType::*;
                if shared {
                    self.start_group("shared ")?;
                }
                match ty {
                    Func => self.print_type_keyword("func")?,
                    Extern => self.print_type_keyword("extern")?,
                    Any => self.print_type_keyword("any")?,
                    None => self.print_type_keyword("none")?,
                    NoExtern => self.print_type_keyword("noextern")?,
                    NoFunc => self.print_type_keyword("nofunc")?,
                    Eq => self.print_type_keyword("eq")?,
                    Struct => self.print_type_keyword("struct")?,
                    Array => self.print_type_keyword("array")?,
                    I31 => self.print_type_keyword("i31")?,
                    Exn => self.print_type_keyword("exn")?,
                    NoExn => self.print_type_keyword("noexn")?,
                    Cont => self.print_type_keyword("cont")?,
                    NoCont => self.print_type_keyword("nocont")?,
                }
                if shared {
                    self.end_group()?;
                }
            }
        }
        Ok(())
    }

    fn print_type_keyword(&mut self, keyword: &str) -> Result<()> {
        self.result.start_type()?;
        self.result.write_str(keyword)?;
        self.result.reset_color()?;
        Ok(())
    }

    fn print_imports(&mut self, state: &mut State, parser: ImportSectionReader<'_>) -> Result<()> {
        for import in parser.into_iter_with_offsets() {
            let (offset, import) = import?;
            self.newline(offset)?;
            self.print_import(state, &import, true)?;
            match import.ty {
                TypeRef::Func(idx) => {
                    debug_assert!(state.core.func_to_type.len() == state.core.funcs as usize);
                    state.core.funcs += 1;
                    state.core.func_to_type.push(Some(idx))
                }
                TypeRef::Table(_) => state.core.tables += 1,
                TypeRef::Memory(_) => state.core.memories += 1,
                TypeRef::Tag(TagType {
                    kind: _,
                    func_type_idx: idx,
                }) => {
                    debug_assert!(state.core.tag_to_type.len() == state.core.tags as usize);
                    state.core.tags += 1;
                    state.core.tag_to_type.push(Some(idx))
                }
                TypeRef::Global(_) => state.core.globals += 1,
            }
        }
        Ok(())
    }

    fn print_import(&mut self, state: &State, import: &Import<'_>, index: bool) -> Result<()> {
        self.start_group("import ")?;
        self.print_str(import.module)?;
        self.result.write_str(" ")?;
        self.print_str(import.name)?;
        self.result.write_str(" ")?;
        self.print_import_ty(state, &import.ty, index)?;
        self.end_group()?;
        Ok(())
    }

    fn print_import_ty(&mut self, state: &State, ty: &TypeRef, index: bool) -> Result<()> {
        match ty {
            TypeRef::Func(f) => {
                self.start_group("func ")?;
                if index {
                    self.print_name(&state.core.func_names, state.core.funcs)?;
                    self.result.write_str(" ")?;
                }
                self.print_core_type_ref(state, *f)?;
            }
            TypeRef::Table(f) => self.print_table_type(state, f, index)?,
            TypeRef::Memory(f) => self.print_memory_type(state, f, index)?,
            TypeRef::Tag(f) => self.print_tag_type(state, f, index)?,
            TypeRef::Global(f) => self.print_global_type(state, f, index)?,
        }
        self.end_group()?;
        Ok(())
    }

    fn print_table_type(&mut self, state: &State, ty: &TableType, index: bool) -> Result<()> {
        self.start_group("table ")?;
        if index {
            self.print_name(&state.core.table_names, state.core.tables)?;
            self.result.write_str(" ")?;
        }
        if ty.shared {
            self.print_type_keyword("shared ")?;
        }
        if ty.table64 {
            self.print_type_keyword("i64 ")?;
        }
        self.print_limits(ty.initial, ty.maximum)?;
        self.result.write_str(" ")?;
        self.print_reftype(state, ty.element_type)?;
        Ok(())
    }

    fn print_memory_type(&mut self, state: &State, ty: &MemoryType, index: bool) -> Result<()> {
        self.start_group("memory ")?;
        if index {
            self.print_name(&state.core.memory_names, state.core.memories)?;
            self.result.write_str(" ")?;
        }
        if ty.memory64 {
            self.print_type_keyword("i64 ")?;
        }
        self.print_limits(ty.initial, ty.maximum)?;
        if ty.shared {
            self.print_type_keyword(" shared")?;
        }
        if let Some(p) = ty.page_size_log2 {
            let p = 1_u64
                .checked_shl(p)
                .ok_or_else(|| anyhow!("left shift overflow").context("invalid page size"))?;

            self.result.write_str(" ")?;
            self.start_group("pagesize ")?;
            write!(self.result, "{p:#x}")?;
            self.end_group()?;
        }
        Ok(())
    }

    fn print_tag_type(&mut self, state: &State, ty: &TagType, index: bool) -> Result<()> {
        self.start_group("tag ")?;
        if index {
            self.print_name(&state.core.tag_names, state.core.tags)?;
            self.result.write_str(" ")?;
        }
        self.print_core_functype_idx(state, ty.func_type_idx, None)?;
        Ok(())
    }

    fn print_limits<T>(&mut self, initial: T, maximum: Option<T>) -> Result<()>
    where
        T: fmt::Display,
    {
        self.result.start_literal()?;
        write!(self.result, "{initial}")?;
        if let Some(max) = maximum {
            write!(self.result, " {max}")?;
        }
        self.result.reset_color()?;
        Ok(())
    }

    fn print_global_type(&mut self, state: &State, ty: &GlobalType, index: bool) -> Result<()> {
        self.start_group("global ")?;
        if index {
            self.print_name(&state.core.global_names, state.core.globals)?;
            self.result.write_str(" ")?;
        }
        if ty.shared || ty.mutable {
            self.result.write_str("(")?;
            if ty.shared {
                self.print_type_keyword("shared ")?;
            }
            if ty.mutable {
                self.print_type_keyword("mut ")?;
            }
            self.print_valtype(state, ty.content_type)?;
            self.result.write_str(")")?;
        } else {
            self.print_valtype(state, ty.content_type)?;
        }
        Ok(())
    }

    fn print_tables(&mut self, state: &mut State, parser: TableSectionReader<'_>) -> Result<()> {
        for table in parser.into_iter_with_offsets() {
            let (offset, table) = table?;
            self.newline(offset)?;
            self.print_table_type(state, &table.ty, true)?;
            match &table.init {
                TableInit::RefNull => {}
                TableInit::Expr(expr) => {
                    self.result.write_str(" ")?;
                    self.print_const_expr(state, expr, self.config.fold_instructions)?;
                }
            }
            self.end_group()?;
            state.core.tables += 1;
        }
        Ok(())
    }

    fn print_memories(&mut self, state: &mut State, parser: MemorySectionReader<'_>) -> Result<()> {
        for memory in parser.into_iter_with_offsets() {
            let (offset, memory) = memory?;
            self.newline(offset)?;
            self.print_memory_type(state, &memory, true)?;
            self.end_group()?;
            state.core.memories += 1;
        }
        Ok(())
    }

    fn print_tags(&mut self, state: &mut State, parser: TagSectionReader<'_>) -> Result<()> {
        for tag in parser.into_iter_with_offsets() {
            let (offset, tag) = tag?;
            self.newline(offset)?;
            self.print_tag_type(state, &tag, true)?;
            self.end_group()?;
            debug_assert!(state.core.tag_to_type.len() == state.core.tags as usize);
            state.core.tags += 1;
            state.core.tag_to_type.push(Some(tag.func_type_idx));
        }
        Ok(())
    }

    fn print_globals(&mut self, state: &mut State, parser: GlobalSectionReader<'_>) -> Result<()> {
        for global in parser.into_iter_with_offsets() {
            let (offset, global) = global?;
            self.newline(offset)?;
            self.print_global_type(state, &global.ty, true)?;
            self.result.write_str(" ")?;
            self.print_const_expr(state, &global.init_expr, self.config.fold_instructions)?;
            self.end_group()?;
            state.core.globals += 1;
        }
        Ok(())
    }

    fn print_code_section_entry(
        &mut self,
        state: &mut State,
        body: &FunctionBody<'_>,
        validator: Option<operand_stack::FuncValidator>,
    ) -> Result<()> {
        self.newline(body.get_binary_reader().original_position())?;
        self.start_group("func ")?;
        let func_idx = state.core.funcs;
        self.print_name(&state.core.func_names, func_idx)?;
        self.result.write_str(" ")?;
        let ty = match state.core.func_to_type.get(func_idx as usize) {
            Some(Some(x)) => *x,
            _ => panic!("invalid function type"),
        };
        let params = self
            .print_core_functype_idx(state, ty, Some(func_idx))?
            .unwrap_or(0);

        // Hints are stored on `self` in reverse order of function index so
        // check the last one and see if it matches this function.
        let hints = match self.code_section_hints.last() {
            Some((f, _)) if *f == func_idx => {
                let (_, hints) = self.code_section_hints.pop().unwrap();
                hints
            }
            _ => Vec::new(),
        };

        if self.config.print_skeleton {
            self.result.write_str(" ...")?;
        } else {
            self.print_func_body(state, func_idx, params, &body, &hints, validator)?;
        }

        self.end_group()?;
        state.core.funcs += 1;
        Ok(())
    }

    fn print_func_body(
        &mut self,
        state: &mut State,
        func_idx: u32,
        params: u32,
        body: &FunctionBody<'_>,
        branch_hints: &[(usize, BranchHint)],
        mut validator: Option<operand_stack::FuncValidator>,
    ) -> Result<()> {
        let mut first = true;
        let mut local_idx = 0;
        let mut locals = NamedLocalPrinter::new("local");
        let mut reader = body.get_binary_reader();
        let func_start = reader.original_position();
        for _ in 0..reader.read_var_u32()? {
            let offset = reader.original_position();
            let cnt = reader.read_var_u32()?;
            let ty = reader.read()?;
            if MAX_LOCALS
                .checked_sub(local_idx)
                .and_then(|s| s.checked_sub(cnt))
                .is_none()
            {
                bail!("function exceeds the maximum number of locals that can be printed");
            }
            for _ in 0..cnt {
                if first {
                    self.newline(offset)?;
                    first = false;
                }
                locals.start_local(Some(func_idx), params + local_idx, self, state)?;
                self.print_valtype(state, ty)?;
                locals.end_local(self)?;
                local_idx += 1;
            }
        }
        locals.finish(self)?;

        if let Some(f) = &mut validator {
            if let Err(e) = f.read_locals(body.get_binary_reader()) {
                validator = None;
                self.newline_unknown_pos()?;
                write!(self.result, ";; locals are invalid: {e}")?;
            }
        }

        let nesting_start = self.nesting;
        let fold_instructions = self.config.fold_instructions;
        let mut operator_state = OperatorState::new(self, OperatorSeparator::Newline);

        if fold_instructions {
            let mut folded_printer = PrintOperatorFolded::new(self, state, &mut operator_state);
            folded_printer.set_offset(func_start);
            folded_printer.begin_function(func_idx)?;
            Self::print_operators(
                &mut reader,
                branch_hints,
                func_start,
                &mut folded_printer,
                validator,
            )?;
        } else {
            let mut flat_printer = PrintOperator::new(self, state, &mut operator_state);
            Self::print_operators(
                &mut reader,
                branch_hints,
                func_start,
                &mut flat_printer,
                validator,
            )?;
        }

        // If this was an invalid function body then the nesting may not
        // have reset back to normal. Fix that up here and forcibly insert
        // a newline as well in case the last instruction was something
        // like an `if` which has a comment after it which could interfere
        // with the closing paren printed for the func.
        if self.nesting != nesting_start {
            self.nesting = nesting_start;
            self.newline(reader.original_position())?;
        }

        Ok(())
    }

    fn print_operators<'a, O: OpPrinter>(
        body: &mut BinaryReader<'a>,
        mut branch_hints: &[(usize, BranchHint)],
        func_start: usize,
        op_printer: &mut O,
        mut validator: Option<operand_stack::FuncValidator>,
    ) -> Result<()> {
        let mut ops = OperatorsReader::new(body.clone());
        while !ops.eof() {
            if ops.is_end_then_eof() {
                let mut annotation = None;
                if let Some(f) = &mut validator {
                    match f.visit_operator(&ops, true) {
                        Ok(()) => {}
                        Err(_) => {
                            annotation = Some(String::from("type mismatch at end of expression"))
                        }
                    }
                }

                ops.read()?; // final "end" opcode terminates instruction sequence
                ops.finish()?;
                op_printer.finalize(annotation.as_deref())?;
                return Ok(());
            }

            // Branch hints are stored in increasing order of their body offset
            // so print them whenever their instruction comes up.
            if let Some(((hint_offset, hint), rest)) = branch_hints.split_first() {
                if hint.func_offset == (ops.original_position() - func_start) as u32 {
                    branch_hints = rest;
                    op_printer.branch_hint(*hint_offset, hint.taken)?;
                }
            }
            let mut annotation = None;
            if let Some(f) = &mut validator {
                let result = f
                    .visit_operator(&ops, false)
                    .map_err(anyhow::Error::from)
                    .and_then(|()| f.visualize_operand_stack(op_printer.use_color()));
                match result {
                    Ok(s) => annotation = Some(s),
                    Err(_) => {
                        validator = None;
                        annotation = Some(String::from("(invalid)"));
                    }
                }
            }
            op_printer.set_offset(ops.original_position());
            op_printer.visit_operator(&mut ops, annotation.as_deref())?;
        }
        ops.finish()?; // for the error message
        bail!("unexpected end of operators");
    }

    fn newline(&mut self, offset: usize) -> Result<()> {
        self.print_newline(Some(offset))
    }

    fn newline_unknown_pos(&mut self) -> Result<()> {
        self.print_newline(None)
    }

    fn print_newline(&mut self, offset: Option<usize>) -> Result<()> {
        self.result.newline()?;
        self.result.start_line(offset);

        if self.config.print_offsets {
            match offset {
                Some(offset) => {
                    self.result.start_comment()?;
                    write!(self.result, "(;@{offset:<6x};)")?;
                    self.result.reset_color()?;
                }
                None => self.result.write_str("           ")?,
            }
        }
        self.line += 1;

        // Clamp the maximum nesting size that we print at something somewhat
        // reasonable to avoid generating hundreds of megabytes of whitespace
        // for small-ish modules that have deep-ish nesting.
        for _ in 0..self.nesting.min(MAX_NESTING_TO_PRINT) {
            self.result.write_str(&self.config.indent_text)?;
        }
        Ok(())
    }

    fn print_exports(&mut self, state: &State, data: ExportSectionReader) -> Result<()> {
        for export in data.into_iter_with_offsets() {
            let (offset, export) = export?;
            self.newline(offset)?;
            self.print_export(state, &export)?;
        }
        Ok(())
    }

    fn print_export(&mut self, state: &State, export: &Export) -> Result<()> {
        self.start_group("export ")?;
        self.print_str(export.name)?;
        self.result.write_str(" ")?;
        self.print_external_kind(state, export.kind, export.index)?;
        self.end_group()?; // export
        Ok(())
    }

    fn print_external_kind(&mut self, state: &State, kind: ExternalKind, index: u32) -> Result<()> {
        match kind {
            ExternalKind::Func => {
                self.start_group("func ")?;
                self.print_idx(&state.core.func_names, index)?;
            }
            ExternalKind::Table => {
                self.start_group("table ")?;
                self.print_idx(&state.core.table_names, index)?;
            }
            ExternalKind::Global => {
                self.start_group("global ")?;
                self.print_idx(&state.core.global_names, index)?;
            }
            ExternalKind::Memory => {
                self.start_group("memory ")?;
                self.print_idx(&state.core.memory_names, index)?;
            }
            ExternalKind::Tag => {
                self.start_group("tag ")?;
                write!(self.result, "{index}")?;
            }
        }
        self.end_group()?;
        Ok(())
    }

    fn print_core_type_ref(&mut self, state: &State, idx: u32) -> Result<()> {
        self.start_group("type ")?;
        self.print_idx(&state.core.type_names, idx)?;
        self.end_group()?;
        Ok(())
    }

    // Note: in the text format, modules can use identifiers that are defined anywhere, but
    // components can only use previously-defined identifiers. In the binary format,
    // invalid components can make forward references to an index that appears in the name section;
    // these can be printed but the output won't parse.
    fn print_idx<K>(&mut self, names: &NamingMap<u32, K>, idx: u32) -> Result<()>
    where
        K: NamingNamespace,
    {
        self._print_idx(&names.index_to_name, idx, K::desc())
    }

    fn _print_idx(&mut self, names: &HashMap<u32, Naming>, idx: u32, desc: &str) -> Result<()> {
        self.result.start_name()?;
        match names.get(&idx) {
            Some(name) => name.write_identifier(self)?,
            None if self.config.name_unnamed => write!(self.result, "$#{desc}{idx}")?,
            None => write!(self.result, "{idx}")?,
        }
        self.result.reset_color()?;
        Ok(())
    }

    fn print_local_idx(&mut self, state: &State, func: u32, idx: u32) -> Result<()> {
        self.result.start_name()?;
        match state.core.local_names.index_to_name.get(&(func, idx)) {
            Some(name) => name.write_identifier(self)?,
            None if self.config.name_unnamed => write!(self.result, "$#local{idx}")?,
            None => write!(self.result, "{idx}")?,
        }
        self.result.reset_color()?;
        Ok(())
    }

    fn print_field_idx(&mut self, state: &State, ty: u32, idx: u32) -> Result<()> {
        self.result.start_name()?;
        match state.core.field_names.index_to_name.get(&(ty, idx)) {
            Some(name) => name.write_identifier(self)?,
            None if self.config.name_unnamed => write!(self.result, "$#field{idx}")?,
            None => write!(self.result, "{idx}")?,
        }
        self.result.reset_color()?;
        Ok(())
    }

    fn print_name<K>(&mut self, names: &NamingMap<u32, K>, cur_idx: u32) -> Result<()>
    where
        K: NamingNamespace,
    {
        self._print_name(&names.index_to_name, cur_idx, K::desc())
    }

    fn _print_name(
        &mut self,
        names: &HashMap<u32, Naming>,
        cur_idx: u32,
        desc: &str,
    ) -> Result<()> {
        self.result.start_name()?;
        match names.get(&cur_idx) {
            Some(name) => {
                name.write(self)?;
                self.result.write_str(" ")?;
            }
            None if self.config.name_unnamed => {
                write!(self.result, "$#{desc}{cur_idx} ")?;
            }
            None => {}
        }
        write!(self.result, "(;{cur_idx};)")?;
        self.result.reset_color()?;
        Ok(())
    }

    fn print_elems(&mut self, state: &mut State, data: ElementSectionReader) -> Result<()> {
        for (i, elem) in data.into_iter_with_offsets().enumerate() {
            let (offset, mut elem) = elem?;
            self.newline(offset)?;
            self.start_group("elem ")?;
            self.print_name(&state.core.element_names, i as u32)?;
            match &mut elem.kind {
                ElementKind::Passive => {}
                ElementKind::Declared => self.result.write_str(" declare")?,
                ElementKind::Active {
                    table_index,
                    offset_expr,
                } => {
                    if let Some(table_index) = *table_index {
                        self.result.write_str(" ")?;
                        self.start_group("table ")?;
                        self.print_idx(&state.core.table_names, table_index)?;
                        self.end_group()?;
                    }
                    self.result.write_str(" ")?;
                    self.print_const_expr_sugar(state, offset_expr, "offset")?;
                }
            }
            self.result.write_str(" ")?;

            if self.config.print_skeleton {
                self.result.write_str("...")?;
            } else {
                match elem.items {
                    ElementItems::Functions(reader) => {
                        self.result.write_str("func")?;
                        for idx in reader {
                            self.result.write_str(" ")?;
                            self.print_idx(&state.core.func_names, idx?)?
                        }
                    }
                    ElementItems::Expressions(ty, reader) => {
                        self.print_reftype(state, ty)?;
                        for expr in reader {
                            self.result.write_str(" ")?;
                            self.print_const_expr_sugar(state, &expr?, "item")?
                        }
                    }
                }
            }
            self.end_group()?;
        }
        Ok(())
    }

    fn print_data(&mut self, state: &mut State, data: DataSectionReader) -> Result<()> {
        for (i, data) in data.into_iter_with_offsets().enumerate() {
            let (offset, data) = data?;
            self.newline(offset)?;
            self.start_group("data ")?;
            self.print_name(&state.core.data_names, i as u32)?;
            self.result.write_str(" ")?;
            match &data.kind {
                DataKind::Passive => {}
                DataKind::Active {
                    memory_index,
                    offset_expr,
                } => {
                    if *memory_index != 0 {
                        self.start_group("memory ")?;
                        self.print_idx(&state.core.memory_names, *memory_index)?;
                        self.end_group()?;
                        self.result.write_str(" ")?;
                    }
                    self.print_const_expr_sugar(state, offset_expr, "offset")?;
                    self.result.write_str(" ")?;
                }
            }
            if self.config.print_skeleton {
                self.result.write_str("...")?;
            } else {
                self.print_bytes(data.data)?;
            }
            self.end_group()?;
        }
        Ok(())
    }

    /// Prints the operators of `expr` space-separated, taking into account that
    /// if there's only one operator in `expr` then instead of `(explicit ...)`
    /// the printing can be `(...)`.
    fn print_const_expr_sugar(
        &mut self,
        state: &mut State,
        expr: &ConstExpr,
        explicit: &str,
    ) -> Result<()> {
        self.start_group("")?;
        let mut reader = expr.get_operators_reader();

        if reader.read().is_ok() && !reader.is_end_then_eof() {
            write!(self.result, "{explicit} ")?;
            self.print_const_expr(state, expr, self.config.fold_instructions)?;
        } else {
            self.print_const_expr(state, expr, false)?;
        }

        self.end_group()?;
        Ok(())
    }

    /// Prints the operators of `expr` space-separated.
    fn print_const_expr(&mut self, state: &mut State, expr: &ConstExpr, fold: bool) -> Result<()> {
        let mut reader = expr.get_binary_reader();
        let mut operator_state = OperatorState::new(self, OperatorSeparator::NoneThenSpace);

        if fold {
            let mut folded_printer = PrintOperatorFolded::new(self, state, &mut operator_state);
            folded_printer.begin_const_expr();
            Self::print_operators(&mut reader, &[], 0, &mut folded_printer, None)?;
        } else {
            let mut op_printer = PrintOperator::new(self, state, &mut operator_state);
            Self::print_operators(&mut reader, &[], 0, &mut op_printer, None)?;
        }

        Ok(())
    }

    fn print_str(&mut self, name: &str) -> Result<()> {
        self.result.start_literal()?;
        self.result.write_str("\"")?;
        self.print_str_contents(name)?;
        self.result.write_str("\"")?;
        self.result.reset_color()?;
        Ok(())
    }

    fn print_str_contents(&mut self, name: &str) -> Result<()> {
        for c in name.chars() {
            let v = c as u32;
            if (0x20..0x7f).contains(&v) && c != '"' && c != '\\' && v < 0xff {
                write!(self.result, "{c}")?;
            } else {
                write!(self.result, "\\u{{{v:x}}}",)?;
            }
        }
        Ok(())
    }

    fn print_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.result.start_literal()?;
        self.result.write_str("\"")?;
        for byte in bytes {
            if *byte >= 0x20 && *byte < 0x7f && *byte != b'"' && *byte != b'\\' {
                write!(self.result, "{}", *byte as char)?;
            } else {
                self.hex_byte(*byte)?;
            }
        }
        self.result.write_str("\"")?;
        self.result.reset_color()?;
        Ok(())
    }

    fn hex_byte(&mut self, byte: u8) -> Result<()> {
        write!(self.result, "\\{byte:02x}")?;
        Ok(())
    }

    fn print_known_custom_section(&mut self, section: CustomSectionReader<'_>) -> Result<bool> {
        match section.as_known() {
            // For now `wasmprinter` has invented syntax for `producers` and
            // `dylink.0` below to use in tests. Note that this syntax is not
            // official at this time.
            KnownCustom::Producers(s) => {
                self.newline(section.range().start)?;
                self.print_producers_section(s)?;
                Ok(true)
            }
            KnownCustom::Dylink0(s) => {
                self.newline(section.range().start)?;
                self.print_dylink0_section(s)?;
                Ok(true)
            }

            // These are parsed during `read_names` and are part of
            // printing elsewhere, so don't print them.
            KnownCustom::Name(_) | KnownCustom::BranchHints(_) => Ok(true),
            #[cfg(feature = "component-model")]
            KnownCustom::ComponentName(_) => Ok(true),

            _ => Ok(false),
        }
    }

    fn print_raw_custom_section(
        &mut self,
        state: &State,
        section: CustomSectionReader<'_>,
    ) -> Result<()> {
        self.newline(section.range().start)?;
        self.start_group("@custom ")?;
        self.print_str(section.name())?;
        if let Some((place, _)) = state.custom_section_place {
            write!(self.result, " ({place})")?;
        }
        self.result.write_str(" ")?;
        if self.config.print_skeleton {
            self.result.write_str("...")?;
        } else {
            self.print_bytes(section.data())?;
        }
        self.end_group()?;
        Ok(())
    }

    fn print_producers_section(&mut self, section: ProducersSectionReader<'_>) -> Result<()> {
        self.start_group("@producers")?;
        for field in section {
            let field = field?;
            for value in field.values.into_iter_with_offsets() {
                let (offset, value) = value?;
                self.newline(offset)?;
                self.start_group(field.name)?;
                self.result.write_str(" ")?;
                self.print_str(value.name)?;
                self.result.write_str(" ")?;
                self.print_str(value.version)?;
                self.end_group()?;
            }
        }
        self.end_group()?;
        Ok(())
    }

    fn print_dylink0_section(&mut self, mut section: Dylink0SectionReader<'_>) -> Result<()> {
        self.start_group("@dylink.0")?;
        loop {
            let start = section.original_position();
            let next = match section.next() {
                Some(Ok(next)) => next,
                Some(Err(e)) => return Err(e.into()),
                None => break,
            };
            match next {
                Dylink0Subsection::MemInfo(info) => {
                    self.newline(start)?;
                    self.start_group("mem-info")?;
                    if info.memory_size > 0 || info.memory_alignment > 0 {
                        write!(
                            self.result,
                            " (memory {} {})",
                            info.memory_size, info.memory_alignment
                        )?;
                    }
                    if info.table_size > 0 || info.table_alignment > 0 {
                        write!(
                            self.result,
                            " (table {} {})",
                            info.table_size, info.table_alignment
                        )?;
                    }
                    self.end_group()?;
                }
                Dylink0Subsection::Needed(needed) => {
                    self.newline(start)?;
                    self.start_group("needed")?;
                    for s in needed {
                        self.result.write_str(" ")?;
                        self.print_str(s)?;
                    }
                    self.end_group()?;
                }
                Dylink0Subsection::ExportInfo(info) => {
                    for info in info {
                        self.newline(start)?;
                        self.start_group("export-info ")?;
                        self.print_str(info.name)?;
                        self.print_dylink0_flags(info.flags)?;
                        self.end_group()?;
                    }
                }
                Dylink0Subsection::ImportInfo(info) => {
                    for info in info {
                        self.newline(start)?;
                        self.start_group("import-info ")?;
                        self.print_str(info.module)?;
                        self.result.write_str(" ")?;
                        self.print_str(info.field)?;
                        self.print_dylink0_flags(info.flags)?;
                        self.end_group()?;
                    }
                }
                Dylink0Subsection::Unknown { ty, .. } => {
                    bail!("don't know how to print dylink.0 subsection id {ty}");
                }
            }
        }
        self.end_group()?;
        Ok(())
    }

    fn print_dylink0_flags(&mut self, mut flags: SymbolFlags) -> Result<()> {
        macro_rules! print_flag {
            ($($name:ident = $text:tt)*) => ({$(
                if flags.contains(SymbolFlags::$name) {
                    flags.remove(SymbolFlags::$name);
                    write!(self.result, concat!(" ", $text))?;
                }
            )*})
        }
        // N.B.: Keep in sync with `parse_sym_flags` in `crates/wast/src/core/custom.rs`.
        print_flag! {
            BINDING_WEAK = "binding-weak"
            BINDING_LOCAL = "binding-local"
            VISIBILITY_HIDDEN = "visibility-hidden"
            UNDEFINED = "undefined"
            EXPORTED = "exported"
            EXPLICIT_NAME = "explicit-name"
            NO_STRIP = "no-strip"
            TLS = "tls"
            ABSOLUTE = "absolute"
        }
        if !flags.is_empty() {
            write!(self.result, " {flags:#x}")?;
        }
        Ok(())
    }

    fn register_branch_hint_section(&mut self, section: BranchHintSectionReader<'_>) -> Result<()> {
        self.code_section_hints.clear();
        for func in section {
            let func = func?;
            if self.code_section_hints.len() >= MAX_WASM_FUNCTIONS as usize {
                bail!("found too many hints");
            }
            if func.hints.count() >= MAX_WASM_FUNCTION_SIZE {
                bail!("found too many hints");
            }
            let hints = func
                .hints
                .into_iter_with_offsets()
                .collect::<wasmparser::Result<Vec<_>>>()?;
            self.code_section_hints.push((func.func, hints));
        }
        self.code_section_hints.reverse();
        Ok(())
    }
}

struct NamedLocalPrinter {
    group_name: &'static str,
    in_group: bool,
    end_group_after_local: bool,
    first: bool,
}

impl NamedLocalPrinter {
    fn new(group_name: &'static str) -> NamedLocalPrinter {
        NamedLocalPrinter {
            group_name,
            in_group: false,
            end_group_after_local: false,
            first: true,
        }
    }

    fn start_local(
        &mut self,
        func: Option<u32>,
        local: u32,
        dst: &mut Printer,
        state: &State,
    ) -> Result<()> {
        let name = state
            .core
            .local_names
            .index_to_name
            .get(&(func.unwrap_or(u32::MAX), local));

        // Named locals must be in their own group, so if we have a name we need
        // to terminate the previous group.
        if name.is_some() && self.in_group {
            dst.end_group()?;
            self.in_group = false;
        }

        if self.first {
            self.first = false;
        } else {
            dst.result.write_str(" ")?;
        }

        // Next we either need a separator if we're already in a group or we
        // need to open a group for our new local.
        if !self.in_group {
            dst.start_group(self.group_name)?;
            dst.result.write_str(" ")?;
            self.in_group = true;
        }

        // Print the optional name if given...
        match name {
            Some(name) => {
                name.write(dst)?;
                dst.result.write_str(" ")?;
                self.end_group_after_local = true;
            }
            None if dst.config.name_unnamed && func.is_some() => {
                write!(dst.result, "$#local{local} ")?;
                self.end_group_after_local = true;
            }
            None => {
                self.end_group_after_local = false;
            }
        }
        Ok(())
    }

    fn end_local(&mut self, dst: &mut Printer) -> Result<()> {
        if self.end_group_after_local {
            dst.end_group()?;
            self.end_group_after_local = false;
            self.in_group = false;
        }
        Ok(())
    }
    fn finish(self, dst: &mut Printer) -> Result<()> {
        if self.in_group {
            dst.end_group()?;
        }
        Ok(())
    }
}

macro_rules! print_float {
    ($name:ident $float:ident $uint:ident $sint:ident $exp_bits:tt) => {
        fn $name(&mut self, mut bits: $uint) -> Result<()> {
            // Calculate a few constants
            let int_width = mem::size_of::<$uint>() * 8;
            let exp_width = $exp_bits;
            let mantissa_width = int_width - 1 - exp_width;
            let bias = (1 << (exp_width - 1)) - 1;
            let max_exp = (1 as $sint) << (exp_width - 1);
            let min_exp = -max_exp + 1;

            // Handle `NaN` and infinity specially
            let f = $float::from_bits(bits);
            if bits >> (int_width - 1) != 0 {
                bits ^= 1 << (int_width - 1);
                self.result.write_str("-")?;
            }
            if f.is_infinite() {
                self.result.start_literal()?;
                self.result.write_str("inf ")?;
                self.result.start_comment()?;
                write!(self.result, "(;={f};)")?;
                self.result.reset_color()?;
                return Ok(());
            }
            if f.is_nan() {
                let payload = bits & ((1 << mantissa_width) - 1);
                self.result.start_literal()?;
                if payload == 1 << (mantissa_width - 1) {
                    self.result.write_str("nan ")?;
                    self.result.start_comment()?;
                    write!(self.result, "(;={f};)")?;
                } else {
                    write!(self.result, "nan:{:#x} ", payload)?;
                    self.result.start_comment()?;
                    write!(self.result, "(;={f};)")?;
                }
                self.result.reset_color()?;
                return Ok(());
            }

            // Figure out our exponent, but keep in mine that it's in an
            // integer width that may not be supported. As a result we do a few
            // tricks here:
            //
            // * Make the MSB the top bit of the exponent, then shift the
            //   exponent to the bottom. This means we now have a signed
            //   integer in `$sint` width representing the whole exponent.
            // * Do the arithmetic for the exponent (subtract)
            // * Next we only care about the lowest `$exp_bits` bits of the
            //   result, but we do care about the sign. Use sign-carrying of
            //   the signed integer shifts to shift it left then shift it back.
            //
            // Overall this should do basic arithmetic for `$exp_bits` bit
            // numbers and get the result back as a signed integer with `$sint`
            // bits in `exponent` representing the same decimal value.
            let mut exponent = (((bits << 1) as $sint) >> (mantissa_width + 1)).wrapping_sub(bias);
            exponent = (exponent << (int_width - exp_width)) >> (int_width - exp_width);
            let mut fraction = bits & ((1 << mantissa_width) - 1);
            self.result.start_literal()?;
            self.result.write_str("0x")?;
            if bits == 0 {
                self.result.write_str("0p+0")?;
            } else {
                self.result.write_str("1")?;
                if fraction > 0 {
                    fraction <<= (int_width - mantissa_width);

                    // Apparently the subnormal is handled here. I don't know
                    // what a subnormal is. If someone else does, please let me
                    // know!
                    if exponent == min_exp {
                        let leading = fraction.leading_zeros();
                        if (leading as usize) < int_width - 1 {
                            fraction <<= leading + 1;
                        } else {
                            fraction = 0;
                        }
                        exponent -= leading as $sint;
                    }

                    self.result.write_str(".")?;
                    while fraction > 0 {
                        write!(self.result, "{:x}", fraction >> (int_width - 4))?;
                        fraction <<= 4;
                    }
                }
                write!(self.result, "p{:+}", exponent)?;
            }
            self.result.start_comment()?;
            write!(self.result, " (;={};)", f)?;
            self.result.reset_color()?;
            Ok(())
        }
    };
}

impl Printer<'_, '_> {
    print_float!(print_f32 f32 u32 i32 8);
    print_float!(print_f64 f64 u64 i64 11);
}

impl Naming {
    fn new<'a>(
        name: &'a str,
        index: u32,
        group: &str,
        used: Option<&mut HashSet<&'a str>>,
    ) -> Naming {
        let mut kind = NamingKind::DollarName;
        if name.chars().any(|c| !is_idchar(c)) {
            kind = NamingKind::DollarQuotedName;
        }

        // If the `name` provided can't be used as the raw identifier for the
        // item that it's describing then a synthetic name must be made. The
        // rules here which generate a name are:
        //
        // * Empty identifiers are not allowed
        // * Identifiers have a fixed set of valid characters
        // * For wasmprinter's purposes we "reserve" identifiers with the `#`
        //   prefix, which is in theory rare to encounter in practice.
        // * If the name has already been used for some other item and cannot
        //   be reused (e.g. because shadowing in this context is not possible).
        //
        // If any of these conditions match then we generate a unique identifier
        // based on `name` but not it exactly. By factoring in the `group`,
        // `index`, and `name` we get a guaranteed unique identifier (due to the
        // leading `#` prefix that we reserve and factoring in of the item
        // index) while preserving human readability at least somewhat (the
        // valid identifier characters of `name` still appear in the returned
        // name).
        if name.is_empty()
            || name.starts_with('#')
            || used.map(|set| !set.insert(name)).unwrap_or(false)
        {
            kind = NamingKind::SyntheticPrefix(format!("#{group}{index}"));
        }
        return Naming {
            kind,
            name: name.to_string(),
        };

        // See https://webassembly.github.io/spec/core/text/values.html#text-id
        fn is_idchar(c: char) -> bool {
            matches!(
                c,
                '0'..='9'
                | 'a'..='z'
                | 'A'..='Z'
                | '!'
                | '#'
                | '$'
                | '%'
                | '&'
                | '\''
                | '*'
                | '+'
                | '-'
                | '.'
                | '/'
                | ':'
                | '<'
                | '='
                | '>'
                | '?'
                | '@'
                | '\\'
                | '^'
                | '_'
                | '`'
                | '|'
                | '~'
            )
        }
    }

    fn write_identifier(&self, printer: &mut Printer<'_, '_>) -> Result<()> {
        match &self.kind {
            NamingKind::DollarName => {
                printer.result.write_str("$")?;
                printer.result.write_str(&self.name)?;
            }
            NamingKind::DollarQuotedName => {
                printer.result.write_str("$\"")?;
                printer.print_str_contents(&self.name)?;
                printer.result.write_str("\"")?;
            }
            NamingKind::SyntheticPrefix(prefix) => {
                printer.result.write_str("$\"")?;
                printer.result.write_str(&prefix)?;
                printer.result.write_str(" ")?;
                printer.print_str_contents(&self.name)?;
                printer.result.write_str("\"")?;
            }
        }
        Ok(())
    }

    fn write(&self, dst: &mut Printer<'_, '_>) -> Result<()> {
        self.write_identifier(dst)?;
        match &self.kind {
            NamingKind::DollarName | NamingKind::DollarQuotedName => {}

            NamingKind::SyntheticPrefix(_) => {
                dst.result.write_str(" ")?;
                dst.start_group("@name \"")?;
                dst.print_str_contents(&self.name)?;
                dst.result.write_str("\"")?;
                dst.end_group()?;
            }
        }
        Ok(())
    }
}

/// Helper trait for the `NamingMap` type's `K` type parameter.
trait NamingNamespace {
    fn desc() -> &'static str;
}

macro_rules! naming_namespaces {
    ($(struct $name:ident => $desc:tt)*) => ($(
        struct $name;

        impl NamingNamespace for $name {
            fn desc() -> &'static str { $desc }
        }
    )*)
}

naming_namespaces! {
    struct NameFunc => "func"
    struct NameGlobal => "global"
    struct NameMemory => "memory"
    struct NameLocal => "local"
    struct NameLabel => "label"
    struct NameTable => "table"
    struct NameType => "type"
    struct NameField => "field"
    struct NameData => "data"
    struct NameElem => "elem"
    struct NameTag => "tag"
}

#[cfg(feature = "component-model")]
naming_namespaces! {
    struct NameModule => "module"
    struct NameInstance => "instance"
    struct NameValue => "value"
    struct NameComponent => "component"
}

fn name_map<K>(into: &mut NamingMap<u32, K>, names: NameMap<'_>, name: &str) -> Result<()> {
    let mut used = HashSet::new();
    for naming in names {
        let naming = naming?;
        into.index_to_name.insert(
            naming.index,
            Naming::new(naming.name, naming.index, name, Some(&mut used)),
        );
    }
    Ok(())
}
