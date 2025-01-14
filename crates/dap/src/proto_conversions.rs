use anyhow::{anyhow, Result};
use client::proto::{
    self, DapChecksum, DapChecksumAlgorithm, DapModule, DapScope, DapScopePresentationHint,
    DapSource, DapSourcePresentationHint, DapStackFrame, DapVariable, SetDebugClientCapabilities,
};
use dap_types::{Capabilities, ScopePresentationHint, Source};

pub trait ProtoConversion {
    type ProtoType;
    type Output;

    fn to_proto(&self) -> Self::ProtoType;
    fn from_proto(payload: Self::ProtoType) -> Self::Output;
}

impl<T> ProtoConversion for Vec<T>
where
    T: ProtoConversion<Output = T>,
{
    type ProtoType = Vec<T::ProtoType>;
    type Output = Self;

    fn to_proto(&self) -> Self::ProtoType {
        self.iter().map(|item| item.to_proto()).collect()
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        payload
            .into_iter()
            .map(|item| T::from_proto(item))
            .collect()
    }
}

impl ProtoConversion for dap_types::Scope {
    type ProtoType = DapScope;
    type Output = Self;

    fn to_proto(&self) -> Self::ProtoType {
        Self::ProtoType {
            name: self.name.clone(),
            presentation_hint: self
                .presentation_hint
                .as_ref()
                .map(|hint| hint.to_proto().into()),
            variables_reference: self.variables_reference,
            named_variables: self.named_variables,
            indexed_variables: self.indexed_variables,
            expensive: self.expensive,
            source: self.source.as_ref().map(Source::to_proto),
            line: self.line,
            end_line: self.end_line,
            column: self.column,
            end_column: self.end_column,
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        let presentation_hint = payload
            .presentation_hint
            .and_then(DapScopePresentationHint::from_i32);
        Self {
            name: payload.name,
            presentation_hint: presentation_hint.map(ScopePresentationHint::from_proto),
            variables_reference: payload.variables_reference,
            named_variables: payload.named_variables,
            indexed_variables: payload.indexed_variables,
            expensive: payload.expensive,
            source: payload.source.map(dap_types::Source::from_proto),
            line: payload.line,
            end_line: payload.end_line,
            column: payload.column,
            end_column: payload.end_column,
        }
    }
}

impl ProtoConversion for dap_types::Variable {
    type ProtoType = DapVariable;
    type Output = Self;

    fn to_proto(&self) -> Self::ProtoType {
        Self::ProtoType {
            name: self.name.clone(),
            value: self.value.clone(),
            r#type: self.type_.clone(),
            evaluate_name: self.evaluate_name.clone(),
            variables_reference: self.variables_reference,
            named_variables: self.named_variables,
            indexed_variables: self.indexed_variables,
            memory_reference: self.memory_reference.clone(),
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        Self {
            name: payload.name,
            value: payload.value,
            type_: payload.r#type,
            evaluate_name: payload.evaluate_name,
            presentation_hint: None, // TODO Debugger Collab Add this
            variables_reference: payload.variables_reference,
            named_variables: payload.named_variables,
            indexed_variables: payload.indexed_variables,
            memory_reference: payload.memory_reference,
        }
    }
}

impl ProtoConversion for dap_types::ScopePresentationHint {
    type ProtoType = DapScopePresentationHint;
    type Output = Self;

    fn to_proto(&self) -> Self::ProtoType {
        match self {
            dap_types::ScopePresentationHint::Locals => DapScopePresentationHint::Locals,
            dap_types::ScopePresentationHint::Arguments => DapScopePresentationHint::Arguments,
            dap_types::ScopePresentationHint::Registers => DapScopePresentationHint::Registers,
            dap_types::ScopePresentationHint::ReturnValue => DapScopePresentationHint::ReturnValue,
            dap_types::ScopePresentationHint::Unknown => DapScopePresentationHint::ScopeUnknown,
            &_ => unreachable!(),
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        match payload {
            DapScopePresentationHint::Locals => dap_types::ScopePresentationHint::Locals,
            DapScopePresentationHint::Arguments => dap_types::ScopePresentationHint::Arguments,
            DapScopePresentationHint::Registers => dap_types::ScopePresentationHint::Registers,
            DapScopePresentationHint::ReturnValue => dap_types::ScopePresentationHint::ReturnValue,
            DapScopePresentationHint::ScopeUnknown => dap_types::ScopePresentationHint::Unknown,
        }
    }
}

impl ProtoConversion for dap_types::SourcePresentationHint {
    type ProtoType = DapSourcePresentationHint;
    type Output = Self;

    fn to_proto(&self) -> Self::ProtoType {
        match self {
            dap_types::SourcePresentationHint::Normal => DapSourcePresentationHint::SourceNormal,
            dap_types::SourcePresentationHint::Emphasize => DapSourcePresentationHint::Emphasize,
            dap_types::SourcePresentationHint::Deemphasize => {
                DapSourcePresentationHint::Deemphasize
            }
            dap_types::SourcePresentationHint::Unknown => DapSourcePresentationHint::SourceUnknown,
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        match payload {
            DapSourcePresentationHint::SourceNormal => dap_types::SourcePresentationHint::Normal,
            DapSourcePresentationHint::Emphasize => dap_types::SourcePresentationHint::Emphasize,
            DapSourcePresentationHint::Deemphasize => {
                dap_types::SourcePresentationHint::Deemphasize
            }
            DapSourcePresentationHint::SourceUnknown => dap_types::SourcePresentationHint::Unknown,
        }
    }
}

impl ProtoConversion for dap_types::Checksum {
    type ProtoType = DapChecksum;
    type Output = Self;

    fn to_proto(&self) -> Self::ProtoType {
        DapChecksum {
            algorithm: self.algorithm.to_proto().into(),
            checksum: self.checksum.clone(),
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        Self {
            algorithm: dap_types::ChecksumAlgorithm::from_proto(payload.algorithm()),
            checksum: payload.checksum,
        }
    }
}

impl ProtoConversion for dap_types::ChecksumAlgorithm {
    type ProtoType = DapChecksumAlgorithm;
    type Output = Self;

    fn to_proto(&self) -> Self::ProtoType {
        match self {
            dap_types::ChecksumAlgorithm::Md5 => DapChecksumAlgorithm::Md5,
            dap_types::ChecksumAlgorithm::Sha1 => DapChecksumAlgorithm::Sha1,
            dap_types::ChecksumAlgorithm::Sha256 => DapChecksumAlgorithm::Sha256,
            dap_types::ChecksumAlgorithm::Timestamp => DapChecksumAlgorithm::Timestamp,
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        match payload {
            DapChecksumAlgorithm::Md5 => dap_types::ChecksumAlgorithm::Md5,
            DapChecksumAlgorithm::Sha1 => dap_types::ChecksumAlgorithm::Sha1,
            DapChecksumAlgorithm::Sha256 => dap_types::ChecksumAlgorithm::Sha256,
            DapChecksumAlgorithm::Timestamp => dap_types::ChecksumAlgorithm::Timestamp,
            DapChecksumAlgorithm::ChecksumAlgorithmUnspecified => unreachable!(),
        }
    }
}

impl ProtoConversion for dap_types::Source {
    type ProtoType = DapSource;
    type Output = Self;

    fn to_proto(&self) -> Self::ProtoType {
        Self::ProtoType {
            name: self.name.clone(),
            path: self.path.clone(),
            source_reference: self.source_reference,
            presentation_hint: self.presentation_hint.map(|hint| hint.to_proto().into()),
            origin: self.origin.clone(),
            sources: self
                .sources
                .clone()
                .map(|src| src.to_proto())
                .unwrap_or_default(),
            adapter_data: Default::default(), // TODO Debugger Collab
            checksums: self
                .checksums
                .clone()
                .map(|c| c.to_proto())
                .unwrap_or_default(),
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        Self {
            name: payload.name.clone(),
            path: payload.path.clone(),
            source_reference: payload.source_reference,
            presentation_hint: payload
                .presentation_hint
                .and_then(DapSourcePresentationHint::from_i32)
                .map(dap_types::SourcePresentationHint::from_proto),
            origin: payload.origin.clone(),
            sources: Some(Vec::<dap_types::Source>::from_proto(payload.sources)),
            checksums: Some(Vec::<dap_types::Checksum>::from_proto(payload.checksums)),
            adapter_data: None, // TODO Debugger Collab
        }
    }
}

impl ProtoConversion for dap_types::StackFrame {
    type ProtoType = DapStackFrame;
    type Output = Self;

    fn to_proto(&self) -> Self::ProtoType {
        Self::ProtoType {
            id: self.id,
            name: self.name.clone(),
            source: self.source.as_ref().map(|src| src.to_proto()),
            line: self.line,
            column: self.column,
            end_line: self.end_line,
            end_column: self.end_column,
            can_restart: self.can_restart,
            instruction_pointer_reference: self.instruction_pointer_reference.clone(),
            module_id: None,         // TODO Debugger Collab
            presentation_hint: None, // TODO Debugger Collab
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
            module_id: None,         // TODO Debugger Collab
            presentation_hint: None, // TODO Debugger Collab
        }
    }
}

impl ProtoConversion for dap_types::Module {
    type ProtoType = DapModule;
    type Output = Result<Self>;

    fn to_proto(&self) -> Self::ProtoType {
        let id = match &self.id {
            dap_types::ModuleId::Number(num) => proto::dap_module_id::Id::Number(*num),
            dap_types::ModuleId::String(string) => proto::dap_module_id::Id::String(string.clone()),
        };

        DapModule {
            id: Some(proto::DapModuleId { id: Some(id) }),
            name: self.name.clone(),
            path: self.path.clone(),
            is_optimized: self.is_optimized,
            is_user_code: self.is_user_code,
            version: self.version.clone(),
            symbol_status: self.symbol_status.clone(),
            symbol_file_path: self.symbol_file_path.clone(),
            date_time_stamp: self.date_time_stamp.clone(),
            address_range: self.address_range.clone(),
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Result<Self> {
        let id = match payload
            .id
            .ok_or(anyhow!("All DapModule proto messages must have an id"))?
            .id
            .ok_or(anyhow!("All DapModuleID proto messages must have an id"))?
        {
            proto::dap_module_id::Id::String(string) => dap_types::ModuleId::String(string),
            proto::dap_module_id::Id::Number(num) => dap_types::ModuleId::Number(num),
        };

        Ok(Self {
            id,
            name: payload.name,
            path: payload.path,
            is_optimized: payload.is_optimized,
            is_user_code: payload.is_user_code,
            version: payload.version,
            symbol_status: payload.symbol_status,
            symbol_file_path: payload.symbol_file_path,
            date_time_stamp: payload.date_time_stamp,
            address_range: payload.address_range,
        })
    }
}

pub fn capabilities_from_proto(payload: &SetDebugClientCapabilities) -> Capabilities {
    Capabilities {
        supports_loaded_sources_request: Some(payload.supports_loaded_sources_request),
        supports_modules_request: Some(payload.supports_modules_request),
        supports_restart_request: Some(payload.supports_restart_request),
        supports_set_expression: Some(payload.supports_set_expression),
        supports_single_thread_execution_requests: Some(
            payload.supports_single_thread_execution_requests,
        ),
        supports_step_back: Some(payload.supports_step_back),
        supports_stepping_granularity: Some(payload.supports_stepping_granularity),
        supports_terminate_threads_request: Some(payload.supports_terminate_threads_request),
        ..Default::default()
    }
}

pub fn capabilities_to_proto(
    capabilities: &Capabilities,
    project_id: u64,
    session_id: u64,
    client_id: u64,
) -> SetDebugClientCapabilities {
    SetDebugClientCapabilities {
        session_id,
        client_id,
        project_id,
        supports_loaded_sources_request: capabilities
            .supports_loaded_sources_request
            .unwrap_or_default(),
        supports_modules_request: capabilities.supports_modules_request.unwrap_or_default(),
        supports_restart_request: capabilities.supports_restart_request.unwrap_or_default(),
        supports_set_expression: capabilities.supports_set_expression.unwrap_or_default(),
        supports_single_thread_execution_requests: capabilities
            .supports_single_thread_execution_requests
            .unwrap_or_default(),
        supports_step_back: capabilities.supports_step_back.unwrap_or_default(),
        supports_stepping_granularity: capabilities
            .supports_stepping_granularity
            .unwrap_or_default(),
        supports_terminate_threads_request: capabilities
            .supports_terminate_threads_request
            .unwrap_or_default(),
    }
}

impl ProtoConversion for dap_types::SteppingGranularity {
    type ProtoType = proto::SteppingGranularity;
    type Output = Self;

    fn to_proto(&self) -> Self::ProtoType {
        match self {
            dap_types::SteppingGranularity::Statement => proto::SteppingGranularity::Statement,
            dap_types::SteppingGranularity::Line => proto::SteppingGranularity::Line,
            dap_types::SteppingGranularity::Instruction => proto::SteppingGranularity::Instruction,
        }
    }

    fn from_proto(payload: Self::ProtoType) -> Self {
        match payload {
            proto::SteppingGranularity::Line => dap_types::SteppingGranularity::Line,
            proto::SteppingGranularity::Instruction => dap_types::SteppingGranularity::Instruction,
            proto::SteppingGranularity::Statement => dap_types::SteppingGranularity::Statement,
        }
    }
}
