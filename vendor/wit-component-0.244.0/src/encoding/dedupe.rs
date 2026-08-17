//! Support for deduplicating module imports when turning them into a component.
//!
//! Core wasm allows for duplicate imports of the same name/field in a core wasm
//! module. This is not allowed in the component model, however, meaning that
//! such a core wasm module cannot be inserted into a component as-is. This
//! module is tasked with tackling this problem.
//!
//! The general idea of this module is to rewrite imports in-place to a
//! different and unique name. The original name is then recorded in
//! [`ModuleImportMap`] which is then plumbed through to the location that
//! classifies all imports. The classification then uses the original names for
//! what an import should be resolved to but records it under the unique names
//! generated here.

use anyhow::Result;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;
use wasm_encoder::reencode::{Reencode, RoundtripReencoder};
use wasm_encoder::{ImportSection, RawSection};
use wasmparser::{Parser, Payload::*};

/// A map of current names (possibly new) to original names, if any.
#[derive(Default)]
pub struct ModuleImportMap {
    /// A two-level map for what names map to what.
    ///
    /// * The first level of the map is the "module" field, or the first string,
    ///   in a wasm import.
    /// * The second level of the map is the "name" field, or the second string,
    ///   in a wasm import.
    /// * The value of the second level is what this name was originally found
    ///   under in the original module. `None` means "same as the hash map key"
    ///   while `Some` means "it's this new name".
    ///
    /// This map is built during `ModuleImportMap::new` and serves double-duty
    /// to actually track duplicate imports.
    renamed_to_original: HashMap<String, HashMap<String, Option<String>>>,
}

impl ModuleImportMap {
    /// Creates a new [`ModuleImportMap`] tracking duplicate imports, if any,
    /// from the `wasm` provided.
    ///
    /// Upon success a new wasm binary (possibly the same as the original) is
    /// returned which is the new source of truth for this module. If duplicate
    /// imports were found then a [`ModuleImportMap`] is returned, otherwise
    /// `None` is returned.
    ///
    /// # Errors
    ///
    /// Returns an error if `wasm` could not be parsed as a wasm module.
    pub fn new<'a>(wasm: Cow<'a, [u8]>) -> Result<(Cow<'a, [u8]>, Option<ModuleImportMap>)> {
        let mut module = wasm_encoder::Module::new();
        let mut ret = ModuleImportMap::default();
        let mut found_duplicate_imports = false;

        for payload in Parser::new(0).parse_all(&wasm) {
            let payload = payload?;
            match &payload {
                Version { encoding, .. } if *encoding == wasmparser::Encoding::Component => {
                    // if this is a component let someone else deal with the
                    // error, we'll punt that up the stack.
                    assert!(!found_duplicate_imports);
                    break;
                }

                // This is the section we're interested in. Go over each import
                // and delegate to `ModuleImportMap::push_import` for figuring
                // out what to do with this import. At the end put the new
                // import section in the `module` we're building.
                ImportSection(i) => {
                    let mut new_import_section = ImportSection::new();
                    for import in i.clone().into_imports() {
                        found_duplicate_imports = ret
                            .push_import(&mut new_import_section, import?)?
                            || found_duplicate_imports;
                    }
                    module.section(&new_import_section);
                }

                // All other sections get plumbed through as-is. This ensures we
                // don't tamper with binary offsets anywhere in the module
                // except the import section, for example.
                _ => {
                    if let Some((id, range)) = payload.as_section() {
                        module.section(&RawSection {
                            id,
                            data: &wasm[range],
                        });
                    }
                }
            }
        }

        if found_duplicate_imports {
            Ok((module.finish().into(), Some(ret)))
        } else {
            Ok((wasm, None))
        }
    }

    /// Returns `Ok(true)` if this is a duplicate import, or `Ok(false)` if it's
    /// a unique import for the first time.
    fn push_import(
        &mut self,
        new_import_section: &mut ImportSection,
        import: wasmparser::Import<'_>,
    ) -> Result<bool> {
        let module_map = self
            .renamed_to_original
            .entry(import.module.to_string())
            .or_insert(HashMap::new());

        // If this import hasn't yet been seen, then great! Record that it is
        // using its original name and encode it with its original name.
        if !module_map.contains_key(import.name) {
            let prev = module_map.insert(import.name.to_string(), None);
            assert!(prev.is_none());
            RoundtripReencoder.parse_import(new_import_section, import)?;
            return Ok(false);
        }

        // FIXME: this is technically O(n^2), but it's also only applicable when
        // a module has lots of imports, and surely that won't happen often...
        // right? ... right?
        //
        // If this isn't fixed by the time someone reads this and is angry about
        // O(n^2), sorry.
        //
        // Otherwise the purpose of this loop is to determine a unique name for
        // the new import, something that hasn't previously been seen before.
        let mut new_name = import.name.to_string();
        for i in 2.. {
            new_name.truncate(import.name.len());
            write!(new_name, " [v{i}]").unwrap();
            if !module_map.contains_key(&new_name) {
                break;
            }
        }

        // Now that `new_name` is unique, record the import in the new import
        // section.
        let mut new_import = import;
        new_import.name = &new_name;
        RoundtripReencoder.parse_import(new_import_section, new_import)?;

        // Also record that `new_name` was originally known as `import.name`
        // for later lookup in `original_names` below.
        let prev = module_map.insert(new_name, Some(import.name.to_string()));
        assert!(prev.is_none());
        Ok(true)
    }

    /// Returns the original `name` that `import` should use, if any.
    ///
    /// If `None` is returned then `import.name` should be used.
    pub fn original_name(&self, import: &wasmparser::Import<'_>) -> Option<&str> {
        self.renamed_to_original
            .get(import.module)?
            .get(import.name)?
            .as_deref()
    }
}
