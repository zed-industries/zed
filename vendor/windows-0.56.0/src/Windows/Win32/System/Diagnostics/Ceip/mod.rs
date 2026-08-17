#[inline]
pub unsafe fn CeipIsOptedIn() -> super::super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn CeipIsOptedIn() -> super::super::super::Foundation:: BOOL);
    CeipIsOptedIn()
}
