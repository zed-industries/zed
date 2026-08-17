use std::{ffi::OsString, io};

use windows_sys::Win32::System::SystemInformation::{
    GetComputerNameExW, COMPUTER_NAME_FORMAT,
};

/// The type of name to be retrieved by [`get_computer_name`].
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum ComputerNameKind {
    /// The name of the DNS domain assigned to the local computer. If the local
    /// computer is a node in a cluster, lpBuffer receives the DNS domain name
    /// of the cluster virtual server.
    DnsDomain,
    /// The fully qualified DNS name that uniquely identifies the local
    /// computer. This name is a combination of the DNS host name and the DNS
    /// domain name, using the form HostName.DomainName. If the local computer
    /// is a node in a cluster, lpBuffer receives the fully qualified DNS name
    /// of the cluster virtual server.
    DnsFullyQualified,
    /// The DNS host name of the local computer. If the local computer is a
    /// node in a cluster, lpBuffer receives the DNS host name of the cluster
    /// virtual server.
    DnsHostname,
    /// The NetBIOS name of the local computer. If the local computer is a node
    /// in a cluster, lpBuffer receives the NetBIOS name of the cluster virtual
    /// server.
    NetBios,
    /// The name of the DNS domain assigned to the local computer. If the local
    /// computer is a node in a cluster, lpBuffer receives the DNS domain name
    /// of the local computer, not the name of the cluster virtual server.
    PhysicalDnsDomain,
    /// The fully qualified DNS name that uniquely identifies the computer. If
    /// the local computer is a node in a cluster, lpBuffer receives the fully
    /// qualified DNS name of the local computer, not the name of the cluster
    /// virtual server.
    ///
    /// The fully qualified DNS name is a combination of the DNS host name and
    /// the DNS domain name, using the form HostName.DomainName.
    PhysicalDnsFullyQualified,
    /// The DNS host name of the local computer. If the local computer is a
    /// node in a cluster, lpBuffer receives the DNS host name of the local
    /// computer, not the name of the cluster virtual server.
    PhysicalDnsHostname,
    /// The NetBIOS name of the local computer. If the local computer is a node
    /// in a cluster, lpBuffer receives the NetBIOS name of the local computer,
    /// not the name of the cluster virtual server.
    PhysicalNetBios,
}

impl ComputerNameKind {
    fn to_format(&self) -> COMPUTER_NAME_FORMAT {
        use self::ComputerNameKind::*;
        use windows_sys::Win32::System::SystemInformation;

        match *self {
            DnsDomain => SystemInformation::ComputerNameDnsDomain,
            DnsFullyQualified => {
                SystemInformation::ComputerNameDnsFullyQualified
            }
            DnsHostname => SystemInformation::ComputerNameDnsHostname,
            NetBios => SystemInformation::ComputerNameNetBIOS,
            PhysicalDnsDomain => {
                SystemInformation::ComputerNamePhysicalDnsDomain
            }
            PhysicalDnsFullyQualified => {
                SystemInformation::ComputerNamePhysicalDnsFullyQualified
            }
            PhysicalDnsHostname => {
                SystemInformation::ComputerNamePhysicalDnsHostname
            }
            PhysicalNetBios => SystemInformation::ComputerNamePhysicalNetBIOS,
        }
    }
}
/// Retrieves a NetBIOS or DNS name associated with the local computer.
///
/// The names are established at system startup, when the system reads them
/// from the registry.
///
/// This corresponds to calling [`GetComputerNameExW`].
///
/// [`GetComputerNameExW`]: https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getcomputernameexw
pub fn get_computer_name(kind: ComputerNameKind) -> io::Result<OsString> {
    use std::os::windows::ffi::OsStringExt;

    let format = kind.to_format();
    let mut len1 = 0;
    // SAFETY: As documented, we call this with a null pointer which will in
    // turn cause this routine to write the required buffer size fo `len1`.
    // Also, we explicitly ignore the return value since we expect this call to
    // fail given that the destination buffer is too small by design.
    let _ =
        unsafe { GetComputerNameExW(format, std::ptr::null_mut(), &mut len1) };

    let len = match usize::try_from(len1) {
        Ok(len) => len,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "GetComputerNameExW buffer length overflowed usize",
            ))
        }
    };
    let mut buf = vec![0; len];
    let mut len2 = len1;
    // SAFETY: We pass a valid pointer to an appropriately sized Vec<u16>.
    let rc =
        unsafe { GetComputerNameExW(format, buf.as_mut_ptr(), &mut len2) };
    if rc == 0 {
        return Err(io::Error::last_os_error());
    }
    // Apparently, the subsequent call writes the number of characters written
    // to the buffer to `len2` but not including the NUL terminator. Notice
    // that in the first call above, the length written to `len1` *does*
    // include the NUL terminator. Therefore, we expect `len1` to be at least
    // one greater than `len2`. If not, then something weird has happened and
    // we report an error.
    if len1 <= len2 {
        let msg = format!(
            "GetComputerNameExW buffer length mismatch, \
             expected length strictly less than {} \
             but got {}",
            len1, len2,
        );
        return Err(io::Error::new(io::ErrorKind::Other, msg));
    }
    let len = usize::try_from(len2).expect("len1 fits implies len2 fits");
    Ok(OsString::from_wide(&buf[..len]))
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test doesn't really check anything other than that we can
    // successfully query all kinds of computer names. We just print them out
    // since there aren't really any properties about the names that we can
    // assert.
    //
    // We specifically run this test in CI with --nocapture so that we can see
    // the output.
    #[test]
    fn itworks() {
        let kinds = [
            ComputerNameKind::DnsDomain,
            ComputerNameKind::DnsFullyQualified,
            ComputerNameKind::DnsHostname,
            ComputerNameKind::NetBios,
            ComputerNameKind::PhysicalDnsDomain,
            ComputerNameKind::PhysicalDnsFullyQualified,
            ComputerNameKind::PhysicalDnsHostname,
            ComputerNameKind::PhysicalNetBios,
        ];
        for kind in kinds {
            let result = get_computer_name(kind);
            let name = result.unwrap();
            println!("{kind:?}: {name:?}");
        }
    }
}
