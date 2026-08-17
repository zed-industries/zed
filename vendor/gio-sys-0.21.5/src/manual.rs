// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(not(target_family = "windows"))]
pub use self::libc_constants::*;
#[cfg(target_family = "windows")]
pub use self::windows_constants::*;

pub type GSocketFamily = libc::c_int;
pub type GSocketMsgFlags = libc::c_int;

#[cfg(target_family = "windows")]
mod windows_constants {
    pub const G_SOCKET_FAMILY_INVALID: super::GSocketFamily =
        windows_sys::Win32::Networking::WinSock::AF_UNSPEC as super::GSocketFamily;
    pub const G_SOCKET_FAMILY_UNIX: super::GSocketFamily =
        windows_sys::Win32::Networking::WinSock::AF_UNIX as super::GSocketFamily;
    pub const G_SOCKET_FAMILY_IPV4: super::GSocketFamily =
        windows_sys::Win32::Networking::WinSock::AF_INET as super::GSocketFamily;
    pub const G_SOCKET_FAMILY_IPV6: super::GSocketFamily =
        windows_sys::Win32::Networking::WinSock::AF_INET6 as super::GSocketFamily;

    pub const G_SOCKET_MSG_NONE: super::GSocketMsgFlags = 0;
    pub const G_SOCKET_MSG_OOB: super::GSocketMsgFlags =
        windows_sys::Win32::Networking::WinSock::MSG_OOB;
    pub const G_SOCKET_MSG_PEEK: super::GSocketMsgFlags =
        windows_sys::Win32::Networking::WinSock::MSG_PEEK;
    pub const G_SOCKET_MSG_DONTROUTE: super::GSocketMsgFlags =
        windows_sys::Win32::Networking::WinSock::MSG_DONTROUTE;
}

#[cfg(not(target_family = "windows"))]
mod libc_constants {
    pub const G_SOCKET_FAMILY_INVALID: super::GSocketFamily = libc::AF_UNSPEC;
    pub const G_SOCKET_FAMILY_UNIX: super::GSocketFamily = libc::AF_UNIX;
    pub const G_SOCKET_FAMILY_IPV4: super::GSocketFamily = libc::AF_INET;
    pub const G_SOCKET_FAMILY_IPV6: super::GSocketFamily = libc::AF_INET6;

    pub const G_SOCKET_MSG_NONE: super::GSocketMsgFlags = 0;
    pub const G_SOCKET_MSG_OOB: super::GSocketMsgFlags = libc::MSG_OOB;
    pub const G_SOCKET_MSG_PEEK: super::GSocketMsgFlags = libc::MSG_PEEK;
    pub const G_SOCKET_MSG_DONTROUTE: super::GSocketMsgFlags = libc::MSG_DONTROUTE;
}

#[cfg(target_family = "windows")]
pub use self::windows_streams::*;

#[cfg(target_family = "windows")]
mod windows_streams {
    use libc::c_void;

    use crate::{
        gboolean, GInputStream, GInputStreamClass, GOutputStream, GOutputStreamClass, GType,
    };

    extern "C" {
        //=========================================================================
        // GWin32InputStream
        //=========================================================================
        pub fn g_win32_input_stream_get_type() -> GType;
        pub fn g_win32_input_stream_new(
            handle: *mut c_void,
            close_handle: gboolean,
        ) -> *mut GInputStream;
        pub fn g_win32_input_stream_get_close_handle(stream: *mut GWin32InputStream) -> gboolean;
        pub fn g_win32_input_stream_get_handle(stream: *mut GWin32InputStream) -> *mut c_void;
        pub fn g_win32_input_stream_set_close_handle(
            stream: *mut GWin32InputStream,
            close_handle: gboolean,
        );

        //=========================================================================
        // GWin32OutputStream
        //=========================================================================
        pub fn g_win32_output_stream_get_type() -> GType;
        pub fn g_win32_output_stream_new(
            handle: *mut c_void,
            close_handle: gboolean,
        ) -> *mut GOutputStream;
        pub fn g_win32_output_stream_get_close_handle(stream: *mut GWin32OutputStream) -> gboolean;
        pub fn g_win32_output_stream_get_handle(stream: *mut GWin32OutputStream) -> *mut c_void;
        pub fn g_win32_output_stream_set_close_handle(
            stream: *mut GWin32OutputStream,
            close_handle: gboolean,
        );
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GWin32InputStreamClass {
        pub parent_class: GInputStreamClass,
        pub _g_reserved1: Option<unsafe extern "C" fn()>,
        pub _g_reserved2: Option<unsafe extern "C" fn()>,
        pub _g_reserved3: Option<unsafe extern "C" fn()>,
        pub _g_reserved4: Option<unsafe extern "C" fn()>,
        pub _g_reserved5: Option<unsafe extern "C" fn()>,
    }

    impl ::std::fmt::Debug for GWin32InputStreamClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(&format!("GWin32InputStreamClass @ {:?}", self as *const _))
                .field("parent_class", &self.parent_class)
                .field("_g_reserved1", &self._g_reserved1)
                .field("_g_reserved2", &self._g_reserved2)
                .field("_g_reserved3", &self._g_reserved3)
                .field("_g_reserved4", &self._g_reserved4)
                .field("_g_reserved5", &self._g_reserved5)
                .finish()
        }
    }

    #[repr(C)]
    pub struct _GWin32InputStreamPrivate(c_void);

    pub type GWin32InputStreamPrivate = *mut _GWin32InputStreamPrivate;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GWin32InputStream {
        pub parent_instance: GInputStream,
        pub priv_: *mut GWin32InputStreamPrivate,
    }

    impl ::std::fmt::Debug for GWin32InputStream {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(&format!("GWin32InputStream @ {:?}", self as *const _))
                .field("parent_instance", &self.parent_instance)
                .finish()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GWin32OutputStreamClass {
        pub parent_class: GOutputStreamClass,
        pub _g_reserved1: Option<unsafe extern "C" fn()>,
        pub _g_reserved2: Option<unsafe extern "C" fn()>,
        pub _g_reserved3: Option<unsafe extern "C" fn()>,
        pub _g_reserved4: Option<unsafe extern "C" fn()>,
        pub _g_reserved5: Option<unsafe extern "C" fn()>,
    }

    impl ::std::fmt::Debug for GWin32OutputStreamClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(&format!("GWin32OutputStreamClass @ {:?}", self as *const _))
                .field("parent_class", &self.parent_class)
                .field("_g_reserved1", &self._g_reserved1)
                .field("_g_reserved2", &self._g_reserved2)
                .field("_g_reserved3", &self._g_reserved3)
                .field("_g_reserved4", &self._g_reserved4)
                .field("_g_reserved5", &self._g_reserved5)
                .finish()
        }
    }

    #[repr(C)]
    pub struct _GWin32OutputStreamPrivate(c_void);

    pub type GWin32OutputStreamPrivate = *mut _GWin32OutputStreamPrivate;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GWin32OutputStream {
        pub parent_instance: GOutputStream,
        pub priv_: *mut GWin32OutputStreamPrivate,
    }

    impl ::std::fmt::Debug for GWin32OutputStream {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(&format!("GWin32OutputStream @ {:?}", self as *const _))
                .field("parent_instance", &self.parent_instance)
                .finish()
        }
    }
}

#[cfg(not(feature = "v2_84"))]
#[cfg(target_family = "unix")]
mod unix_mount_compat {
    #![allow(clippy::missing_safety_doc)]

    use crate::*;

    pub unsafe fn g_unix_mount_entry_compare(
        mount1: *mut GUnixMountEntry,
        mount2: *mut GUnixMountEntry,
    ) -> c_int {
        g_unix_mount_compare(mount1, mount2)
    }
    pub unsafe fn g_unix_mount_entry_copy(
        mount_entry: *mut GUnixMountEntry,
    ) -> *mut GUnixMountEntry {
        g_unix_mount_copy(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_free(mount_entry: *mut GUnixMountEntry) {
        g_unix_mount_free(mount_entry);
    }
    pub unsafe fn g_unix_mount_entry_get_device_path(
        mount_entry: *mut GUnixMountEntry,
    ) -> *const c_char {
        g_unix_mount_get_device_path(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_get_fs_type(
        mount_entry: *mut GUnixMountEntry,
    ) -> *const c_char {
        g_unix_mount_get_fs_type(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_get_mount_path(
        mount_entry: *mut GUnixMountEntry,
    ) -> *const c_char {
        g_unix_mount_get_mount_path(mount_entry)
    }
    #[cfg(feature = "v2_58")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_58")))]
    pub unsafe fn g_unix_mount_entry_get_options(
        mount_entry: *mut GUnixMountEntry,
    ) -> *const c_char {
        g_unix_mount_get_options(mount_entry)
    }
    #[cfg(feature = "v2_60")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
    pub unsafe fn g_unix_mount_entry_get_root_path(
        mount_entry: *mut GUnixMountEntry,
    ) -> *const c_char {
        g_unix_mount_get_root_path(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_guess_can_eject(
        mount_entry: *mut GUnixMountEntry,
    ) -> gboolean {
        g_unix_mount_guess_can_eject(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_guess_icon(mount_entry: *mut GUnixMountEntry) -> *mut GIcon {
        g_unix_mount_guess_icon(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_guess_name(mount_entry: *mut GUnixMountEntry) -> *mut c_char {
        g_unix_mount_guess_name(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_guess_should_display(
        mount_entry: *mut GUnixMountEntry,
    ) -> gboolean {
        g_unix_mount_guess_should_display(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_guess_symbolic_icon(
        mount_entry: *mut GUnixMountEntry,
    ) -> *mut GIcon {
        g_unix_mount_guess_symbolic_icon(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_is_readonly(mount_entry: *mut GUnixMountEntry) -> gboolean {
        g_unix_mount_is_readonly(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_is_system_internal(
        mount_entry: *mut GUnixMountEntry,
    ) -> gboolean {
        g_unix_mount_is_system_internal(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_at(
        mount_path: *const c_char,
        time_read: *mut u64,
    ) -> *mut GUnixMountEntry {
        g_unix_mount_at(mount_path, time_read)
    }
    pub unsafe fn g_unix_mount_entry_for(
        file_path: *const c_char,
        time_read: *mut u64,
    ) -> *mut GUnixMountEntry {
        g_unix_mount_for(file_path, time_read)
    }

    #[cfg(feature = "v2_82")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_82")))]
    pub unsafe fn g_unix_mount_entries_get_from_file(
        table_path: *const c_char,
        time_read_out: *mut u64,
        n_entries_out: *mut size_t,
    ) -> *mut *mut GUnixMountEntry {
        g_unix_mounts_get_from_file(table_path, time_read_out, n_entries_out)
    }

    pub unsafe fn g_unix_mount_entries_get(time_read: *mut u64) -> *mut glib::GList {
        g_unix_mounts_get(time_read)
    }

    pub unsafe fn g_unix_mount_entries_changed_since(time: u64) -> gboolean {
        g_unix_mounts_changed_since(time)
    }
}

#[cfg(not(feature = "v2_84"))]
#[cfg(target_family = "unix")]
pub use unix_mount_compat::*;
