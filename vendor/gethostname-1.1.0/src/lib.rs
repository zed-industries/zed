// Copyright Sebastian Wiesner <sebastian@swsnr.de>

// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at

// 	http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

//! [gethostname()][ghn] for all platforms.
//!
//! ```
//! use gethostname::gethostname;
//!
//! println!("Hostname: {:?}", gethostname());
//! ```
//!
//! [ghn]: http://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html

#![deny(warnings, missing_docs, clippy::all, clippy::pedantic)]
// On the Windows target our doc links don't necessarily work.
#![cfg_attr(windows, allow(rustdoc::broken_intra_doc_links))]

use std::ffi::OsString;

/// Get the standard host name for the current machine.
///
/// On Unix call [`rustix::system::uname`] to obtain the node name, see
/// [`rustix::system::Uname::nodename`].
///
/// On Windows return the DNS host name of the local computer, as returned by
/// [GetComputerNameExW] with `ComputerNamePhysicalDnsHostname` as `NameType`.
/// We call this function twice to obtain the appropriate buffer size; there is
/// a race condition window between these two calls where a change to the node
/// name would result in a wrong buffer size which could cause this function to
/// panic.
///
/// Note that this host name does not have a well-defined meaning in terms of
/// network name resolution.  Specifically, it's not guaranteed that the
/// returned name can be resolved in any particular way, e.g. DNS.
///
/// [GetComputerNameExW]: https://docs.microsoft.com/en-us/windows/desktop/api/sysinfoapi/nf-sysinfoapi-getcomputernameexw
#[must_use]
pub fn gethostname() -> OsString {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        OsString::from_vec(rustix::system::uname().nodename().to_bytes().to_vec())
    }
    #[cfg(windows)]
    {
        get_computer_physical_dns_hostname()
    }
}

#[cfg(windows)]
fn get_computer_physical_dns_hostname() -> OsString {
    use std::os::windows::ffi::OsStringExt;

    // The DNS host name of the local computer. If the local computer is a node
    // in a cluster, lpBuffer receives the DNS host name of the local computer,
    // not the name of the cluster virtual server.
    pub const COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME: i32 = 5;

    // https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getcomputernameexw
    ::windows_link::link!("kernel32.dll" "system" fn GetComputerNameExW(nametype: i32, lpbuffer: *mut u16, nsize: *mut u32) -> i32);

    let mut buffer_size: u32 = 0;

    unsafe {
        // This call always fails with ERROR_MORE_DATA, because we pass NULL to
        // get the required buffer size.  GetComputerNameExW then fills buffer_size with the size
        // of the host name string plus a trailing zero byte.
        GetComputerNameExW(
            COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME,
            std::ptr::null_mut(),
            &mut buffer_size,
        )
    };
    assert!(
        0 < buffer_size,
        "GetComputerNameExW did not provide buffer size"
    );

    let mut buffer = vec![0_u16; buffer_size as usize];
    unsafe {
        if GetComputerNameExW(
            COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0
        {
            panic!(
                "GetComputerNameExW failed to read hostname.
        Please report this issue to <https://codeberg.org/swsnr/gethostname.rs/issues>!"
            );
        }
    }
    assert!(
        // GetComputerNameExW returns the size _without_ the trailing zero byte on the second call
        buffer_size as usize == buffer.len() - 1,
        "GetComputerNameExW changed the buffer size unexpectedly"
    );

    let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
    OsString::from_wide(&buffer[0..end])
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn gethostname_matches_system_hostname() {
        let mut command = if cfg!(windows) {
            Command::new("hostname")
        } else {
            let mut uname = Command::new("uname");
            uname.arg("-n");
            uname
        };
        let output = command.output().expect("failed to get hostname");
        if output.status.success() {
            let hostname = String::from_utf8_lossy(&output.stdout);
            assert!(
                !hostname.is_empty(),
                "Failed to get hostname: hostname empty?"
            );
            // Convert both sides to lowercase; hostnames are case-insensitive
            // anyway.
            assert_eq!(
                super::gethostname().into_string().unwrap().to_lowercase(),
                hostname.trim_end().to_lowercase()
            );
        } else {
            panic!(
                "Failed to get hostname! {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
