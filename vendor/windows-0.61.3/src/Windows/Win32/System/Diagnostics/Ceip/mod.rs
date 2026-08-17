#[inline]
pub unsafe fn CeipIsOptedIn() -> windows_core::BOOL {
    windows_link::link!("kernel32.dll" "system" fn CeipIsOptedIn() -> windows_core::BOOL);
    unsafe { CeipIsOptedIn() }
}
