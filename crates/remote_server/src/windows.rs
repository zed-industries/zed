use windows::Win32::System::Com::{
    CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, IDispatch,
    IServiceProvider,
};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::Shell::{
    CSIDL_DESKTOP, IShellBrowser, IShellDispatch2, IShellFolderViewDual, IShellWindows,
    SID_STopLevelBrowser, SVGIO_BACKGROUND, SWC_DESKTOP, SWFO_NEEDDISPATCH, ShellWindows,
};
use windows::core::{BSTR, Interface};

pub fn shell_execute_from_explorer(
    file: &str,
    parameters: &str,
    directory: &str,
) -> anyhow::Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap();

        let mut _hwnd = Default::default();
        let shell_dispatch: IShellDispatch2 =
            CoCreateInstance::<_, IShellWindows>(&ShellWindows, None, CLSCTX_LOCAL_SERVER)?
                .FindWindowSW(
                    &VARIANT::from(CSIDL_DESKTOP as i32),
                    &VARIANT::default(),
                    SWC_DESKTOP,
                    &mut _hwnd,
                    SWFO_NEEDDISPATCH,
                )?
                .cast::<IServiceProvider>()?
                .QueryService::<IShellBrowser>(&SID_STopLevelBrowser)?
                .QueryActiveShellView()?
                .GetItemObject::<IDispatch>(SVGIO_BACKGROUND)?
                .cast::<IShellFolderViewDual>()?
                .Application()?
                .cast()?;

        shell_dispatch.ShellExecute(
            &BSTR::from(file),
            &VARIANT::from(parameters),
            &VARIANT::from(directory),
            &VARIANT::from(""),
            &VARIANT::from(0i32),
        )?;

        Ok(())
    }
}
