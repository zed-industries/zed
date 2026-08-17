#[cfg(target_os = "illumos")]
use std::convert::TryInto;
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "illumos",
    target_os = "hurd",
))]
use std::env;
use std::{
    ffi::{c_void, CStr, OsString},
    fs,
    io::{Error, ErrorKind},
    mem,
    os::{
        raw::{c_char, c_int},
        unix::ffi::OsStringExt,
    },
    slice,
};
#[cfg(target_os = "macos")]
use std::{
    os::{
        raw::{c_long, c_uchar},
        unix::ffi::OsStrExt,
    },
    ptr::null_mut,
};

use crate::{
    os::{Os, Target},
    Arch, DesktopEnv, Platform, Result,
};

#[cfg(any(target_os = "linux", target_os = "hurd"))]
#[repr(C)]
struct PassWd {
    pw_name: *const c_void,
    pw_passwd: *const c_void,
    pw_uid: u32,
    pw_gid: u32,
    pw_gecos: *const c_void,
    pw_dir: *const c_void,
    pw_shell: *const c_void,
}

#[cfg(any(
    target_os = "macos",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
))]
#[repr(C)]
struct PassWd {
    pw_name: *const c_void,
    pw_passwd: *const c_void,
    pw_uid: u32,
    pw_gid: u32,
    pw_change: isize,
    pw_class: *const c_void,
    pw_gecos: *const c_void,
    pw_dir: *const c_void,
    pw_shell: *const c_void,
    pw_expire: isize,
    pw_fields: i32,
}

#[cfg(target_os = "illumos")]
#[repr(C)]
struct PassWd {
    pw_name: *const c_void,
    pw_passwd: *const c_void,
    pw_uid: u32,
    pw_gid: u32,
    pw_age: *const c_void,
    pw_comment: *const c_void,
    pw_gecos: *const c_void,
    pw_dir: *const c_void,
    pw_shell: *const c_void,
}

#[cfg(target_os = "illumos")]
extern "system" {
    fn getpwuid_r(
        uid: u32,
        pwd: *mut PassWd,
        buf: *mut c_void,
        buflen: c_int,
    ) -> *mut PassWd;
}

#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "hurd",
))]
extern "system" {
    fn getpwuid_r(
        uid: u32,
        pwd: *mut PassWd,
        buf: *mut c_void,
        buflen: usize,
        result: *mut *mut PassWd,
    ) -> i32;
}

extern "system" {
    fn geteuid() -> u32;
    fn gethostname(name: *mut c_void, len: usize) -> i32;
}

#[cfg(target_os = "macos")]
#[link(name = "CoreFoundation", kind = "framework")]
extern "system" {
    fn CFStringGetCString(
        the_string: *mut c_void,
        buffer: *mut u8,
        buffer_size: c_long,
        encoding: u32,
    ) -> c_uchar;
    fn CFStringGetLength(the_string: *mut c_void) -> c_long;
    fn CFStringGetMaximumSizeForEncoding(
        length: c_long,
        encoding: u32,
    ) -> c_long;
    fn CFRelease(cf: *const c_void);
}

#[cfg(target_os = "macos")]
#[link(name = "SystemConfiguration", kind = "framework")]
extern "system" {
    fn SCDynamicStoreCopyComputerName(
        store: *mut c_void,
        encoding: *mut u32,
    ) -> *mut c_void;
}

enum Name {
    User,
    Real,
}

unsafe fn strlen(cs: *const c_void) -> usize {
    let mut len = 0;
    let mut cs: *const u8 = cs.cast();
    while *cs != 0 {
        len += 1;
        cs = cs.offset(1);
    }
    len
}

unsafe fn strlen_gecos(cs: *const c_void) -> usize {
    let mut len = 0;
    let mut cs: *const u8 = cs.cast();
    while *cs != 0 && *cs != b',' {
        len += 1;
        cs = cs.offset(1);
    }
    len
}

fn os_from_cstring_gecos(string: *const c_void) -> Result<OsString> {
    if string.is_null() {
        return Err(super::err_null_record());
    }

    // Get a byte slice of the c string.
    let slice = unsafe {
        let length = strlen_gecos(string);

        if length == 0 {
            return Err(super::err_empty_record());
        }

        slice::from_raw_parts(string.cast(), length)
    };

    // Turn byte slice into Rust String.
    Ok(OsString::from_vec(slice.to_vec()))
}

fn os_from_cstring(string: *const c_void) -> Result<OsString> {
    if string.is_null() {
        return Err(super::err_null_record());
    }

    // Get a byte slice of the c string.
    let slice = unsafe {
        let length = strlen(string);

        if length == 0 {
            return Err(super::err_empty_record());
        }

        slice::from_raw_parts(string.cast(), length)
    };

    // Turn byte slice into Rust String.
    Ok(OsString::from_vec(slice.to_vec()))
}

#[cfg(target_os = "macos")]
fn os_from_cfstring(string: *mut c_void) -> OsString {
    if string.is_null() {
        return "".to_string().into();
    }

    unsafe {
        let len = CFStringGetLength(string);
        let capacity =
            CFStringGetMaximumSizeForEncoding(len, 134_217_984 /* UTF8 */) + 1;
        let mut out = Vec::with_capacity(capacity as usize);
        if CFStringGetCString(
            string,
            out.as_mut_ptr(),
            capacity,
            134_217_984, /* UTF8 */
        ) != 0
        {
            out.set_len(strlen(out.as_ptr().cast())); // Remove trailing NUL byte
            out.shrink_to_fit();
            CFRelease(string);
            OsString::from_vec(out)
        } else {
            CFRelease(string);
            "".to_string().into()
        }
    }
}

// This function must allocate, because a slice or `Cow<OsStr>` would still
// reference `passwd` which is dropped when this function returns.
#[inline(always)]
fn getpwuid(name: Name) -> Result<OsString> {
    const BUF_SIZE: usize = 16_384; // size from the man page
    let mut buffer = mem::MaybeUninit::<[u8; BUF_SIZE]>::uninit();
    let mut passwd = mem::MaybeUninit::<PassWd>::uninit();

    // Get PassWd `struct`.
    let passwd = unsafe {
        #[cfg(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "hurd",
        ))]
        {
            let mut _passwd = mem::MaybeUninit::<*mut PassWd>::uninit();
            let ret = getpwuid_r(
                geteuid(),
                passwd.as_mut_ptr(),
                buffer.as_mut_ptr() as *mut c_void,
                BUF_SIZE,
                _passwd.as_mut_ptr(),
            );

            if ret != 0 {
                return Err(Error::last_os_error());
            }

            let _passwd = _passwd.assume_init();

            if _passwd.is_null() {
                return Err(super::err_null_record());
            }
            passwd.assume_init()
        }

        #[cfg(target_os = "illumos")]
        {
            let ret = getpwuid_r(
                geteuid(),
                passwd.as_mut_ptr(),
                buffer.as_mut_ptr() as *mut c_void,
                BUF_SIZE.try_into().unwrap_or(c_int::MAX),
            );

            if ret.is_null() {
                return Err(Error::last_os_error());
            }
            passwd.assume_init()
        }
    };

    // Extract names.
    if let Name::Real = name {
        os_from_cstring_gecos(passwd.pw_gecos)
    } else {
        os_from_cstring(passwd.pw_name)
    }
}

#[cfg(target_os = "macos")]
fn distro_xml(data: String) -> Result<String> {
    let mut product_name = None;
    let mut user_visible_version = None;

    if let Some(start) = data.find("<dict>") {
        if let Some(end) = data.find("</dict>") {
            let mut set_product_name = false;
            let mut set_user_visible_version = false;

            for line in data[start + "<dict>".len()..end].lines() {
                let line = line.trim();

                if line.starts_with("<key>") {
                    match line["<key>".len()..].trim_end_matches("</key>") {
                        "ProductName" => set_product_name = true,
                        "ProductUserVisibleVersion" => {
                            set_user_visible_version = true
                        }
                        "ProductVersion" => {
                            if user_visible_version.is_none() {
                                set_user_visible_version = true
                            }
                        }
                        _ => {}
                    }
                } else if line.starts_with("<string>") {
                    if set_product_name {
                        product_name = Some(
                            line["<string>".len()..]
                                .trim_end_matches("</string>"),
                        );
                        set_product_name = false;
                    } else if set_user_visible_version {
                        user_visible_version = Some(
                            line["<string>".len()..]
                                .trim_end_matches("</string>"),
                        );
                        set_user_visible_version = false;
                    }
                }
            }
        }
    }

    Ok(if let Some(product_name) = product_name {
        if let Some(user_visible_version) = user_visible_version {
            format!("{} {}", product_name, user_visible_version)
        } else {
            product_name.to_string()
        }
    } else {
        user_visible_version
            .map(|v| format!("Mac OS (Unknown) {}", v))
            .ok_or_else(|| {
                Error::new(ErrorKind::InvalidData, "Parsing failed")
            })?
    })
}

#[cfg(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
))]
#[repr(C)]
struct UtsName {
    sysname: [c_char; 256],
    nodename: [c_char; 256],
    release: [c_char; 256],
    version: [c_char; 256],
    machine: [c_char; 256],
}

#[cfg(target_os = "illumos")]
#[repr(C)]
struct UtsName {
    sysname: [c_char; 257],
    nodename: [c_char; 257],
    release: [c_char; 257],
    version: [c_char; 257],
    machine: [c_char; 257],
}

#[cfg(target_os = "dragonfly")]
#[repr(C)]
struct UtsName {
    sysname: [c_char; 32],
    nodename: [c_char; 32],
    release: [c_char; 32],
    version: [c_char; 32],
    machine: [c_char; 32],
}

#[cfg(any(target_os = "linux", target_os = "android",))]
#[repr(C)]
struct UtsName {
    sysname: [c_char; 65],
    nodename: [c_char; 65],
    release: [c_char; 65],
    version: [c_char; 65],
    machine: [c_char; 65],
    domainname: [c_char; 65],
}

#[cfg(target_os = "hurd")]
#[repr(C)]
struct UtsName {
    sysname: [c_char; 1024],
    nodename: [c_char; 1024],
    release: [c_char; 1024],
    version: [c_char; 1024],
    machine: [c_char; 1024],
}

// Buffer initialization
impl Default for UtsName {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[inline(always)]
unsafe fn uname(buf: *mut UtsName) -> c_int {
    extern "C" {
        #[cfg(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "dragonfly",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "illumos",
            target_os = "hurd",
        ))]
        fn uname(buf: *mut UtsName) -> c_int;

        #[cfg(target_os = "freebsd")]
        fn __xuname(nmln: c_int, buf: *mut c_void) -> c_int;
    }

    // Polyfill `uname()` for FreeBSD
    #[inline(always)]
    #[cfg(target_os = "freebsd")]
    unsafe extern "C" fn uname(buf: *mut UtsName) -> c_int {
        __xuname(256, buf.cast())
    }

    uname(buf)
}

impl Target for Os {
    fn langs(self) -> Result<String> {
        super::unix_lang()
    }

    fn realname(self) -> Result<OsString> {
        getpwuid(Name::Real)
    }

    fn username(self) -> Result<OsString> {
        getpwuid(Name::User)
    }

    fn devicename(self) -> Result<OsString> {
        #[cfg(target_os = "macos")]
        {
            let out = os_from_cfstring(unsafe {
                SCDynamicStoreCopyComputerName(null_mut(), null_mut())
            });

            if out.as_bytes().is_empty() {
                return Err(super::err_empty_record());
            }

            Ok(out)
        }

        #[cfg(target_os = "illumos")]
        {
            let mut nodename = fs::read("/etc/nodename")?;

            // Remove all at and after the first newline (before end of file)
            if let Some(slice) = nodename.split(|x| *x == b'\n').next() {
                nodename.drain(slice.len()..);
            }

            if nodename.is_empty() {
                return Err(super::err_empty_record());
            }

            Ok(OsString::from_vec(nodename))
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "hurd",
        ))]
        {
            let machine_info = fs::read("/etc/machine-info")?;

            for i in machine_info.split(|b| *b == b'\n') {
                let mut j = i.split(|b| *b == b'=');

                if j.next() == Some(b"PRETTY_HOSTNAME") {
                    let pretty_hostname = j.next().ok_or(Error::new(
                        ErrorKind::InvalidData,
                        "parsing failed",
                    ))?;
                    let pretty_hostname = if pretty_hostname.starts_with(b"\"")
                        && pretty_hostname.ends_with(b"\"")
                    {
                        &pretty_hostname[1..pretty_hostname.len() - 1]
                    } else {
                        pretty_hostname
                    };
                    let pretty_hostname = {
                        let mut vec = Vec::with_capacity(pretty_hostname.len());
                        let mut pretty_hostname = pretty_hostname.iter();

                        while let Some(&c) = pretty_hostname.next() {
                            if c == b'\\' {
                                vec.push(match pretty_hostname.next() {
                                    Some(b'\\') => b'\\',
                                    Some(b't') => b'\t',
                                    Some(b'r') => b'\r',
                                    Some(b'n') => b'\n',
                                    Some(b'\'') => b'\'',
                                    Some(b'"') => b'"',
                                    _ => {
                                        return Err(Error::new(
                                            ErrorKind::InvalidData,
                                            "parsing failed",
                                        ));
                                    }
                                });
                            } else {
                                vec.push(c);
                            }
                        }

                        vec
                    };

                    return Ok(OsString::from_vec(pretty_hostname));
                }
            }

            Err(super::err_missing_record())
        }
    }

    fn hostname(self) -> Result<String> {
        // Maximum hostname length = 255, plus a NULL byte.
        let mut string = Vec::<u8>::with_capacity(256);

        unsafe {
            if gethostname(string.as_mut_ptr().cast(), 255) == -1 {
                return Err(Error::last_os_error());
            }

            string.set_len(strlen(string.as_ptr().cast()));
        };

        String::from_utf8(string).map_err(|_| {
            Error::new(ErrorKind::InvalidData, "Hostname not valid UTF-8")
        })
    }

    fn distro(self) -> Result<String> {
        #[cfg(target_os = "macos")]
        {
            if let Ok(data) = fs::read_to_string(
                "/System/Library/CoreServices/ServerVersion.plist",
            ) {
                distro_xml(data)
            } else if let Ok(data) = fs::read_to_string(
                "/System/Library/CoreServices/SystemVersion.plist",
            ) {
                distro_xml(data)
            } else {
                Err(super::err_missing_record())
            }
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "illumos",
            target_os = "hurd",
        ))]
        {
            let program = fs::read("/etc/os-release")?;
            let distro = String::from_utf8_lossy(&program);
            let err = || Error::new(ErrorKind::InvalidData, "Parsing failed");
            let mut fallback = None;

            for i in distro.split('\n') {
                let mut j = i.split('=');

                match j.next().ok_or_else(err)? {
                    "PRETTY_NAME" => {
                        return Ok(j
                            .next()
                            .ok_or_else(err)?
                            .trim_matches('"')
                            .to_string());
                    }
                    "NAME" => {
                        fallback = Some(
                            j.next()
                                .ok_or_else(err)?
                                .trim_matches('"')
                                .to_string(),
                        )
                    }
                    _ => {}
                }
            }

            fallback.ok_or_else(err)
        }
    }

    fn desktop_env(self) -> DesktopEnv {
        #[cfg(target_os = "macos")]
        let env = "Aqua";

        // FIXME: WhoAmI 2.0: use `let else`
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "illumos",
            target_os = "hurd",
        ))]
        let env = env::var_os("DESKTOP_SESSION");
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "illumos",
            target_os = "hurd",
        ))]
        let env = if let Some(ref env) = env {
            env.to_string_lossy()
        } else {
            return DesktopEnv::Unknown("Unknown".to_string());
        };

        if env.eq_ignore_ascii_case("AQUA") {
            DesktopEnv::Aqua
        } else if env.eq_ignore_ascii_case("GNOME") {
            DesktopEnv::Gnome
        } else if env.eq_ignore_ascii_case("LXDE") {
            DesktopEnv::Lxde
        } else if env.eq_ignore_ascii_case("OPENBOX") {
            DesktopEnv::Openbox
        } else if env.eq_ignore_ascii_case("I3") {
            DesktopEnv::I3
        } else if env.eq_ignore_ascii_case("UBUNTU") {
            DesktopEnv::Ubuntu
        } else if env.eq_ignore_ascii_case("PLASMA5") {
            DesktopEnv::Kde
        } else {
            DesktopEnv::Unknown(env.to_string())
        }
    }

    #[inline(always)]
    fn platform(self) -> Platform {
        #[cfg(target_os = "linux")]
        {
            Platform::Linux
        }

        #[cfg(target_os = "macos")]
        {
            Platform::MacOS
        }

        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        ))]
        {
            Platform::Bsd
        }

        #[cfg(target_os = "illumos")]
        {
            Platform::Illumos
        }

        #[cfg(target_os = "hurd")]
        {
            Platform::Hurd
        }
    }

    #[inline(always)]
    fn arch(self) -> Result<Arch> {
        let mut buf = UtsName::default();

        if unsafe { uname(&mut buf) } == -1 {
            return Err(Error::last_os_error());
        }

        let arch_str =
            unsafe { CStr::from_ptr(buf.machine.as_ptr()) }.to_string_lossy();

        Ok(match arch_str.as_ref() {
            "aarch64" | "arm64" | "aarch64_be" | "armv8b" | "armv8l" => {
                Arch::Arm64
            }
            "armv5" => Arch::ArmV5,
            "armv6" | "arm" => Arch::ArmV6,
            "armv7" => Arch::ArmV7,
            "i386" => Arch::I386,
            "i586" => Arch::I586,
            "i686" | "i686-AT386" => Arch::I686,
            "mips" => Arch::Mips,
            "mipsel" => Arch::MipsEl,
            "mips64" => Arch::Mips64,
            "mips64el" => Arch::Mips64El,
            "powerpc" | "ppc" | "ppcle" => Arch::PowerPc,
            "powerpc64" | "ppc64" | "ppc64le" => Arch::PowerPc64,
            "powerpc64le" => Arch::PowerPc64Le,
            "riscv32" => Arch::Riscv32,
            "riscv64" => Arch::Riscv64,
            "s390x" => Arch::S390x,
            "sparc" => Arch::Sparc,
            "sparc64" => Arch::Sparc64,
            "x86_64" | "amd64" => Arch::X64,
            _ => Arch::Unknown(arch_str.into_owned()),
        })
    }
}
