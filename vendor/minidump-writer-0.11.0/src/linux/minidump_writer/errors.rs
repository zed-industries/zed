use {
    super::super::{
        auxv::AuxvError,
        dso_debug::SectionDsoDebugError,
        maps_reader::MapsReaderError,
        minidump_writer::{
            app_memory::SectionAppMemoryError, exception_stream::SectionExceptionStreamError,
            handle_data_stream::SectionHandleDataStreamError, mappings::SectionMappingsError,
            memory_info_list_stream::SectionMemInfoListError,
            memory_list_stream::SectionMemListError, systeminfo_stream::SectionSystemInfoError,
            thread_list_stream::SectionThreadListError,
            thread_names_stream::SectionThreadNamesError,
        },
        module_reader::ModuleReaderError,
        serializers::*,
        Pid,
    },
    crate::{dir_section::FileWriterError, mem_writer::MemoryWriterError, serializers::*},
    error_graph::ErrorList,
    nix::errno::Errno,
    procfs_core::ProcError,
    std::ffi::OsString,
    thiserror::Error,
};

#[cfg(target_os = "android")]
use super::super::android::AndroidError;

#[derive(Debug, Error, serde::Serialize)]
pub enum WriterError {
    #[error("Error during init phase")]
    InitError(#[from] InitError),
    #[error("Failed when writing section AppMemory")]
    SectionAppMemoryError(#[from] SectionAppMemoryError),
    #[error("Failed when writing section ExceptionStream")]
    SectionExceptionStreamError(#[from] SectionExceptionStreamError),
    #[error("Failed when writing section HandleDataStream")]
    SectionHandleDataStreamError(#[from] SectionHandleDataStreamError),
    #[error("Failed when writing section MappingsError")]
    SectionMappingsError(#[from] SectionMappingsError),
    #[error("Failed when writing section MemList")]
    SectionMemListError(#[from] SectionMemListError),
    #[error("Failed when writing section SystemInfo")]
    SectionSystemInfoError(#[from] SectionSystemInfoError),
    #[error("Failed when writing section MemoryInfoList")]
    SectionMemoryInfoListError(#[from] SectionMemInfoListError),
    #[error("Failed when writing section ThreadList")]
    SectionThreadListError(#[from] SectionThreadListError),
    #[error("Failed when writing section ThreadNameList")]
    SectionThreadNamesError(#[from] SectionThreadNamesError),
    #[error("Failed when writing section DsoDebug")]
    SectionDsoDebugError(#[from] SectionDsoDebugError),
    #[error("Failed to write to memory")]
    MemoryWriterError(#[from] MemoryWriterError),
    #[error("Failed to write to file")]
    FileWriterError(#[from] FileWriterError),
    #[error("Failed to get current timestamp when writing header of minidump")]
    SystemTimeError(
        #[from]
        #[serde(serialize_with = "serialize_system_time_error")]
        std::time::SystemTimeError,
    ),
    #[error("Errors occurred while initializing PTraceDumper")]
    InitErrors(#[source] ErrorList<InitError>),
    #[error("Errors occurred while resuming threads")]
    ResumeThreadsErrors(#[source] ErrorList<WriterError>),
    #[error("Errors occurred while writing system info")]
    WriteSystemInfoErrors(#[source] ErrorList<SectionSystemInfoError>),
    #[error("Failed writing cpuinfo")]
    WriteCpuInfoFailed(#[source] MemoryWriterError),
    #[error("Failed writing thread proc status")]
    WriteThreadProcStatusFailed(#[source] MemoryWriterError),
    #[error("Failed writing OS Release Information")]
    WriteOsReleaseInfoFailed(#[source] MemoryWriterError),
    #[error("Failed writing process command line")]
    WriteCommandLineFailed(#[source] MemoryWriterError),
    #[error("Writing process environment failed")]
    WriteEnvironmentFailed(#[source] MemoryWriterError),
    #[error("Failed to write auxv file")]
    WriteAuxvFailed(#[source] MemoryWriterError),
    #[error("Failed to write maps file")]
    WriteMapsFailed(#[source] MemoryWriterError),
    #[error("Failed writing DSO Debug Stream")]
    WriteDSODebugStreamFailed(#[source] SectionDsoDebugError),
    #[error("Failed writing limits file")]
    WriteLimitsFailed(#[source] MemoryWriterError),
    #[error("Failed writing handle data stream")]
    WriteHandleDataStreamFailed(#[source] SectionHandleDataStreamError),
    #[error("Failed writing handle data stream direction entry")]
    WriteHandleDataStreamDirentFailed(#[source] FileWriterError),
    #[error("Failed to convert soft error list to JSON")]
    ConvertToJsonFailed(
        #[source]
        #[serde(skip)]
        serde_json::Error,
    ),
    #[error("nix::ptrace::attach(Pid={0}) failed")]
    PtraceAttachError(
        Pid,
        #[source]
        #[serde(serialize_with = "serialize_nix_error")]
        nix::Error,
    ),
    #[error("nix::ptrace::detach(Pid={0}) failed")]
    PtraceDetachError(
        Pid,
        #[source]
        #[serde(serialize_with = "serialize_nix_error")]
        nix::Error,
    ),
    #[error("wait::waitpid(Pid={0}) failed")]
    WaitPidError(
        Pid,
        #[source]
        #[serde(serialize_with = "serialize_nix_error")]
        nix::Error,
    ),
    #[error("Skipped thread {0} due to it being part of the seccomp sandbox's trusted code")]
    DetachSkippedThread(Pid),
    #[error("Maps reader error")]
    MapsReaderError(#[from] MapsReaderError),
    #[error("Failed to get PAGE_SIZE from system")]
    SysConfError(
        #[from]
        #[serde(serialize_with = "serialize_nix_error")]
        nix::Error,
    ),

    #[error("No mapping for stack pointer found")]
    NoStackPointerMapping,
    #[error("Failed slice conversion")]
    TryFromSliceError(
        #[from]
        #[serde(skip)]
        std::array::TryFromSliceError,
    ),
    #[error("Couldn't parse as ELF file")]
    ELFParsingFailed(
        #[from]
        #[serde(serialize_with = "serialize_goblin_error")]
        goblin::error::Error,
    ),
    #[error("Could not read value from module")]
    ModuleReaderError(#[from] ModuleReaderError),
    #[error("Not safe to open mapping: {}", .0.to_string_lossy())]
    NotSafeToOpenMapping(OsString),
    #[error("Failed integer conversion")]
    TryFromIntError(
        #[from]
        #[serde(skip)]
        std::num::TryFromIntError,
    ),
}

#[derive(Debug, Error, serde::Serialize)]
pub enum InitError {
    #[error("failed to read auxv")]
    ReadAuxvFailed(#[source] super::super::auxv::AuxvError),
    #[error("IO error for file {0}")]
    IOError(
        String,
        #[source]
        #[serde(serialize_with = "serialize_io_error")]
        std::io::Error,
    ),
    #[cfg(target_os = "android")]
    #[error("Failed Android specific late init")]
    AndroidLateInitError(#[from] AndroidError),
    #[error("Failed to read the page size")]
    PageSizeError(
        #[from]
        #[serde(serialize_with = "serialize_nix_error")]
        nix::Error,
    ),
    #[error("Ptrace does not function within the same process")]
    CannotPtraceSameProcess,
    #[error("Failed to stop the target process")]
    StopProcessFailed(#[source] StopProcessError),
    #[error("Errors occurred while filling missing Auxv info")]
    FillMissingAuxvInfoErrors(#[source] ErrorList<AuxvError>),
    #[error("Failed filling missing Auxv info")]
    FillMissingAuxvInfoFailed(#[source] AuxvError),
    #[error("Failed reading proc/pid/task entry for process")]
    ReadProcessThreadEntryFailed(
        #[source]
        #[serde(serialize_with = "serialize_io_error")]
        std::io::Error,
    ),
    #[error("Process task entry `{0:?}` could not be parsed as a TID")]
    ProcessTaskEntryNotTid(OsString),
    #[error("Failed to read thread name")]
    ReadThreadNameFailed(
        #[source]
        #[serde(serialize_with = "serialize_io_error")]
        std::io::Error,
    ),
    #[error("Proc task directory `{0:?}` is not a directory")]
    ProcPidTaskNotDirectory(String),
    #[error("Errors while enumerating threads")]
    EnumerateThreadsErrors(#[source] ErrorList<InitError>),
    #[error("Failed to enumerate threads")]
    EnumerateThreadsFailed(#[source] Box<InitError>),
    #[error("Failed to read process map file")]
    ReadProcessMapFileFailed(
        #[source]
        #[serde(serialize_with = "serialize_proc_error")]
        ProcError,
    ),
    #[error("Failed to aggregate process mappings")]
    AggregateMappingsFailed(#[source] MapsReaderError),
    #[error("Failed to enumerate process mappings")]
    EnumerateMappingsFailed(#[source] Box<InitError>),
    #[error("Errors occurred while suspending threads")]
    SuspendThreadsErrors(#[source] ErrorList<WriterError>),
    #[error("No threads left to suspend out of {0}")]
    SuspendNoThreadsLeft(usize),
    #[error("Crash thread does not reference principal mapping")]
    PrincipalMappingNotReferenced,
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum StopProcessError {
    #[error("Failed to stop the process")]
    Stop(
        #[from]
        #[serde(serialize_with = "serialize_nix_error")]
        nix::Error,
    ),
    #[error("Failed to get the process state")]
    State(
        #[from]
        #[serde(serialize_with = "serialize_proc_error")]
        ProcError,
    ),
    #[error("Timeout waiting for process to stop")]
    Timeout,
}

#[derive(Debug, thiserror::Error)]
pub enum ContinueProcessError {
    #[error("Failed to continue the process")]
    Continue(#[from] Errno),
}
