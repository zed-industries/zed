/* Copyright 2018 Mozilla Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::prelude::*;
use crate::{
    AbstractHeapType, BinaryReaderError, Encoding, FromReader, FunctionBody, HeapType, Parser,
    Payload, RefType, Result, SectionLimited, ValType, WASM_MODULE_VERSION, WasmFeatures,
    limits::*,
};
use ::core::mem;
use ::core::ops::Range;
use ::core::sync::atomic::{AtomicUsize, Ordering};
use alloc::sync::Arc;

/// Test whether the given buffer contains a valid WebAssembly module or component,
/// analogous to [`WebAssembly.validate`][js] in the JS API.
///
/// This functions requires the bytes to validate are entirely resident in memory.
/// Additionally this validates the given bytes with the default set of WebAssembly
/// features implemented by `wasmparser`.
///
/// For more fine-tuned control over validation it's recommended to review the
/// documentation of [`Validator`].
///
/// Upon success, the type information for the top-level module or component will
/// be returned.
///
/// [js]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/validate
pub fn validate(bytes: &[u8]) -> Result<Types> {
    Validator::new().validate_all(bytes)
}

#[test]
fn test_validate() {
    assert!(validate(&[0x0, 0x61, 0x73, 0x6d, 0x1, 0x0, 0x0, 0x0]).is_ok());
    assert!(validate(&[0x0, 0x61, 0x73, 0x6d, 0x2, 0x0, 0x0, 0x0]).is_err());
}

#[cfg(feature = "component-model")]
mod component;
#[cfg(feature = "component-model")]
pub mod component_types;
mod core;
mod func;
#[cfg(feature = "component-model")]
pub mod names;
mod operators;
pub mod types;

#[cfg(feature = "component-model")]
use self::component::*;
pub use self::core::ValidatorResources;
use self::core::*;
use self::types::{TypeAlloc, Types, TypesRef};
pub use func::{FuncToValidate, FuncValidator, FuncValidatorAllocations};
pub use operators::Frame;

fn check_max(cur_len: usize, amt_added: u32, max: usize, desc: &str, offset: usize) -> Result<()> {
    if max
        .checked_sub(cur_len)
        .and_then(|amt| amt.checked_sub(amt_added as usize))
        .is_none()
    {
        if max == 1 {
            bail!(offset, "multiple {desc}");
        }

        bail!(offset, "{desc} count exceeds limit of {max}");
    }

    Ok(())
}

fn combine_type_sizes(a: u32, b: u32, offset: usize) -> Result<u32> {
    match a.checked_add(b) {
        Some(sum) if sum < MAX_WASM_TYPE_SIZE => Ok(sum),
        _ => Err(format_err!(
            offset,
            "effective type size exceeds the limit of {MAX_WASM_TYPE_SIZE}",
        )),
    }
}

/// A unique identifier for a particular `Validator`.
///
/// Allows you to save the `ValidatorId` of the [`Validator`][crate::Validator]
/// you get identifiers out of (e.g. [`CoreTypeId`][crate::types::CoreTypeId])
/// and then later assert that you are pairing those identifiers with the same
/// `Validator` instance when accessing the identifier's associated data.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct ValidatorId(usize);

impl Default for ValidatorId {
    #[inline]
    fn default() -> Self {
        static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
        ValidatorId(ID_COUNTER.fetch_add(1, Ordering::AcqRel))
    }
}

/// Validator for a WebAssembly binary module or component.
///
/// This structure encapsulates state necessary to validate a WebAssembly
/// binary. This implements validation as defined by the [core
/// specification][core]. A `Validator` is designed, like
/// [`Parser`], to accept incremental input over time.
/// Additionally a `Validator` is also designed for parallel validation of
/// functions as they are received.
///
/// It's expected that you'll be using a [`Parser`] in tandem with a
/// `Validator`. As each [`Payload`](crate::Payload) is received from a
/// [`Parser`] you'll pass it into a `Validator` to test the validity of the
/// payload. Note that all payloads received from a [`Parser`] are expected to
/// be passed to a [`Validator`]. For example if you receive
/// [`Payload::TypeSection`](crate::Payload) you'll call
/// [`Validator::type_section`] to validate this.
///
/// The design of [`Validator`] is intended that you'll interleave, in your own
/// application's processing, calls to validation. Each variant, after it's
/// received, will be validated and then your application would proceed as
/// usual. At all times, however, you'll have access to the [`Validator`] and
/// the validation context up to that point. This enables applications to check
/// the types of functions and learn how many globals there are, for example.
///
/// [core]: https://webassembly.github.io/spec/core/valid/index.html
#[derive(Default)]
pub struct Validator {
    id: ValidatorId,

    /// The current state of the validator.
    state: State,

    /// The global type space used by the validator and any sub-validators.
    types: TypeAlloc,

    /// The module state when parsing a WebAssembly module.
    module: Option<ModuleState>,

    /// With the component model enabled, this stores the pushed component states.
    /// The top of the stack is the current component state.
    #[cfg(feature = "component-model")]
    components: Vec<ComponentState>,

    /// Enabled WebAssembly feature flags, dictating what's valid and what
    /// isn't.
    features: WasmFeatures,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum State {
    /// A header has not yet been parsed.
    ///
    /// The value is the expected encoding for the header.
    Unparsed(Option<Encoding>),
    /// A module header has been parsed.
    ///
    /// The associated module state is available via [`Validator::module`].
    Module,
    /// A component header has been parsed.
    ///
    /// The associated component state exists at the top of the
    /// validator's [`Validator::components`] stack.
    #[cfg(feature = "component-model")]
    Component,
    /// The parse has completed and no more data is expected.
    End,
}

impl State {
    fn ensure_parsable(&self, offset: usize) -> Result<()> {
        match self {
            Self::Module => Ok(()),
            #[cfg(feature = "component-model")]
            Self::Component => Ok(()),
            Self::Unparsed(_) => Err(BinaryReaderError::new(
                "unexpected section before header was parsed",
                offset,
            )),
            Self::End => Err(BinaryReaderError::new(
                "unexpected section after parsing has completed",
                offset,
            )),
        }
    }

    fn ensure_module(&self, section: &str, offset: usize) -> Result<()> {
        self.ensure_parsable(offset)?;
        let _ = section;

        match self {
            Self::Module => Ok(()),
            #[cfg(feature = "component-model")]
            Self::Component => Err(format_err!(
                offset,
                "unexpected module {section} section while parsing a component",
            )),
            _ => unreachable!(),
        }
    }

    #[cfg(feature = "component-model")]
    fn ensure_component(&self, section: &str, offset: usize) -> Result<()> {
        self.ensure_parsable(offset)?;

        match self {
            Self::Component => Ok(()),
            Self::Module => Err(format_err!(
                offset,
                "unexpected component {section} section while parsing a module",
            )),
            _ => unreachable!(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Unparsed(None)
    }
}

impl WasmFeatures {
    /// NOTE: This only checks that the value type corresponds to the feature set!!
    ///
    /// To check that reference types are valid, we need access to the module
    /// types. Use module.check_value_type.
    pub(crate) fn check_value_type(&self, ty: ValType) -> Result<(), &'static str> {
        match ty {
            ValType::I32 | ValType::I64 => Ok(()),
            ValType::F32 | ValType::F64 => {
                if self.floats() {
                    Ok(())
                } else {
                    Err("floating-point support is disabled")
                }
            }
            ValType::Ref(r) => self.check_ref_type(r),
            ValType::V128 => {
                if self.simd() {
                    Ok(())
                } else {
                    Err("SIMD support is not enabled")
                }
            }
        }
    }

    pub(crate) fn check_ref_type(&self, r: RefType) -> Result<(), &'static str> {
        if !self.reference_types() {
            return Err("reference types support is not enabled");
        }
        match r.heap_type() {
            HeapType::Concrete(_) => {
                // Note that `self.gc_types()` is not checked here because
                // concrete pointers to function types are allowed. GC types
                // are disallowed by instead rejecting the definition of
                // array/struct types and only allowing the definition of
                // function types.

                // Indexed types require either the function-references or gc
                // proposal as gc implies function references here.
                if self.function_references() || self.gc() {
                    Ok(())
                } else {
                    Err("function references required for index reference types")
                }
            }
            HeapType::Exact(_) => {
                // Exact types were introduced wit hthe custom descriptors
                // proposal.
                if self.custom_descriptors() {
                    Ok(())
                } else {
                    Err("custom descriptors required for exact reference types")
                }
            }
            HeapType::Abstract { shared, ty } => {
                use AbstractHeapType::*;
                if shared && !self.shared_everything_threads() {
                    return Err(
                        "shared reference types require the shared-everything-threads proposal",
                    );
                }

                // Apply the "gc-types" feature which disallows all heap types
                // except exnref/funcref.
                if !self.gc_types() && ty != Func && ty != Exn {
                    return Err("gc types are disallowed but found type which requires gc");
                }

                match (ty, r.is_nullable()) {
                    // funcref/externref only require `reference-types`.
                    (Func, true) | (Extern, true) => Ok(()),

                    // Non-nullable func/extern references requires the
                    // `function-references` proposal.
                    (Func | Extern, false) => {
                        if self.function_references() {
                            Ok(())
                        } else {
                            Err("function references required for non-nullable types")
                        }
                    }

                    // These types were added in the gc proposal.
                    (Any | None | Eq | Struct | Array | I31 | NoExtern | NoFunc, _) => {
                        if self.gc() {
                            Ok(())
                        } else {
                            Err("heap types not supported without the gc feature")
                        }
                    }

                    // These types were added in the exception-handling proposal.
                    (Exn | NoExn, _) => {
                        if self.exceptions() {
                            Ok(())
                        } else {
                            Err(
                                "exception refs not supported without the exception handling feature",
                            )
                        }
                    }

                    // These types were added in the stack switching proposal.
                    (Cont | NoCont, _) => {
                        if self.stack_switching() {
                            Ok(())
                        } else {
                            Err(
                                "continuation refs not supported without the stack switching feature",
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Possible return values from [`Validator::payload`].
#[allow(clippy::large_enum_variant)]
pub enum ValidPayload<'a> {
    /// The payload validated, no further action need be taken.
    Ok,
    /// The payload validated, but it started a nested module or component.
    ///
    /// This result indicates that the specified parser should be used instead
    /// of the currently-used parser until this returned one ends.
    Parser(Parser),
    /// A function was found to be validated.
    Func(FuncToValidate<ValidatorResources>, FunctionBody<'a>),
    /// The end payload was validated and the types known to the validator
    /// are provided.
    End(Types),
}

impl Validator {
    /// Creates a new [`Validator`] ready to validate a WebAssembly module
    /// or component.
    ///
    /// The new validator will receive payloads parsed from
    /// [`Parser`], and expects the first payload received to be
    /// the version header from the parser.
    pub fn new() -> Validator {
        Validator::default()
    }

    /// Creates a new [`Validator`] which has the specified set of wasm
    /// features activated for validation.
    ///
    /// This function is the same as [`Validator::new`] except it also allows
    /// you to customize the active wasm features in use for validation. This
    /// can allow enabling experimental proposals or also turning off
    /// on-by-default wasm proposals.
    pub fn new_with_features(features: WasmFeatures) -> Validator {
        let mut ret = Validator::new();
        ret.features = features;
        ret
    }

    /// Returns the wasm features used for this validator.
    pub fn features(&self) -> &WasmFeatures {
        &self.features
    }

    /// Reset this validator's state such that it is ready to validate a new
    /// Wasm module or component.
    ///
    /// This does *not* clear or reset the internal state keeping track of
    /// validated (and deduplicated and canonicalized) types, allowing you to
    /// use the same type identifiers (such as
    /// [`CoreTypeId`][crate::types::CoreTypeId]) for the same types that are
    /// defined multiple times across different modules and components.
    ///
    /// # Panics
    ///
    /// This function will panic if the validator was mid-way through
    /// validating a binary. Validation must complete entirely or not have
    /// started at all for this method to be called.
    ///
    /// # Examples
    ///
    /// ```
    /// fn foo() -> anyhow::Result<()> {
    /// use wasmparser::Validator;
    ///
    /// let mut validator = Validator::default();
    ///
    /// // Two wasm modules, both of which define the same type, but at
    /// // different indices in their respective types index spaces.
    /// let wasm1 = wat::parse_str("
    ///     (module
    ///         (type $same_type (func (param i32) (result f64)))
    ///     )
    /// ")?;
    /// let wasm2 = wat::parse_str("
    ///     (module
    ///         (type $different_type (func))
    ///         (type $same_type (func (param i32) (result f64)))
    ///     )
    /// ")?;
    ///
    /// // Validate the first Wasm module and get the ID of its type.
    /// let types = validator.validate_all(&wasm1)?;
    /// let id1 = types.as_ref().core_type_at_in_module(0);
    ///
    /// // Reset the validator so we can parse the second wasm module inside
    /// // this validator's same context.
    /// validator.reset();
    ///
    /// // Validate the second Wasm module and get the ID of its second type,
    /// // which is the same type as the first Wasm module's only type.
    /// let types = validator.validate_all(&wasm2)?;
    /// let id2 = types.as_ref().core_type_at_in_module(1);
    ///
    /// // Because both modules were processed in the same `Validator`, they
    /// // share the same types context and therefore the same type defined
    /// // multiple times across different modules will be deduplicated and
    /// // assigned the same identifier!
    /// assert_eq!(id1, id2);
    /// assert_eq!(types[id1], types[id2]);
    /// # Ok(())
    /// # }
    /// # foo().unwrap()
    /// ```
    pub fn reset(&mut self) {
        let Validator {
            // Not changing the identifier; users should be able to observe that
            // they are using the same validation context, even after resetting.
            id: _,

            // Don't mess with `types`, we specifically want to reuse canonicalization.
            types: _,

            // Also leave features as they are. While this is perhaps not
            // strictly necessary, it helps us avoid weird bugs where we have
            // different views of what is or is not a valid type at different
            // times, despite using the same `TypeList` and hash consing
            // context, and therefore there could be moments in time where we
            // have "invalid" types inside our current types list.
            features: _,

            state,
            module,
            #[cfg(feature = "component-model")]
            components,
        } = self;

        assert!(
            matches!(state, State::End) || matches!(state, State::Unparsed(None)),
            "cannot reset a validator that did not successfully complete validation"
        );
        assert!(module.is_none());
        #[cfg(feature = "component-model")]
        assert!(components.is_empty());

        *state = State::default();
    }

    /// Get this validator's unique identifier.
    ///
    /// Allows you to assert that you are always working with the same
    /// `Validator` instance, when you can't otherwise statically ensure that
    /// property by e.g. storing a reference to the validator inside your
    /// structure.
    pub fn id(&self) -> ValidatorId {
        self.id
    }

    /// Validates an entire in-memory module or component with this validator.
    ///
    /// This function will internally create a [`Parser`] to parse the `bytes`
    /// provided. The entire module or component specified by `bytes` will be
    /// parsed and validated.
    ///
    /// Upon success, the type information for the top-level module or component
    /// will be returned.
    pub fn validate_all(&mut self, bytes: &[u8]) -> Result<Types> {
        let mut functions_to_validate = Vec::new();
        let mut last_types = None;
        let mut parser = Parser::new(0);
        let _ = &mut parser;
        #[cfg(feature = "features")]
        parser.set_features(self.features);
        for payload in parser.parse_all(bytes) {
            match self.payload(&payload?)? {
                ValidPayload::Func(a, b) => {
                    functions_to_validate.push((a, b));
                }
                ValidPayload::End(types) => {
                    // Only the last (top-level) type information will be returned
                    last_types = Some(types);
                }
                _ => {}
            }
        }

        let mut allocs = FuncValidatorAllocations::default();
        for (func, body) in functions_to_validate {
            let mut validator = func.into_validator(allocs);
            validator.validate(&body)?;
            allocs = validator.into_allocations();
        }

        Ok(last_types.unwrap())
    }

    /// Gets the types known by the validator so far within the
    /// module/component `level` modules/components up from the
    /// module/component currently being parsed.
    ///
    /// For instance, calling `validator.types(0)` will get the types of the
    /// module/component currently being parsed, and `validator.types(1)` will
    /// get the types of the component containing that module/component.
    ///
    /// Returns `None` if there is no module/component that many levels up.
    pub fn types(&self, mut level: usize) -> Option<TypesRef<'_>> {
        if let Some(module) = &self.module {
            if level == 0 {
                return Some(TypesRef::from_module(self.id, &self.types, &module.module));
            } else {
                level -= 1;
                let _ = level;
            }
        }

        #[cfg(feature = "component-model")]
        return self
            .components
            .iter()
            .nth_back(level)
            .map(|component| TypesRef::from_component(self.id, &self.types, component));
        #[cfg(not(feature = "component-model"))]
        return None;
    }

    /// Convenience function to validate a single [`Payload`].
    ///
    /// This function is intended to be used as a convenience. It will
    /// internally perform any validation necessary to validate the [`Payload`]
    /// provided. The convenience part is that you're likely already going to
    /// be matching on [`Payload`] in your application, at which point it's more
    /// appropriate to call the individual methods on [`Validator`] per-variant
    /// in [`Payload`], such as [`Validator::type_section`].
    ///
    /// This function returns a [`ValidPayload`] variant on success, indicating
    /// one of a few possible actions that need to be taken after a payload is
    /// validated. For example function contents are not validated here, they're
    /// returned through [`ValidPayload`] for validation by the caller.
    pub fn payload<'a>(&mut self, payload: &Payload<'a>) -> Result<ValidPayload<'a>> {
        use crate::Payload::*;
        match payload {
            Version {
                num,
                encoding,
                range,
            } => self.version(*num, *encoding, range)?,

            // Module sections
            TypeSection(s) => self.type_section(s)?,
            ImportSection(s) => self.import_section(s)?,
            FunctionSection(s) => self.function_section(s)?,
            TableSection(s) => self.table_section(s)?,
            MemorySection(s) => self.memory_section(s)?,
            TagSection(s) => self.tag_section(s)?,
            GlobalSection(s) => self.global_section(s)?,
            ExportSection(s) => self.export_section(s)?,
            StartSection { func, range } => self.start_section(*func, range)?,
            ElementSection(s) => self.element_section(s)?,
            DataCountSection { count, range } => self.data_count_section(*count, range)?,
            CodeSectionStart {
                count: _,
                range,
                size: _,
            } => self.code_section_start(range)?,
            CodeSectionEntry(body) => {
                let func_validator = self.code_section_entry(body)?;
                return Ok(ValidPayload::Func(func_validator, body.clone()));
            }
            DataSection(s) => self.data_section(s)?,

            // Component sections
            #[cfg(feature = "component-model")]
            ModuleSection {
                parser,
                unchecked_range: range,
                ..
            } => {
                self.module_section(range)?;
                return Ok(ValidPayload::Parser(parser.clone()));
            }
            #[cfg(feature = "component-model")]
            InstanceSection(s) => self.instance_section(s)?,
            #[cfg(feature = "component-model")]
            CoreTypeSection(s) => self.core_type_section(s)?,
            #[cfg(feature = "component-model")]
            ComponentSection {
                parser,
                unchecked_range: range,
                ..
            } => {
                self.component_section(range)?;
                return Ok(ValidPayload::Parser(parser.clone()));
            }
            #[cfg(feature = "component-model")]
            ComponentInstanceSection(s) => self.component_instance_section(s)?,
            #[cfg(feature = "component-model")]
            ComponentAliasSection(s) => self.component_alias_section(s)?,
            #[cfg(feature = "component-model")]
            ComponentTypeSection(s) => self.component_type_section(s)?,
            #[cfg(feature = "component-model")]
            ComponentCanonicalSection(s) => self.component_canonical_section(s)?,
            #[cfg(feature = "component-model")]
            ComponentStartSection { start, range } => self.component_start_section(start, range)?,
            #[cfg(feature = "component-model")]
            ComponentImportSection(s) => self.component_import_section(s)?,
            #[cfg(feature = "component-model")]
            ComponentExportSection(s) => self.component_export_section(s)?,

            End(offset) => return Ok(ValidPayload::End(self.end(*offset)?)),

            CustomSection { .. } => {} // no validation for custom sections
            UnknownSection { id, range, .. } => self.unknown_section(*id, range)?,
        }
        Ok(ValidPayload::Ok)
    }

    /// Validates [`Payload::Version`](crate::Payload).
    pub fn version(&mut self, num: u16, encoding: Encoding, range: &Range<usize>) -> Result<()> {
        match &self.state {
            State::Unparsed(expected) => {
                if let Some(expected) = expected {
                    if *expected != encoding {
                        bail!(
                            range.start,
                            "expected a version header for a {}",
                            match expected {
                                Encoding::Module => "module",
                                Encoding::Component => "component",
                            }
                        );
                    }
                }
            }
            _ => {
                return Err(BinaryReaderError::new(
                    "wasm version header out of order",
                    range.start,
                ));
            }
        }

        self.state = match encoding {
            Encoding::Module => {
                if num == WASM_MODULE_VERSION {
                    assert!(self.module.is_none());
                    self.module = Some(ModuleState::new(self.features));
                    State::Module
                } else {
                    bail!(range.start, "unknown binary version: {num:#x}");
                }
            }
            Encoding::Component => {
                if !self.features.component_model() {
                    bail!(
                        range.start,
                        "unknown binary version and encoding combination: {num:#x} and 0x1, \
                        note: encoded as a component but the WebAssembly component model feature \
                        is not enabled - enable the feature to allow component validation",
                    );
                }
                #[cfg(feature = "component-model")]
                if num == crate::WASM_COMPONENT_VERSION {
                    self.components
                        .push(ComponentState::new(ComponentKind::Component, self.features));
                    State::Component
                } else if num < crate::WASM_COMPONENT_VERSION {
                    bail!(range.start, "unsupported component version: {num:#x}");
                } else {
                    bail!(range.start, "unknown component version: {num:#x}");
                }
                #[cfg(not(feature = "component-model"))]
                bail!(
                    range.start,
                    "component model validation support disabled \
                     at compile time"
                );
            }
        };

        Ok(())
    }

    /// Validates [`Payload::TypeSection`](crate::Payload).
    pub fn type_section(&mut self, section: &crate::TypeSectionReader<'_>) -> Result<()> {
        self.process_module_section(
            section,
            "type",
            |state, _types, count, offset| {
                check_max(
                    state.module.types.len(),
                    count,
                    MAX_WASM_TYPES,
                    "types",
                    offset,
                )?;
                state.module.assert_mut().types.reserve(count as usize);
                Ok(())
            },
            |state, types, rec_group, offset| {
                state
                    .module
                    .assert_mut()
                    .add_types(rec_group, types, offset, true)?;
                Ok(())
            },
        )
    }

    /// Validates [`Payload::ImportSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a module.
    pub fn import_section(&mut self, section: &crate::ImportSectionReader<'_>) -> Result<()> {
        self.process_module_section(
            section,
            "import",
            |state, _, count, offset| {
                check_max(
                    state.module.imports.len(),
                    count,
                    MAX_WASM_IMPORTS,
                    "imports",
                    offset,
                )?;
                state.module.assert_mut().imports.reserve(count as usize);
                Ok(())
            },
            |state, types, imports, _offset| {
                let state = state.module.assert_mut();
                for import_and_offset in imports {
                    let (offset, import) = import_and_offset?;
                    state.add_import(import, types, offset)?;
                }
                Ok(())
            },
        )
    }

    /// Validates [`Payload::FunctionSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a module.
    pub fn function_section(&mut self, section: &crate::FunctionSectionReader<'_>) -> Result<()> {
        self.process_module_section(
            section,
            "function",
            |state, _, count, offset| {
                check_max(
                    state.module.functions.len(),
                    count,
                    MAX_WASM_FUNCTIONS,
                    "functions",
                    offset,
                )?;
                state.module.assert_mut().functions.reserve(count as usize);
                Ok(())
            },
            |state, types, ty, offset| state.module.assert_mut().add_function(ty, types, offset),
        )
    }

    /// Validates [`Payload::TableSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a module.
    pub fn table_section(&mut self, section: &crate::TableSectionReader<'_>) -> Result<()> {
        self.process_module_section(
            section,
            "table",
            |state, _, count, offset| {
                check_max(
                    state.module.tables.len(),
                    count,
                    state.module.max_tables(),
                    "tables",
                    offset,
                )?;
                state.module.assert_mut().tables.reserve(count as usize);
                Ok(())
            },
            |state, types, table, offset| state.add_table(table, types, offset),
        )
    }

    /// Validates [`Payload::MemorySection`](crate::Payload).
    ///
    /// This method should only be called when parsing a module.
    pub fn memory_section(&mut self, section: &crate::MemorySectionReader<'_>) -> Result<()> {
        self.process_module_section(
            section,
            "memory",
            |state, _, count, offset| {
                check_max(
                    state.module.memories.len(),
                    count,
                    state.module.max_memories(),
                    "memories",
                    offset,
                )?;
                state.module.assert_mut().memories.reserve(count as usize);
                Ok(())
            },
            |state, _, ty, offset| state.module.assert_mut().add_memory(ty, offset),
        )
    }

    /// Validates [`Payload::TagSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a module.
    pub fn tag_section(&mut self, section: &crate::TagSectionReader<'_>) -> Result<()> {
        if !self.features.exceptions() {
            return Err(BinaryReaderError::new(
                "exceptions proposal not enabled",
                section.range().start,
            ));
        }
        self.process_module_section(
            section,
            "tag",
            |state, _, count, offset| {
                check_max(
                    state.module.tags.len(),
                    count,
                    MAX_WASM_TAGS,
                    "tags",
                    offset,
                )?;
                state.module.assert_mut().tags.reserve(count as usize);
                Ok(())
            },
            |state, types, ty, offset| state.module.assert_mut().add_tag(ty, types, offset),
        )
    }

    /// Validates [`Payload::GlobalSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a module.
    pub fn global_section(&mut self, section: &crate::GlobalSectionReader<'_>) -> Result<()> {
        self.process_module_section(
            section,
            "global",
            |state, _, count, offset| {
                check_max(
                    state.module.globals.len(),
                    count,
                    MAX_WASM_GLOBALS,
                    "globals",
                    offset,
                )?;
                state.module.assert_mut().globals.reserve(count as usize);
                Ok(())
            },
            |state, types, global, offset| state.add_global(global, types, offset),
        )
    }

    /// Validates [`Payload::ExportSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a module.
    pub fn export_section(&mut self, section: &crate::ExportSectionReader<'_>) -> Result<()> {
        self.process_module_section(
            section,
            "export",
            |state, _, count, offset| {
                check_max(
                    state.module.exports.len(),
                    count,
                    MAX_WASM_EXPORTS,
                    "exports",
                    offset,
                )?;
                state.module.assert_mut().exports.reserve(count as usize);
                Ok(())
            },
            |state, types, e, offset| {
                let state = state.module.assert_mut();
                let ty = state.export_to_entity_type(&e, offset)?;
                state.add_export(e.name, ty, offset, false /* checked above */, types)
            },
        )
    }

    /// Validates [`Payload::StartSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a module.
    pub fn start_section(&mut self, func: u32, range: &Range<usize>) -> Result<()> {
        let offset = range.start;
        self.state.ensure_module("start", offset)?;
        let state = self.module.as_mut().unwrap();

        let ty = state.module.get_func_type(func, &self.types, offset)?;
        if !ty.params().is_empty() || !ty.results().is_empty() {
            return Err(BinaryReaderError::new(
                "invalid start function type",
                offset,
            ));
        }

        Ok(())
    }

    /// Validates [`Payload::ElementSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a module.
    pub fn element_section(&mut self, section: &crate::ElementSectionReader<'_>) -> Result<()> {
        self.process_module_section(
            section,
            "element",
            |state, _, count, offset| {
                check_max(
                    state.module.element_types.len(),
                    count,
                    MAX_WASM_ELEMENT_SEGMENTS,
                    "element segments",
                    offset,
                )?;
                state
                    .module
                    .assert_mut()
                    .element_types
                    .reserve(count as usize);
                Ok(())
            },
            |state, types, e, offset| state.add_element_segment(e, types, offset),
        )
    }

    /// Validates [`Payload::DataCountSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a module.
    pub fn data_count_section(&mut self, count: u32, range: &Range<usize>) -> Result<()> {
        let offset = range.start;
        self.state.ensure_module("data count", offset)?;

        let state = self.module.as_mut().unwrap();

        if count > MAX_WASM_DATA_SEGMENTS as u32 {
            return Err(BinaryReaderError::new(
                "data count section specifies too many data segments",
                offset,
            ));
        }

        state.module.assert_mut().data_count = Some(count);
        Ok(())
    }

    /// Validates [`Payload::CodeSectionStart`](crate::Payload).
    ///
    /// This method should only be called when parsing a module.
    pub fn code_section_start(&mut self, range: &Range<usize>) -> Result<()> {
        let offset = range.start;
        self.state.ensure_module("code", offset)?;

        let state = self.module.as_mut().unwrap();

        // Take a snapshot of the types when we start the code section.
        state.module.assert_mut().snapshot = Some(Arc::new(self.types.commit()));

        Ok(())
    }

    /// Validates [`Payload::CodeSectionEntry`](crate::Payload).
    ///
    /// This function will prepare a [`FuncToValidate`] which can be used to
    /// create a [`FuncValidator`] to validate the function. The function body
    /// provided will not be parsed or validated by this function.
    ///
    /// Note that the returned [`FuncToValidate`] is "connected" to this
    /// [`Validator`] in that it uses the internal context of this validator for
    /// validating the function. The [`FuncToValidate`] can be sent to another
    /// thread, for example, to offload actual processing of functions
    /// elsewhere.
    ///
    /// This method should only be called when parsing a module.
    pub fn code_section_entry(
        &mut self,
        body: &crate::FunctionBody,
    ) -> Result<FuncToValidate<ValidatorResources>> {
        let offset = body.range().start;
        self.state.ensure_module("code", offset)?;
        check_max(
            0,
            u32::try_from(body.range().len())
                .expect("usize already validated to u32 during section-length decoding"),
            MAX_WASM_FUNCTION_SIZE,
            "function body size",
            offset,
        )?;

        let state = self.module.as_mut().unwrap();

        let (index, ty) = state.next_code_index_and_type();
        Ok(FuncToValidate {
            index,
            ty,
            resources: ValidatorResources(state.module.arc().clone()),
            features: self.features,
        })
    }

    /// Validates [`Payload::DataSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a module.
    pub fn data_section(&mut self, section: &crate::DataSectionReader<'_>) -> Result<()> {
        self.process_module_section(
            section,
            "data",
            |_, _, count, offset| {
                check_max(0, count, MAX_WASM_DATA_SEGMENTS, "data segments", offset)
            },
            |state, types, d, offset| state.add_data_segment(d, types, offset),
        )
    }

    /// Validates [`Payload::ModuleSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a component.
    #[cfg(feature = "component-model")]
    pub fn module_section(&mut self, range: &Range<usize>) -> Result<()> {
        self.state.ensure_component("module", range.start)?;

        let current = self.components.last_mut().unwrap();
        check_max(
            current.core_modules.len(),
            1,
            MAX_WASM_MODULES,
            "modules",
            range.start,
        )?;

        match mem::replace(&mut self.state, State::Unparsed(Some(Encoding::Module))) {
            State::Component => {}
            _ => unreachable!(),
        }

        Ok(())
    }

    /// Validates [`Payload::InstanceSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a component.
    #[cfg(feature = "component-model")]
    pub fn instance_section(&mut self, section: &crate::InstanceSectionReader) -> Result<()> {
        self.process_component_section(
            section,
            "core instance",
            |components, _, count, offset| {
                let current = components.last_mut().unwrap();
                check_max(
                    current.instance_count(),
                    count,
                    MAX_WASM_INSTANCES,
                    "instances",
                    offset,
                )?;
                current.core_instances.reserve(count as usize);
                Ok(())
            },
            |components, types, _features, instance, offset| {
                components
                    .last_mut()
                    .unwrap()
                    .add_core_instance(instance, types, offset)
            },
        )
    }

    /// Validates [`Payload::CoreTypeSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a component.
    #[cfg(feature = "component-model")]
    pub fn core_type_section(&mut self, section: &crate::CoreTypeSectionReader<'_>) -> Result<()> {
        self.process_component_section(
            section,
            "core type",
            |components, _types, count, offset| {
                let current = components.last_mut().unwrap();
                check_max(current.type_count(), count, MAX_WASM_TYPES, "types", offset)?;
                current.core_types.reserve(count as usize);
                Ok(())
            },
            |components, types, _features, ty, offset| {
                ComponentState::add_core_type(
                    components, ty, types, offset, false, /* checked above */
                )
            },
        )
    }

    /// Validates [`Payload::ComponentSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a component.
    #[cfg(feature = "component-model")]
    pub fn component_section(&mut self, range: &Range<usize>) -> Result<()> {
        self.state.ensure_component("component", range.start)?;

        let current = self.components.last_mut().unwrap();
        check_max(
            current.components.len(),
            1,
            MAX_WASM_COMPONENTS,
            "components",
            range.start,
        )?;

        match mem::replace(&mut self.state, State::Unparsed(Some(Encoding::Component))) {
            State::Component => {}
            _ => unreachable!(),
        }

        Ok(())
    }

    /// Validates [`Payload::ComponentInstanceSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a component.
    #[cfg(feature = "component-model")]
    pub fn component_instance_section(
        &mut self,
        section: &crate::ComponentInstanceSectionReader,
    ) -> Result<()> {
        self.process_component_section(
            section,
            "instance",
            |components, _, count, offset| {
                let current = components.last_mut().unwrap();
                check_max(
                    current.instance_count(),
                    count,
                    MAX_WASM_INSTANCES,
                    "instances",
                    offset,
                )?;
                current.instances.reserve(count as usize);
                Ok(())
            },
            |components, types, _features, instance, offset| {
                components
                    .last_mut()
                    .unwrap()
                    .add_instance(instance, types, offset)
            },
        )
    }

    /// Validates [`Payload::ComponentAliasSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a component.
    #[cfg(feature = "component-model")]
    pub fn component_alias_section(
        &mut self,
        section: &crate::ComponentAliasSectionReader<'_>,
    ) -> Result<()> {
        self.process_component_section(
            section,
            "alias",
            |_, _, _, _| Ok(()), // maximums checked via `add_alias`
            |components, types, _features, alias, offset| -> Result<(), BinaryReaderError> {
                ComponentState::add_alias(components, alias, types, offset)
            },
        )
    }

    /// Validates [`Payload::ComponentTypeSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a component.
    #[cfg(feature = "component-model")]
    pub fn component_type_section(
        &mut self,
        section: &crate::ComponentTypeSectionReader,
    ) -> Result<()> {
        self.process_component_section(
            section,
            "type",
            |components, _types, count, offset| {
                let current = components.last_mut().unwrap();
                check_max(current.type_count(), count, MAX_WASM_TYPES, "types", offset)?;
                current.types.reserve(count as usize);
                Ok(())
            },
            |components, types, _features, ty, offset| {
                ComponentState::add_type(
                    components, ty, types, offset, false, /* checked above */
                )
            },
        )
    }

    /// Validates [`Payload::ComponentCanonicalSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a component.
    #[cfg(feature = "component-model")]
    pub fn component_canonical_section(
        &mut self,
        section: &crate::ComponentCanonicalSectionReader,
    ) -> Result<()> {
        self.process_component_section(
            section,
            "function",
            |components, _, count, offset| {
                let current = components.last_mut().unwrap();
                check_max(
                    current.function_count(),
                    count,
                    MAX_WASM_FUNCTIONS,
                    "functions",
                    offset,
                )?;
                current.funcs.reserve(count as usize);
                Ok(())
            },
            |components, types, _features, func, offset| {
                let current = components.last_mut().unwrap();
                current.canonical_function(func, types, offset)
            },
        )
    }

    /// Validates [`Payload::ComponentStartSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a component.
    #[cfg(feature = "component-model")]
    pub fn component_start_section(
        &mut self,
        f: &crate::ComponentStartFunction,
        range: &Range<usize>,
    ) -> Result<()> {
        self.state.ensure_component("start", range.start)?;

        self.components.last_mut().unwrap().add_start(
            f.func_index,
            &f.arguments,
            f.results,
            &mut self.types,
            range.start,
        )
    }

    /// Validates [`Payload::ComponentImportSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a component.
    #[cfg(feature = "component-model")]
    pub fn component_import_section(
        &mut self,
        section: &crate::ComponentImportSectionReader,
    ) -> Result<()> {
        self.process_component_section(
            section,
            "import",
            |_, _, _, _| Ok(()), // add_import will check limits
            |components, types, _features, import, offset| {
                components
                    .last_mut()
                    .unwrap()
                    .add_import(import, types, offset)
            },
        )
    }

    /// Validates [`Payload::ComponentExportSection`](crate::Payload).
    ///
    /// This method should only be called when parsing a component.
    #[cfg(feature = "component-model")]
    pub fn component_export_section(
        &mut self,
        section: &crate::ComponentExportSectionReader,
    ) -> Result<()> {
        self.process_component_section(
            section,
            "export",
            |components, _, count, offset| {
                let current = components.last_mut().unwrap();
                check_max(
                    current.exports.len(),
                    count,
                    MAX_WASM_EXPORTS,
                    "exports",
                    offset,
                )?;
                current.exports.reserve(count as usize);
                Ok(())
            },
            |components, types, _features, export, offset| {
                let current = components.last_mut().unwrap();
                let ty = current.export_to_entity_type(&export, types, offset)?;
                current.add_export(
                    export.name,
                    ty,
                    types,
                    offset,
                    false, /* checked above */
                )
            },
        )
    }

    /// Validates [`Payload::UnknownSection`](crate::Payload).
    ///
    /// Currently always returns an error.
    pub fn unknown_section(&mut self, id: u8, range: &Range<usize>) -> Result<()> {
        Err(format_err!(range.start, "malformed section id: {id}"))
    }

    /// Validates [`Payload::End`](crate::Payload).
    ///
    /// Returns the types known to the validator for the module or component.
    pub fn end(&mut self, offset: usize) -> Result<Types> {
        match mem::replace(&mut self.state, State::End) {
            State::Unparsed(_) => Err(BinaryReaderError::new(
                "cannot call `end` before a header has been parsed",
                offset,
            )),
            State::End => Err(BinaryReaderError::new(
                "cannot call `end` after parsing has completed",
                offset,
            )),
            State::Module => {
                let mut state = self.module.take().unwrap();

                // If there's a parent component, we'll add a module to the parent state
                // and continue to validate the component
                #[cfg(feature = "component-model")]
                if let Some(parent) = self.components.last_mut() {
                    parent.add_core_module(&state.module, &mut self.types, offset)?;
                    self.state = State::Component;
                }

                Ok(Types::from_module(
                    self.id,
                    self.types.commit(),
                    state.module.arc().clone(),
                ))
            }
            #[cfg(feature = "component-model")]
            State::Component => {
                let mut component = self.components.pop().unwrap();

                // Validate that all values were used for the component
                if let Some(index) = component.values.iter().position(|(_, used)| !*used) {
                    bail!(
                        offset,
                        "value index {index} was not used as part of an \
                         instantiation, start function, or export"
                    );
                }

                // If there's a parent component, pop the stack, add it to the parent,
                // and continue to validate the component
                let ty = component.finish(&self.types, offset)?;
                if let Some(parent) = self.components.last_mut() {
                    parent.add_component(ty, &mut self.types)?;
                    self.state = State::Component;
                }

                Ok(Types::from_component(
                    self.id,
                    self.types.commit(),
                    component,
                ))
            }
        }
    }

    fn process_module_section<'a, T>(
        &mut self,
        section: &SectionLimited<'a, T>,
        name: &str,
        validate_section: impl FnOnce(&mut ModuleState, &mut TypeAlloc, u32, usize) -> Result<()>,
        mut validate_item: impl FnMut(&mut ModuleState, &mut TypeAlloc, T, usize) -> Result<()>,
    ) -> Result<()>
    where
        T: FromReader<'a>,
    {
        let offset = section.range().start;
        self.state.ensure_module(name, offset)?;

        let state = self.module.as_mut().unwrap();

        validate_section(state, &mut self.types, section.count(), offset)?;

        for item in section.clone().into_iter_with_offsets() {
            let (offset, item) = item?;
            validate_item(state, &mut self.types, item, offset)?;
        }

        Ok(())
    }

    #[cfg(feature = "component-model")]
    fn process_component_section<'a, T>(
        &mut self,
        section: &SectionLimited<'a, T>,
        name: &str,
        validate_section: impl FnOnce(
            &mut Vec<ComponentState>,
            &mut TypeAlloc,
            u32,
            usize,
        ) -> Result<()>,
        mut validate_item: impl FnMut(
            &mut Vec<ComponentState>,
            &mut TypeAlloc,
            &WasmFeatures,
            T,
            usize,
        ) -> Result<()>,
    ) -> Result<()>
    where
        T: FromReader<'a>,
    {
        let offset = section.range().start;

        self.state.ensure_component(name, offset)?;
        validate_section(
            &mut self.components,
            &mut self.types,
            section.count(),
            offset,
        )?;

        for item in section.clone().into_iter_with_offsets() {
            let (offset, item) = item?;
            validate_item(
                &mut self.components,
                &mut self.types,
                &self.features,
                item,
                offset,
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{GlobalType, MemoryType, RefType, TableType, ValType, Validator, WasmFeatures};
    use anyhow::Result;

    #[test]
    fn test_module_type_information() -> Result<()> {
        let bytes = wat::parse_str(
            r#"
            (module
                (type (func (param i32 i64) (result i32)))
                (memory 1 5)
                (table 10 funcref)
                (global (mut i32) (i32.const 0))
                (func (type 0) (i32.const 0))
                (tag (param i64 i32))
                (elem funcref (ref.func 0))
            )
        "#,
        )?;

        let mut validator =
            Validator::new_with_features(WasmFeatures::default() | WasmFeatures::EXCEPTIONS);

        let types = validator.validate_all(&bytes)?;
        let types = types.as_ref();

        assert_eq!(types.core_type_count_in_module(), 2);
        assert_eq!(types.memory_count(), 1);
        assert_eq!(types.table_count(), 1);
        assert_eq!(types.global_count(), 1);
        assert_eq!(types.function_count(), 1);
        assert_eq!(types.tag_count(), 1);
        assert_eq!(types.element_count(), 1);
        assert_eq!(types.module_count(), 0);
        assert_eq!(types.component_count(), 0);
        assert_eq!(types.core_instance_count(), 0);
        assert_eq!(types.value_count(), 0);

        let id = types.core_type_at_in_module(0);
        let ty = types[id].unwrap_func();
        assert_eq!(ty.params(), [ValType::I32, ValType::I64]);
        assert_eq!(ty.results(), [ValType::I32]);

        let id = types.core_type_at_in_module(1);
        let ty = types[id].unwrap_func();
        assert_eq!(ty.params(), [ValType::I64, ValType::I32]);
        assert_eq!(ty.results(), []);

        assert_eq!(
            types.memory_at(0),
            MemoryType {
                memory64: false,
                shared: false,
                initial: 1,
                maximum: Some(5),
                page_size_log2: None,
            }
        );

        assert_eq!(
            types.table_at(0),
            TableType {
                initial: 10,
                maximum: None,
                element_type: RefType::FUNCREF,
                table64: false,
                shared: false,
            }
        );

        assert_eq!(
            types.global_at(0),
            GlobalType {
                content_type: ValType::I32,
                mutable: true,
                shared: false
            }
        );

        let id = types.core_function_at(0);
        let ty = types[id].unwrap_func();
        assert_eq!(ty.params(), [ValType::I32, ValType::I64]);
        assert_eq!(ty.results(), [ValType::I32]);

        let ty = types.tag_at(0);
        let ty = types[ty].unwrap_func();
        assert_eq!(ty.params(), [ValType::I64, ValType::I32]);
        assert_eq!(ty.results(), []);

        assert_eq!(types.element_at(0), RefType::FUNCREF);

        Ok(())
    }

    #[test]
    fn test_type_id_aliasing() -> Result<()> {
        let bytes = wat::parse_str(
            r#"
            (component
              (type $T (list string))
              (alias outer 0 $T (type $A1))
              (alias outer 0 $T (type $A2))
            )
        "#,
        )?;

        let mut validator =
            Validator::new_with_features(WasmFeatures::default() | WasmFeatures::COMPONENT_MODEL);

        let types = validator.validate_all(&bytes)?;
        let types = types.as_ref();

        let t_id = types.component_defined_type_at(0);
        let a1_id = types.component_defined_type_at(1);
        let a2_id = types.component_defined_type_at(2);

        // The ids should all be the same
        assert!(t_id == a1_id);
        assert!(t_id == a2_id);
        assert!(a1_id == a2_id);

        // However, they should all point to the same type
        assert!(std::ptr::eq(&types[t_id], &types[a1_id],));
        assert!(std::ptr::eq(&types[t_id], &types[a2_id],));

        Ok(())
    }

    #[test]
    fn test_type_id_exports() -> Result<()> {
        let bytes = wat::parse_str(
            r#"
            (component
              (type $T (list string))
              (export $A1 "A1" (type $T))
              (export $A2 "A2" (type $T))
            )
        "#,
        )?;

        let mut validator =
            Validator::new_with_features(WasmFeatures::default() | WasmFeatures::COMPONENT_MODEL);

        let types = validator.validate_all(&bytes)?;
        let types = types.as_ref();

        let t_id = types.component_defined_type_at(0);
        let a1_id = types.component_defined_type_at(1);
        let a2_id = types.component_defined_type_at(2);

        // The ids should all be the same
        assert!(t_id != a1_id);
        assert!(t_id != a2_id);
        assert!(a1_id != a2_id);

        // However, they should all point to the same type
        assert!(std::ptr::eq(&types[t_id], &types[a1_id],));
        assert!(std::ptr::eq(&types[t_id], &types[a2_id],));

        Ok(())
    }

    #[test]
    fn reset_fresh_validator() {
        Validator::new().reset();
    }
}
