use crate::{Architecture, BinaryFormat, Environment, OperatingSystem, Triple, Vendor};

// Include the implementations of the `HOST` object containing information
// about the current host.
include!(concat!(env!("OUT_DIR"), "/host.rs"));

#[cfg(test)]
mod tests {
    #[cfg(target_os = "aix")]
    #[test]
    fn test_aix() {
        use super::*;
        assert_eq!(OperatingSystem::host(), OperatingSystem::Aix);
    }

    #[cfg(target_os = "cygwin")]
    #[test]
    fn test_cygwin() {
        use super::*;
        assert_eq!(OperatingSystem::host(), OperatingSystem::Cygwin);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_linux() {
        use super::*;
        assert_eq!(OperatingSystem::host(), OperatingSystem::Linux);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_macos() {
        use super::*;
        assert_eq!(OperatingSystem::host(), OperatingSystem::Darwin(None));
    }

    #[cfg(windows)]
    #[test]
    fn test_windows() {
        use super::*;
        assert_eq!(OperatingSystem::host(), OperatingSystem::Windows);
    }

    #[cfg(target_pointer_width = "16")]
    #[test]
    fn test_ptr16() {
        use super::*;
        assert_eq!(Architecture::host().pointer_width().unwrap().bits(), 16);
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_ptr32() {
        use super::*;
        assert_eq!(Architecture::host().pointer_width().unwrap().bits(), 32);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_ptr64() {
        use super::*;
        assert_eq!(Architecture::host().pointer_width().unwrap().bits(), 64);
    }

    #[test]
    fn host_object() {
        use super::*;
        assert_eq!(HOST, Triple::host());
    }
}
