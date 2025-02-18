use std::io;
use std::io::Read;
use windows::Win32::System::SystemServices::{MAILSLOT_WAIT_FOREVER, SECURITY_DESCRIPTOR_REVISION};
use windows::{
    core::*, Win32::Foundation::*, Win32::Security::*, Win32::Storage::FileSystem::*,
    Win32::System::Mailslots::*,
};

pub struct Mailslot {
    handle: HANDLE,
}
unsafe impl Send for Mailslot {}

impl Mailslot {
    pub fn new(name: &str) -> io::Result<Self> {
        let sd = windows::Win32::Security::PSECURITY_DESCRIPTOR::default();
        unsafe {
            InitializeSecurityDescriptor(sd, SECURITY_DESCRIPTOR_REVISION)?;
            SetSecurityDescriptorDacl(sd, true, None, false)?;
        }
        // TODO(raggi); deinit. scopeguard maybe?

        let sa = SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: sd.0,
            bInheritHandle: FALSE,
        };

        let mailslot_path = format!("\\\\.\\mailslot\\{}", name);
        let handle = unsafe {
            CreateMailslotW(
                &HSTRING::from(mailslot_path),
                1024, // max message size 1024, matches Linux datagram buf size
                MAILSLOT_WAIT_FOREVER,
                Some(&sa),
            )?
        };

        if handle == INVALID_HANDLE_VALUE {
            return Err(std::io::Error::last_os_error());
        }

        Ok(Mailslot { handle })
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read(buf)
    }
}

impl io::Read for Mailslot {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut len: u32 = buf.len() as u32;
        unsafe { ReadFile(self.handle, Some(buf), Some(&mut len), None) }
            .map(|_| len as usize)
            .map_err(|e| e.into())
    }
}

impl Drop for Mailslot {
    fn drop(&mut self) {
        if self.handle.is_invalid() {
            return;
        }
        unsafe {
            let _ = windows::Win32::Foundation::CloseHandle(self.handle);
        }
    }
}
