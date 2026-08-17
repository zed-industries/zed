use gimli::write::{
    AttributeValue, LineProgram, Reference, StringTable, Unit, UnitEntryId, UnitId, UnitTable,
};
use wasmtime_environ::StaticModuleIndex;
use wasmtime_versioned_export_macros::versioned_stringify_ident;

use crate::debug::{Compilation, ModuleMemoryOffset};

/// Internal Wasm utility types DIEs such as WebAssemblyPtr and WasmtimeVMContext.
///
/// For unwrapping Wasm pointer, the WasmtimeVMContext has the `set()` method
/// that allows to control current Wasm memory to inspect.
/// Notice that "wasmtime_set_vmctx_memory" is an external/builtin subprogram
/// that is not part of Wasm code.
///
/// This CU is currently per-module since VMContext memory structure is per-module;
/// some of the contained types could be made global (per-Compilation).
pub struct ModuleSyntheticUnit {
    unit_id: UnitId,
    vmctx_ptr_die_id: UnitEntryId,
    wasm_ptr_die_id: UnitEntryId,
}

macro_rules! add_tag {
    ($unit:ident, $parent_id:ident, $tag:expr => $die:ident as $die_id:ident { $($a:path = $v:expr),* }) => {
        let $die_id = $unit.add($parent_id, $tag);
        let $die = $unit.get_mut($die_id);
        $( $die.set($a, $v); )*
    };
}

impl ModuleSyntheticUnit {
    pub fn new(
        module: StaticModuleIndex,
        compilation: &Compilation<'_>,
        encoding: gimli::Encoding,
        out_units: &mut UnitTable,
        out_strings: &mut StringTable,
    ) -> Self {
        let unit_id = Self::create_unit(encoding, out_units, out_strings);
        let unit = out_units.get_mut(unit_id);
        let vmctx_ptr_die_id = Self::create_vmctx_ptr_die(module, compilation, unit, out_strings);
        let wasm_ptr_die_id = Self::create_wasm_ptr_die(unit, out_strings);

        Self {
            unit_id,
            vmctx_ptr_die_id,
            wasm_ptr_die_id,
        }
    }

    pub fn vmctx_ptr_die_ref(&self) -> Reference {
        Reference::Entry(self.unit_id, self.vmctx_ptr_die_id)
    }

    pub fn wasm_ptr_die_ref(&self) -> Reference {
        Reference::Entry(self.unit_id, self.wasm_ptr_die_id)
    }

    fn create_unit(
        encoding: gimli::Encoding,
        out_units: &mut UnitTable,
        out_strings: &mut StringTable,
    ) -> UnitId {
        let unit_id = out_units.add(Unit::new(encoding, LineProgram::none()));
        let unit = out_units.get_mut(unit_id);
        let unit_die = unit.get_mut(unit.root());
        unit_die.set(
            gimli::DW_AT_name,
            AttributeValue::StringRef(out_strings.add("WasmtimeModuleSyntheticUnit")),
        );
        unit_die.set(
            gimli::DW_AT_producer,
            AttributeValue::StringRef(out_strings.add("wasmtime")),
        );
        unit_die.set(
            gimli::DW_AT_language,
            AttributeValue::Language(gimli::DW_LANG_C11),
        );
        unit_id
    }

    fn create_vmctx_ptr_die(
        module: StaticModuleIndex,
        compilation: &Compilation<'_>,
        unit: &mut Unit,
        out_strings: &mut StringTable,
    ) -> UnitEntryId {
        // Build DW_TAG_base_type for Wasm byte:
        //  .. DW_AT_name = u8
        //  .. DW_AT_encoding = DW_ATE_unsigned
        //  .. DW_AT_byte_size = 1
        let root_id = unit.root();
        add_tag!(unit, root_id, gimli::DW_TAG_base_type => memory_byte_die as memory_byte_die_id {
            gimli::DW_AT_name = AttributeValue::StringRef(out_strings.add("u8")),
            gimli::DW_AT_encoding = AttributeValue::Encoding(gimli::DW_ATE_unsigned),
            gimli::DW_AT_byte_size = AttributeValue::Data1(1)
        });

        // Build DW_TAG_pointer_type that references Wasm bytes:
        //  .. DW_AT_name = "u8*"
        //  .. DW_AT_type = <memory_byte_die>
        add_tag!(unit, root_id, gimli::DW_TAG_pointer_type => memory_bytes_die as memory_bytes_die_id {
            gimli::DW_AT_name = AttributeValue::StringRef(out_strings.add("u8*")),
            gimli::DW_AT_type = AttributeValue::UnitRef(memory_byte_die_id)
        });

        // Create artificial VMContext type and its reference for convenience viewing
        // its fields (such as memory ref) in a debugger. Build DW_TAG_structure_type:
        //   .. DW_AT_name = "WasmtimeVMContext"
        let vmctx_die_id = unit.add(root_id, gimli::DW_TAG_structure_type);
        let vmctx_die = unit.get_mut(vmctx_die_id);
        vmctx_die.set(
            gimli::DW_AT_name,
            AttributeValue::StringRef(out_strings.add("WasmtimeVMContext")),
        );

        // TODO multiple memories
        match compilation.module_memory_offsets[module] {
            ModuleMemoryOffset::Defined(memory_offset) => {
                // The context has defined memory: extend the WasmtimeVMContext size
                // past the "memory" field.
                const MEMORY_FIELD_SIZE_PLUS_PADDING: u32 = 8;
                vmctx_die.set(
                    gimli::DW_AT_byte_size,
                    AttributeValue::Data4(memory_offset + MEMORY_FIELD_SIZE_PLUS_PADDING),
                );

                // Define the "memory" field which is a direct pointer to allocated Wasm memory.
                // Build DW_TAG_member:
                //  .. DW_AT_name = "memory"
                //  .. DW_AT_type = <memory_bytes_die>
                //  .. DW_AT_data_member_location = `memory_offset`
                add_tag!(unit, vmctx_die_id, gimli::DW_TAG_member => m_die as m_die_id {
                    gimli::DW_AT_name = AttributeValue::StringRef(out_strings.add("memory")),
                    gimli::DW_AT_type = AttributeValue::UnitRef(memory_bytes_die_id),
                    gimli::DW_AT_data_member_location = AttributeValue::Udata(memory_offset as u64)
                });
            }
            ModuleMemoryOffset::Imported { .. } => {
                // TODO implement convenience pointer to and additional types for VMMemoryImport.
            }
            ModuleMemoryOffset::None => (),
        }

        // Build DW_TAG_pointer_type for `WasmtimeVMContext*`:
        //  .. DW_AT_name = "WasmtimeVMContext*"
        //  .. DW_AT_type = <vmctx_die>
        add_tag!(unit, root_id, gimli::DW_TAG_pointer_type => vmctx_ptr_die as vmctx_ptr_die_id {
            gimli::DW_AT_name = AttributeValue::StringRef(out_strings.add("WasmtimeVMContext*")),
            gimli::DW_AT_type = AttributeValue::UnitRef(vmctx_die_id)
        });

        // Build vmctx_die's DW_TAG_subprogram for `set` method:
        //  .. DW_AT_linkage_name = "wasmtime_set_vmctx_memory"
        //  .. DW_AT_name = "set"
        //  .. DW_TAG_formal_parameter
        //  ..  .. DW_AT_type = <vmctx_ptr_die>
        //  ..  .. DW_AT_artificial = 1
        add_tag!(unit, vmctx_die_id, gimli::DW_TAG_subprogram => vmctx_set as vmctx_set_id {
            gimli::DW_AT_linkage_name = AttributeValue::StringRef(out_strings.add(versioned_stringify_ident!(wasmtime_set_vmctx_memory))),
            gimli::DW_AT_name = AttributeValue::StringRef(out_strings.add("set"))
        });
        add_tag!(unit, vmctx_set_id, gimli::DW_TAG_formal_parameter => vmctx_set_this_param as vmctx_set_this_param_id {
            gimli::DW_AT_type = AttributeValue::UnitRef(vmctx_ptr_die_id),
            gimli::DW_AT_artificial = AttributeValue::Flag(true)
        });

        vmctx_ptr_die_id
    }

    fn create_wasm_ptr_die(unit: &mut Unit, out_strings: &mut StringTable) -> UnitEntryId {
        // Build DW_TAG_base_type for generic `WebAssemblyPtr`.
        //  .. DW_AT_name = "WebAssemblyPtr"
        //  .. DW_AT_byte_size = 4
        //  .. DW_AT_encoding = DW_ATE_unsigned
        const WASM_PTR_LEN: u8 = 4;
        let root_id = unit.root();
        add_tag!(unit, root_id, gimli::DW_TAG_base_type => wp_die as wp_die_id {
            gimli::DW_AT_name = AttributeValue::StringRef(out_strings.add("WebAssemblyPtr")),
            gimli::DW_AT_byte_size = AttributeValue::Data1(WASM_PTR_LEN),
            gimli::DW_AT_encoding = AttributeValue::Encoding(gimli::DW_ATE_unsigned)
        });

        wp_die_id
    }
}
