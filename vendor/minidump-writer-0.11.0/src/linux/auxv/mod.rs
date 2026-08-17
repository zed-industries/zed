use {
    self::reader::ProcfsAuxvIter,
    super::Pid,
    crate::serializers::*,
    error_graph::WriteErrorList,
    failspot::failspot,
    std::{fs::File, io::BufReader},
    thiserror::Error,
};

mod reader;

/// The type used in auxv keys and values.
#[cfg(target_pointer_width = "32")]
pub type AuxvType = u32;
/// The type used in auxv keys and values.
#[cfg(target_pointer_width = "64")]
pub type AuxvType = u64;

#[cfg(target_os = "android")]
mod consts {
    use super::AuxvType;
    pub const AT_PHDR: AuxvType = 3;
    pub const AT_PHNUM: AuxvType = 5;
    pub const AT_ENTRY: AuxvType = 9;
    pub const AT_SYSINFO_EHDR: AuxvType = 33;
}
#[cfg(not(target_os = "android"))]
mod consts {
    use super::AuxvType;
    pub const AT_PHDR: AuxvType = libc::AT_PHDR;
    pub const AT_PHNUM: AuxvType = libc::AT_PHNUM;
    pub const AT_ENTRY: AuxvType = libc::AT_ENTRY;
    pub const AT_SYSINFO_EHDR: AuxvType = libc::AT_SYSINFO_EHDR;
}

/// An auxv key-value pair.
#[derive(Debug, PartialEq, Eq)]
pub struct AuxvPair {
    pub key: AuxvType,
    pub value: AuxvType,
}

/// Auxv info that can be passed from crashing process
///
/// Since `/proc/{pid}/auxv` can sometimes be inaccessible, the calling process should prefer to transfer this
/// information directly using the Linux `getauxval()` call (if possible).
///
/// Any field that is set to `0` will be considered unset. In that case, minidump-writer might try other techniques
/// to obtain it (like reading `/proc/{pid}/auxv`).
#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct DirectAuxvDumpInfo {
    /// The value of `getauxval(AT_PHNUM)`
    pub program_header_count: AuxvType,
    /// The value of `getauxval(AT_PHDR)`
    pub program_header_address: AuxvType,
    /// The value of `getauxval(AT_SYSINFO_EHDR)`
    pub linux_gate_address: AuxvType,
    /// The value of `getauxval(AT_ENTRY)`
    pub entry_address: AuxvType,
}

impl From<DirectAuxvDumpInfo> for AuxvDumpInfo {
    fn from(f: DirectAuxvDumpInfo) -> AuxvDumpInfo {
        AuxvDumpInfo {
            program_header_count: (f.program_header_count > 0).then_some(f.program_header_count),
            program_header_address: (f.program_header_address > 0)
                .then_some(f.program_header_address),
            linux_gate_address: (f.linux_gate_address > 0).then_some(f.linux_gate_address),
            entry_address: (f.entry_address > 0).then_some(f.entry_address),
        }
    }
}

#[derive(Debug, Default)]
pub struct AuxvDumpInfo {
    program_header_count: Option<AuxvType>,
    program_header_address: Option<AuxvType>,
    linux_gate_address: Option<AuxvType>,
    entry_address: Option<AuxvType>,
}

impl AuxvDumpInfo {
    pub fn try_filling_missing_info(
        &mut self,
        pid: Pid,
        mut soft_errors: impl WriteErrorList<AuxvError>,
    ) -> Result<(), AuxvError> {
        if self.is_complete() {
            return Ok(());
        }

        let auxv_path = format!("/proc/{pid}/auxv");
        let auxv_file = File::open(&auxv_path).map_err(|e| AuxvError::OpenError(auxv_path, e))?;

        for pair_result in ProcfsAuxvIter::new(BufReader::new(auxv_file)) {
            let AuxvPair { key, value } = match pair_result {
                Ok(pair) => pair,
                Err(e) => {
                    soft_errors.push(e);
                    continue;
                }
            };
            let dest_field = match key {
                consts::AT_PHNUM => &mut self.program_header_count,
                consts::AT_PHDR => &mut self.program_header_address,
                consts::AT_SYSINFO_EHDR => &mut self.linux_gate_address,
                consts::AT_ENTRY => &mut self.entry_address,
                _ => continue,
            };
            if dest_field.is_none() {
                *dest_field = Some(value);
            }
        }

        failspot!(FillMissingAuxvInfo soft_errors.push(AuxvError::InvalidFormat));

        Ok(())
    }
    pub fn get_program_header_count(&self) -> Option<AuxvType> {
        self.program_header_count
    }
    pub fn get_program_header_address(&self) -> Option<AuxvType> {
        self.program_header_address
    }
    pub fn get_linux_gate_address(&self) -> Option<AuxvType> {
        self.linux_gate_address
    }
    pub fn get_entry_address(&self) -> Option<AuxvType> {
        self.entry_address
    }
    pub fn is_complete(&self) -> bool {
        self.program_header_count.is_some()
            && self.program_header_address.is_some()
            && self.linux_gate_address.is_some()
            && self.entry_address.is_some()
    }
}

#[derive(Debug, Error, serde::Serialize)]
pub enum AuxvError {
    #[error("Failed to open file {0}")]
    OpenError(
        String,
        #[source]
        #[serde(serialize_with = "serialize_io_error")]
        std::io::Error,
    ),
    #[error("No auxv entry found for PID {0}")]
    NoAuxvEntryFound(Pid),
    #[error("Invalid auxv format (should not hit EOF before AT_NULL)")]
    InvalidFormat,
    #[error("IO Error")]
    IOError(
        #[from]
        #[serde(serialize_with = "serialize_io_error")]
        std::io::Error,
    ),
}
