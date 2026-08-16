//! proto_conversions.rs - DAP protocol conversion utilities.
//! 
//! Converts between DAP types and internal Zed types.
//! 
//! This module implements the `ProtoConversion` trait for various DAP types,
//! enabling serialization/deserialization for the JSON-RPC 2.0 daemon.
//! 
//! NOTE: This version simplifies type mappings because dap-types v1.x uses flat
//! struct names (Variable, Source, StackFrame) without Dap prefixes or nested
//! conversion types (ModuleId, SourcePresentationHint, etc.).

impl ProtoConversion for dap_types::Variable {
    type ProtoType = dap_types::Variable;
    type Output = Self;

    fn to_proto(self) -> Self::ProtoType {
        Self::ProtoType {
            name: self.name,
            value: self.value,
            type_: self.type_,
            evaluate_name: self.evaluate_name,
            presentation_hint: None,
            variables_reference: self.variables_reference,
            named_variables: self.named_variables,
            indexed_variables: self.indexed_variables,
            memory_reference: self.memory_reference,
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        Self {
            name: payload.name,
            value: payload.value,
            type_: payload.r#type,
            evaluate_name: payload.evaluate_name,
            presentation_hint: None,
            variables_reference: payload.variables_reference,
            named_variables: payload.named_variables,
            indexed_variables: payload.indexed_variables,
            memory_reference: payload.memory_reference,
            declaration_location_reference: None,
            value_location_reference: None,
        }
    }
}

impl ProtoConversion for dap_types::Source {
    type ProtoType = dap_types::Source;
    type Output = Self;

    fn to_proto(self) -> Self::ProtoType {
        Self::ProtoType {
            name: self.name,
            path: self.path,
            source_reference: self.source_reference,
            presentation_hint: self.presentation_hint.map(|hint| hint.to_proto()),
            origin: self.origin,
            sources: self.sources.map(|src| src.to_proto()).unwrap_or_default(),
            adapter_data: self.adapter_data.clone().map(|a| a.to_proto()).into(),
            checksums: self.checksums.map(|c| c.to_proto()).unwrap_or_default(),
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        Self {
            name: payload.name,
            path: payload.path,
            source_reference: payload.source_reference,
            presentation_hint: payload
                .presentation_hint
                .and_then(|hint| {
                    // Source presentation hint mapping simplified
                    None
                })
                .map(dap_types::SourcePresentationHint::from_proto),
            origin: payload.origin,
            sources: Some(Vec::<dap_types::Source>::from_proto(payload.sources)),
            checksums: Some(Vec::<dap_types::Checksum>::from_proto(payload.checksums)),
            adapter_data: payload.adapter_data.map(|a| dap_types::AdapterData::from_proto(a)),
        }
    }
}

impl ProtoConversion for dap_types::StackFrame {
    type ProtoType = dap_types::StackFrame;
    type Output = Self;

    fn to_proto(self) -> Self::ProtoType {
        Self::ProtoType {
            id: self.id,
            name: self.name.clone(),
            source: self.source.map(|src| src.to_proto()),
            line: self.line,
            column: self.column,
            end_line: self.end_line,
            end_column: self.end_column,
            can_restart: self.can_restart,
            instruction_pointer_reference: self.instruction_pointer_reference,
            // module_id and presentation_hint mapped as Option fields
            // (types ModuleId/StackFramePresentationHint not available in dap-types v1.x)
            module_id: self.module_id.map(|mid| mid.to_proto()).into(),
            presentation_hint: self.presentation_hint.map(|hint| hint.to_proto().into()),
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        Self {
            id: payload.id,
            name: payload.name,
            source: payload.source.map(dap_types::Source::from_proto),
            line: payload.line,
            column: payload.column,
            end_line: payload.end_line,
            end_column: payload.end_column,
            can_restart: payload.can_restart,
            instruction_pointer_reference: payload.instruction_pointer_reference,
            // module_id and presentation_hint - simplified mapping
            module_id: payload.module_id.map(|mid| dap_types::ModuleId::from_proto(mid)),
            presentation_hint: payload.presentation_hint.map(|hint| {
                dap_types::StackFramePresentationHint::from_proto(hint)
            }),
        }
    }
}