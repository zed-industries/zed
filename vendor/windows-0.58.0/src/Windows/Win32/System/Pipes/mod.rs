#[inline]
pub unsafe fn CallNamedPipeA<P0>(lpnamedpipename: P0, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CallNamedPipeA(lpnamedpipename : windows_core::PCSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, ntimeout : u32) -> super::super::Foundation:: BOOL);
    CallNamedPipeA(lpnamedpipename.param().abi(), core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, lpbytesread, ntimeout).ok()
}
#[inline]
pub unsafe fn CallNamedPipeW<P0>(lpnamedpipename: P0, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CallNamedPipeW(lpnamedpipename : windows_core::PCWSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, ntimeout : u32) -> super::super::Foundation:: BOOL);
    CallNamedPipeW(lpnamedpipename.param().abi(), core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, lpbytesread, ntimeout)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ConnectNamedPipe<P0>(hnamedpipe: P0, lpoverlapped: Option<*mut super::IO::OVERLAPPED>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ConnectNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE, lpoverlapped : *mut super::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    ConnectNamedPipe(hnamedpipe.param().abi(), core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CreateNamedPipeA<P0>(lpname: P0, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateNamedPipeA(lpname : windows_core::PCSTR, dwopenmode : super::super::Storage::FileSystem:: FILE_FLAGS_AND_ATTRIBUTES, dwpipemode : NAMED_PIPE_MODE, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    let result__ = CreateNamedPipeA(lpname.param().abi(), dwopenmode, dwpipemode, nmaxinstances, noutbuffersize, ninbuffersize, ndefaulttimeout, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())));
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CreateNamedPipeW<P0>(lpname: P0, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateNamedPipeW(lpname : windows_core::PCWSTR, dwopenmode : super::super::Storage::FileSystem:: FILE_FLAGS_AND_ATTRIBUTES, dwpipemode : NAMED_PIPE_MODE, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    CreateNamedPipeW(lpname.param().abi(), dwopenmode, dwpipemode, nmaxinstances, noutbuffersize, ninbuffersize, ndefaulttimeout, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreatePipe(hreadpipe: *mut super::super::Foundation::HANDLE, hwritepipe: *mut super::super::Foundation::HANDLE, lppipeattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, nsize: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn CreatePipe(hreadpipe : *mut super::super::Foundation:: HANDLE, hwritepipe : *mut super::super::Foundation:: HANDLE, lppipeattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, nsize : u32) -> super::super::Foundation:: BOOL);
    CreatePipe(hreadpipe, hwritepipe, core::mem::transmute(lppipeattributes.unwrap_or(std::ptr::null())), nsize).ok()
}
#[inline]
pub unsafe fn DisconnectNamedPipe<P0>(hnamedpipe: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn DisconnectNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    DisconnectNamedPipe(hnamedpipe.param().abi()).ok()
}
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameA<P0>(pipe: P0, clientcomputername: &mut [u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameA(pipe : super::super::Foundation:: HANDLE, clientcomputername : windows_core::PSTR, clientcomputernamelength : u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeClientComputerNameA(pipe.param().abi(), core::mem::transmute(clientcomputername.as_ptr()), clientcomputername.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameW<P0>(pipe: P0, clientcomputername: windows_core::PWSTR, clientcomputernamelength: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameW(pipe : super::super::Foundation:: HANDLE, clientcomputername : windows_core::PWSTR, clientcomputernamelength : u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeClientComputerNameW(pipe.param().abi(), core::mem::transmute(clientcomputername), clientcomputernamelength)
}
#[inline]
pub unsafe fn GetNamedPipeClientProcessId<P0>(pipe: P0, clientprocessid: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeClientProcessId(pipe : super::super::Foundation:: HANDLE, clientprocessid : *mut u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeClientProcessId(pipe.param().abi(), clientprocessid).ok()
}
#[inline]
pub unsafe fn GetNamedPipeClientSessionId<P0>(pipe: P0, clientsessionid: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeClientSessionId(pipe : super::super::Foundation:: HANDLE, clientsessionid : *mut u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeClientSessionId(pipe.param().abi(), clientsessionid).ok()
}
#[inline]
pub unsafe fn GetNamedPipeHandleStateA<P0>(hnamedpipe: P0, lpstate: Option<*mut NAMED_PIPE_MODE>, lpcurinstances: Option<*mut u32>, lpmaxcollectioncount: Option<*mut u32>, lpcollectdatatimeout: Option<*mut u32>, lpusername: Option<&mut [u8]>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateA(hnamedpipe : super::super::Foundation:: HANDLE, lpstate : *mut NAMED_PIPE_MODE, lpcurinstances : *mut u32, lpmaxcollectioncount : *mut u32, lpcollectdatatimeout : *mut u32, lpusername : windows_core::PSTR, nmaxusernamesize : u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeHandleStateA(hnamedpipe.param().abi(), core::mem::transmute(lpstate.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpcurinstances.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpmaxcollectioncount.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpcollectdatatimeout.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpusername.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpusername.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
}
#[inline]
pub unsafe fn GetNamedPipeHandleStateW<P0>(hnamedpipe: P0, lpstate: Option<*mut NAMED_PIPE_MODE>, lpcurinstances: Option<*mut u32>, lpmaxcollectioncount: Option<*mut u32>, lpcollectdatatimeout: Option<*mut u32>, lpusername: Option<&mut [u16]>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateW(hnamedpipe : super::super::Foundation:: HANDLE, lpstate : *mut NAMED_PIPE_MODE, lpcurinstances : *mut u32, lpmaxcollectioncount : *mut u32, lpcollectdatatimeout : *mut u32, lpusername : windows_core::PWSTR, nmaxusernamesize : u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeHandleStateW(hnamedpipe.param().abi(), core::mem::transmute(lpstate.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpcurinstances.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpmaxcollectioncount.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpcollectdatatimeout.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpusername.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpusername.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn GetNamedPipeInfo<P0>(hnamedpipe: P0, lpflags: Option<*mut NAMED_PIPE_MODE>, lpoutbuffersize: Option<*mut u32>, lpinbuffersize: Option<*mut u32>, lpmaxinstances: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeInfo(hnamedpipe : super::super::Foundation:: HANDLE, lpflags : *mut NAMED_PIPE_MODE, lpoutbuffersize : *mut u32, lpinbuffersize : *mut u32, lpmaxinstances : *mut u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeInfo(hnamedpipe.param().abi(), core::mem::transmute(lpflags.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpoutbuffersize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpinbuffersize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpmaxinstances.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn GetNamedPipeServerProcessId<P0>(pipe: P0, serverprocessid: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeServerProcessId(pipe : super::super::Foundation:: HANDLE, serverprocessid : *mut u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeServerProcessId(pipe.param().abi(), serverprocessid).ok()
}
#[inline]
pub unsafe fn GetNamedPipeServerSessionId<P0>(pipe: P0, serversessionid: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeServerSessionId(pipe : super::super::Foundation:: HANDLE, serversessionid : *mut u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeServerSessionId(pipe.param().abi(), serversessionid).ok()
}
#[inline]
pub unsafe fn ImpersonateNamedPipeClient<P0>(hnamedpipe: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn ImpersonateNamedPipeClient(hnamedpipe : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    ImpersonateNamedPipeClient(hnamedpipe.param().abi()).ok()
}
#[inline]
pub unsafe fn PeekNamedPipe<P0>(hnamedpipe: P0, lpbuffer: Option<*mut core::ffi::c_void>, nbuffersize: u32, lpbytesread: Option<*mut u32>, lptotalbytesavail: Option<*mut u32>, lpbytesleftthismessage: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn PeekNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE, lpbuffer : *mut core::ffi::c_void, nbuffersize : u32, lpbytesread : *mut u32, lptotalbytesavail : *mut u32, lpbytesleftthismessage : *mut u32) -> super::super::Foundation:: BOOL);
    PeekNamedPipe(hnamedpipe.param().abi(), core::mem::transmute(lpbuffer.unwrap_or(std::ptr::null_mut())), nbuffersize, core::mem::transmute(lpbytesread.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lptotalbytesavail.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpbytesleftthismessage.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn SetNamedPipeHandleState<P0>(hnamedpipe: P0, lpmode: Option<*const NAMED_PIPE_MODE>, lpmaxcollectioncount: Option<*const u32>, lpcollectdatatimeout: Option<*const u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetNamedPipeHandleState(hnamedpipe : super::super::Foundation:: HANDLE, lpmode : *const NAMED_PIPE_MODE, lpmaxcollectioncount : *const u32, lpcollectdatatimeout : *const u32) -> super::super::Foundation:: BOOL);
    SetNamedPipeHandleState(hnamedpipe.param().abi(), core::mem::transmute(lpmode.unwrap_or(std::ptr::null())), core::mem::transmute(lpmaxcollectioncount.unwrap_or(std::ptr::null())), core::mem::transmute(lpcollectdatatimeout.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn TransactNamedPipe<P0>(hnamedpipe: P0, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesread: *mut u32, lpoverlapped: Option<*mut super::IO::OVERLAPPED>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn TransactNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, lpoverlapped : *mut super::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    TransactNamedPipe(hnamedpipe.param().abi(), core::mem::transmute(lpinbuffer.unwrap_or(std::ptr::null())), ninbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), noutbuffersize, lpbytesread, core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn WaitNamedPipeA<P0>(lpnamedpipename: P0, ntimeout: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WaitNamedPipeA(lpnamedpipename : windows_core::PCSTR, ntimeout : u32) -> super::super::Foundation:: BOOL);
    WaitNamedPipeA(lpnamedpipename.param().abi(), ntimeout).ok()
}
#[inline]
pub unsafe fn WaitNamedPipeW<P0>(lpnamedpipename: P0, ntimeout: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WaitNamedPipeW(lpnamedpipename : windows_core::PCWSTR, ntimeout : u32) -> super::super::Foundation:: BOOL);
    WaitNamedPipeW(lpnamedpipename.param().abi(), ntimeout)
}
pub const NMPWAIT_NOWAIT: u32 = 1u32;
pub const NMPWAIT_USE_DEFAULT_WAIT: u32 = 0u32;
pub const NMPWAIT_WAIT_FOREVER: u32 = 4294967295u32;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_CLIENT_END: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_NOWAIT: NAMED_PIPE_MODE = NAMED_PIPE_MODE(1u32);
pub const PIPE_READMODE_BYTE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_READMODE_MESSAGE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(2u32);
pub const PIPE_REJECT_REMOTE_CLIENTS: NAMED_PIPE_MODE = NAMED_PIPE_MODE(8u32);
pub const PIPE_SERVER_END: NAMED_PIPE_MODE = NAMED_PIPE_MODE(1u32);
pub const PIPE_TYPE_BYTE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_TYPE_MESSAGE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(4u32);
pub const PIPE_UNLIMITED_INSTANCES: u32 = 255u32;
pub const PIPE_WAIT: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NAMED_PIPE_MODE(pub u32);
impl windows_core::TypeKind for NAMED_PIPE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NAMED_PIPE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NAMED_PIPE_MODE").field(&self.0).finish()
    }
}
impl NAMED_PIPE_MODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NAMED_PIPE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NAMED_PIPE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NAMED_PIPE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NAMED_PIPE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NAMED_PIPE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
