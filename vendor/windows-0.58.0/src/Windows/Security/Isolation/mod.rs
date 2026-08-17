#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironment, IIsolatedWindowsEnvironment_Vtbl, 0x41d24597_c328_4467_b37f_4dfc6f60b6bc);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub StartProcessSilentlyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, IsolatedWindowsEnvironmentActivator, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StartProcessSilentlyAsync: usize,
    #[cfg(feature = "deprecated")]
    pub StartProcessSilentlyWithTelemetryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, IsolatedWindowsEnvironmentActivator, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StartProcessSilentlyWithTelemetryAsync: usize,
    #[cfg(feature = "deprecated")]
    pub ShareFolderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShareFolderAsync: usize,
    #[cfg(feature = "deprecated")]
    pub ShareFolderWithTelemetryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShareFolderWithTelemetryAsync: usize,
    #[cfg(feature = "deprecated")]
    pub LaunchFileWithUIAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LaunchFileWithUIAsync: usize,
    #[cfg(feature = "deprecated")]
    pub LaunchFileWithUIAndTelemetryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LaunchFileWithUIAndTelemetryAsync: usize,
    #[cfg(feature = "deprecated")]
    pub TerminateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TerminateAsync: usize,
    #[cfg(feature = "deprecated")]
    pub TerminateWithTelemetryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TerminateWithTelemetryAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RegisterMessageReceiver: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RegisterMessageReceiver: usize,
    #[cfg(feature = "deprecated")]
    pub UnregisterMessageReceiver: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UnregisterMessageReceiver: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironment2, IIsolatedWindowsEnvironment2_Vtbl, 0x2d365f39_88bd_4ab4_93cf_7e2bcef337c0);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironment2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironment2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub PostMessageToReceiverAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    PostMessageToReceiverAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub PostMessageToReceiverWithTelemetryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    PostMessageToReceiverWithTelemetryAsync: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironment3, IIsolatedWindowsEnvironment3_Vtbl, 0xcb7fc7d2_d06e_4c26_8ada_dacdaaad03f5);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironment3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironment3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub GetUserInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetUserInfo: usize,
    #[cfg(feature = "deprecated")]
    pub ShareFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShareFileAsync: usize,
    #[cfg(feature = "deprecated")]
    pub ShareFileWithTelemetryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShareFileWithTelemetryAsync: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironment4, IIsolatedWindowsEnvironment4_Vtbl, 0x11e3701a_dd9e_4f1b_812c_4020f307f93c);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironment4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironment4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ChangePriority: unsafe extern "system" fn(*mut core::ffi::c_void, IsolatedWindowsEnvironmentCreationPriority) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ChangePriority: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentCreateResult, IIsolatedWindowsEnvironmentCreateResult_Vtbl, 0xef9a5e58_dcd7_45c2_9c85_ab642a715e8e);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentCreateResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentCreateResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentCreateStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExtendedError: usize,
    #[cfg(feature = "deprecated")]
    pub Environment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Environment: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentCreateResult2, IIsolatedWindowsEnvironmentCreateResult2_Vtbl, 0xa547dbc7_61d4_4fb8_ab5c_edefa3d388ad);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentCreateResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentCreateResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ChangeCreationPriority: unsafe extern "system" fn(*mut core::ffi::c_void, IsolatedWindowsEnvironmentCreationPriority) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ChangeCreationPriority: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentFactory, IIsolatedWindowsEnvironmentFactory_Vtbl, 0x1aca93e7_e804_454d_8466_f9897c20b0f6);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateAsync: usize,
    #[cfg(feature = "deprecated")]
    pub CreateWithTelemetryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWithTelemetryAsync: usize,
    #[cfg(feature = "deprecated")]
    pub GetById: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetById: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindByOwnerId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindByOwnerId: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentFile, IIsolatedWindowsEnvironmentFile_Vtbl, 0x4d5ae1ef_029f_4101_8c35_fe91bf9cd5f0);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentFile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentFile_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub HostPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    HostPath: usize,
    #[cfg(feature = "deprecated")]
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Close: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentFile2, IIsolatedWindowsEnvironmentFile2_Vtbl, 0x4eeb8dec_ad5d_4b0a_b754_f36c3d46d684);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentFile2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentFile2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub GuestPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GuestPath: usize,
    #[cfg(feature = "deprecated")]
    pub IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsReadOnly: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentHostStatics, IIsolatedWindowsEnvironmentHostStatics_Vtbl, 0x2c0e22c7_05a0_517a_b81c_6ee8790c381f);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentHostStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentHostStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub IsReady: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsReady: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub HostErrors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    HostErrors: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentLaunchFileResult, IIsolatedWindowsEnvironmentLaunchFileResult_Vtbl, 0x685d4176_f6e0_4569_b1aa_215c0ff5b257);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentLaunchFileResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentLaunchFileStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExtendedError: usize,
    #[cfg(feature = "deprecated")]
    pub File: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    File: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentOptions, IIsolatedWindowsEnvironmentOptions_Vtbl, 0xb71d98f7_61f0_4008_b207_0bf9eb2d76f2);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub EnvironmentOwnerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EnvironmentOwnerId: usize,
    #[cfg(feature = "deprecated")]
    pub SetEnvironmentOwnerId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetEnvironmentOwnerId: usize,
    #[cfg(feature = "deprecated")]
    pub AllowedClipboardFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentAllowedClipboardFormats) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AllowedClipboardFormats: usize,
    #[cfg(feature = "deprecated")]
    pub SetAllowedClipboardFormats: unsafe extern "system" fn(*mut core::ffi::c_void, IsolatedWindowsEnvironmentAllowedClipboardFormats) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAllowedClipboardFormats: usize,
    #[cfg(feature = "deprecated")]
    pub ClipboardCopyPasteDirections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClipboardCopyPasteDirections: usize,
    #[cfg(feature = "deprecated")]
    pub SetClipboardCopyPasteDirections: unsafe extern "system" fn(*mut core::ffi::c_void, IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetClipboardCopyPasteDirections: usize,
    #[cfg(feature = "deprecated")]
    pub AvailablePrinters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentAvailablePrinters) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AvailablePrinters: usize,
    #[cfg(feature = "deprecated")]
    pub SetAvailablePrinters: unsafe extern "system" fn(*mut core::ffi::c_void, IsolatedWindowsEnvironmentAvailablePrinters) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAvailablePrinters: usize,
    #[cfg(feature = "deprecated")]
    pub SharedHostFolderPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SharedHostFolderPath: usize,
    #[cfg(feature = "deprecated")]
    pub SharedFolderNameInEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SharedFolderNameInEnvironment: usize,
    #[cfg(feature = "deprecated")]
    pub ShareHostFolderForUntrustedItems: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShareHostFolderForUntrustedItems: usize,
    #[cfg(feature = "deprecated")]
    pub PersistUserProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PersistUserProfile: usize,
    #[cfg(feature = "deprecated")]
    pub SetPersistUserProfile: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPersistUserProfile: usize,
    #[cfg(feature = "deprecated")]
    pub AllowGraphicsHardwareAcceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AllowGraphicsHardwareAcceleration: usize,
    #[cfg(feature = "deprecated")]
    pub SetAllowGraphicsHardwareAcceleration: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAllowGraphicsHardwareAcceleration: usize,
    #[cfg(feature = "deprecated")]
    pub AllowCameraAndMicrophoneAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AllowCameraAndMicrophoneAccess: usize,
    #[cfg(feature = "deprecated")]
    pub SetAllowCameraAndMicrophoneAccess: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAllowCameraAndMicrophoneAccess: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentOptions2, IIsolatedWindowsEnvironmentOptions2_Vtbl, 0x10d7cc31_8b8f_4b9d_b22c_617103b55b08);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub WindowAnnotationOverride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    WindowAnnotationOverride: usize,
    #[cfg(feature = "deprecated")]
    pub SetWindowAnnotationOverride: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetWindowAnnotationOverride: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentOptions3, IIsolatedWindowsEnvironmentOptions3_Vtbl, 0x98d5aa23_161f_4cd9_8a9c_269b30122b0d);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentOptions3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentOptions3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub AllowedClipboardFormatsToEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentAllowedClipboardFormats) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AllowedClipboardFormatsToEnvironment: usize,
    #[cfg(feature = "deprecated")]
    pub SetAllowedClipboardFormatsToEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, IsolatedWindowsEnvironmentAllowedClipboardFormats) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAllowedClipboardFormatsToEnvironment: usize,
    #[cfg(feature = "deprecated")]
    pub AllowedClipboardFormatsToHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentAllowedClipboardFormats) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AllowedClipboardFormatsToHost: usize,
    #[cfg(feature = "deprecated")]
    pub SetAllowedClipboardFormatsToHost: unsafe extern "system" fn(*mut core::ffi::c_void, IsolatedWindowsEnvironmentAllowedClipboardFormats) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAllowedClipboardFormatsToHost: usize,
    #[cfg(feature = "deprecated")]
    pub CreationPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentCreationPriority) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreationPriority: usize,
    #[cfg(feature = "deprecated")]
    pub SetCreationPriority: unsafe extern "system" fn(*mut core::ffi::c_void, IsolatedWindowsEnvironmentCreationPriority) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetCreationPriority: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentOwnerRegistrationData, IIsolatedWindowsEnvironmentOwnerRegistrationData_Vtbl, 0xf888ec22_e8cf_56c0_b1df_90af4ad80e84);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentOwnerRegistrationData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ShareableFolders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ShareableFolders: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ProcessesRunnableAsSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ProcessesRunnableAsSystem: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ProcessesRunnableAsUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ProcessesRunnableAsUser: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ActivationFileExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ActivationFileExtensions: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentOwnerRegistrationResult, IIsolatedWindowsEnvironmentOwnerRegistrationResult_Vtbl, 0x6dab9451_6169_55df_8f51_790e99d7277d);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentOwnerRegistrationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentOwnerRegistrationStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExtendedError: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentOwnerRegistrationStatics, IIsolatedWindowsEnvironmentOwnerRegistrationStatics_Vtbl, 0x10951754_204b_5ec9_9de3_df792d074a61);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentOwnerRegistrationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Register: usize,
    #[cfg(feature = "deprecated")]
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Unregister: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentPostMessageResult, IIsolatedWindowsEnvironmentPostMessageResult_Vtbl, 0x0dfa28fa_2ef0_4d8f_b341_3171b2df93b1);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentPostMessageResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentPostMessageStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExtendedError: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentProcess, IIsolatedWindowsEnvironmentProcess_Vtbl, 0xa858c3ef_8172_4f10_af93_cbe60af88d09);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentProcess {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentProcess_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentProcessState) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    State: usize,
    #[cfg(feature = "deprecated")]
    pub ExitCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExitCode: usize,
    #[cfg(feature = "deprecated")]
    pub WaitForExit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    WaitForExit: usize,
    #[cfg(feature = "deprecated")]
    pub WaitForExitWithTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    WaitForExitWithTimeout: usize,
    #[cfg(feature = "deprecated")]
    pub WaitForExitAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    WaitForExitAsync: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentShareFileRequestOptions, IIsolatedWindowsEnvironmentShareFileRequestOptions_Vtbl, 0xc9190ed8_0fd0_4946_bb88_117a60737b61);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentShareFileRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub AllowWrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AllowWrite: usize,
    #[cfg(feature = "deprecated")]
    pub SetAllowWrite: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAllowWrite: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentShareFileResult, IIsolatedWindowsEnvironmentShareFileResult_Vtbl, 0xaec7caa7_9ac6_4bf5_8b91_5c1adf0d7d00);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentShareFileResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentShareFileResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentShareFileStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExtendedError: usize,
    #[cfg(feature = "deprecated")]
    pub File: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    File: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentShareFolderRequestOptions, IIsolatedWindowsEnvironmentShareFolderRequestOptions_Vtbl, 0xc405eb7d_7053_4f6a_9b87_746846ed19b2);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentShareFolderRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub AllowWrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AllowWrite: usize,
    #[cfg(feature = "deprecated")]
    pub SetAllowWrite: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAllowWrite: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentShareFolderResult, IIsolatedWindowsEnvironmentShareFolderResult_Vtbl, 0x556ba72e_ca9d_4211_b143_1cedc86eb2fe);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentShareFolderResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentShareFolderStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExtendedError: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentStartProcessResult, IIsolatedWindowsEnvironmentStartProcessResult_Vtbl, 0x8fa1dc2f_57da_4bb5_9c06_fa072d2032e2);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentStartProcessResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsolatedWindowsEnvironmentStartProcessStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExtendedError: usize,
    #[cfg(feature = "deprecated")]
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Process: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentTelemetryParameters, IIsolatedWindowsEnvironmentTelemetryParameters_Vtbl, 0xebdb3cab_7a3a_4524_a0f4_f96e284d33cd);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentTelemetryParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CorrelationId: usize,
    #[cfg(feature = "deprecated")]
    pub SetCorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetCorrelationId: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentUserInfo, IIsolatedWindowsEnvironmentUserInfo_Vtbl, 0x8a9c75ae_69ba_4001_96fc_19a02703b340);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentUserInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentUserInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub EnvironmentUserSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EnvironmentUserSid: usize,
    #[cfg(feature = "deprecated")]
    pub EnvironmentUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EnvironmentUserName: usize,
    #[cfg(feature = "deprecated")]
    pub TryWaitForSignInAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryWaitForSignInAsync: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsEnvironmentUserInfo2, IIsolatedWindowsEnvironmentUserInfo2_Vtbl, 0xb0bdd5dd_91d7_481e_94f2_2a5a6bdf9383);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsEnvironmentUserInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsEnvironmentUserInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub TryWaitForSignInWithProgressAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryWaitForSignInWithProgressAsync: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsHostMessengerStatics, IIsolatedWindowsHostMessengerStatics_Vtbl, 0x06e444bb_53c0_4889_8fa3_53592e37cf21);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsHostMessengerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsHostMessengerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub PostMessageToReceiver: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    PostMessageToReceiver: usize,
    #[cfg(feature = "deprecated")]
    pub GetFileId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetFileId: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IIsolatedWindowsHostMessengerStatics2, IIsolatedWindowsHostMessengerStatics2_Vtbl, 0x55ef9ebc_0444_42ad_832d_1b89c089d1ca);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IIsolatedWindowsHostMessengerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IIsolatedWindowsHostMessengerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RegisterHostMessageReceiver: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RegisterHostMessageReceiver: usize,
    #[cfg(feature = "deprecated")]
    pub UnregisterHostMessageReceiver: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UnregisterHostMessageReceiver: usize,
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironment(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironment, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironment {
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn StartProcessSilentlyAsync(&self, hostexepath: &windows_core::HSTRING, arguments: &windows_core::HSTRING, activator: IsolatedWindowsEnvironmentActivator) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartProcessSilentlyAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(hostexepath), core::mem::transmute_copy(arguments), activator, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn StartProcessSilentlyWithTelemetryAsync<P0>(&self, hostexepath: &windows_core::HSTRING, arguments: &windows_core::HSTRING, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>
    where
        P0: windows_core::Param<IsolatedWindowsEnvironmentTelemetryParameters>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartProcessSilentlyWithTelemetryAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(hostexepath), core::mem::transmute_copy(arguments), activator, telemetryparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShareFolderAsync<P0>(&self, hostfolder: &windows_core::HSTRING, requestoptions: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>
    where
        P0: windows_core::Param<IsolatedWindowsEnvironmentShareFolderRequestOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShareFolderAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(hostfolder), requestoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShareFolderWithTelemetryAsync<P0, P1>(&self, hostfolder: &windows_core::HSTRING, requestoptions: P0, telemetryparameters: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>
    where
        P0: windows_core::Param<IsolatedWindowsEnvironmentShareFolderRequestOptions>,
        P1: windows_core::Param<IsolatedWindowsEnvironmentTelemetryParameters>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShareFolderWithTelemetryAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(hostfolder), requestoptions.param().abi(), telemetryparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn LaunchFileWithUIAsync(&self, appexepath: &windows_core::HSTRING, argumentstemplate: &windows_core::HSTRING, filepath: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchFileWithUIAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appexepath), core::mem::transmute_copy(argumentstemplate), core::mem::transmute_copy(filepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn LaunchFileWithUIAndTelemetryAsync<P0>(&self, appexepath: &windows_core::HSTRING, argumentstemplate: &windows_core::HSTRING, filepath: &windows_core::HSTRING, telemetryparameters: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>
    where
        P0: windows_core::Param<IsolatedWindowsEnvironmentTelemetryParameters>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchFileWithUIAndTelemetryAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appexepath), core::mem::transmute_copy(argumentstemplate), core::mem::transmute_copy(filepath), telemetryparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn TerminateAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TerminateAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn TerminateWithTelemetryAsync<P0>(&self, telemetryparameters: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<IsolatedWindowsEnvironmentTelemetryParameters>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TerminateWithTelemetryAsync)(windows_core::Interface::as_raw(this), telemetryparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RegisterMessageReceiver<P0>(&self, receiverid: windows_core::GUID, messagereceivedcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MessageReceivedCallback>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterMessageReceiver)(windows_core::Interface::as_raw(this), receiverid, messagereceivedcallback.param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn UnregisterMessageReceiver(&self, receiverid: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UnregisterMessageReceiver)(windows_core::Interface::as_raw(this), receiverid).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn PostMessageToReceiverAsync<P0>(&self, receiverid: windows_core::GUID, message: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PostMessageToReceiverAsync)(windows_core::Interface::as_raw(this), receiverid, message.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn PostMessageToReceiverWithTelemetryAsync<P0, P1>(&self, receiverid: windows_core::GUID, message: P0, telemetryparameters: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::IInspectable>>,
        P1: windows_core::Param<IsolatedWindowsEnvironmentTelemetryParameters>,
    {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PostMessageToReceiverWithTelemetryAsync)(windows_core::Interface::as_raw(this), receiverid, message.param().abi(), telemetryparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn GetUserInfo(&self) -> windows_core::Result<IsolatedWindowsEnvironmentUserInfo> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUserInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShareFileAsync<P0>(&self, filepath: &windows_core::HSTRING, options: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>
    where
        P0: windows_core::Param<IsolatedWindowsEnvironmentShareFileRequestOptions>,
    {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShareFileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(filepath), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShareFileWithTelemetryAsync<P0, P1>(&self, filepath: &windows_core::HSTRING, options: P0, telemetryparameters: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>
    where
        P0: windows_core::Param<IsolatedWindowsEnvironmentShareFileRequestOptions>,
        P1: windows_core::Param<IsolatedWindowsEnvironmentTelemetryParameters>,
    {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShareFileWithTelemetryAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(filepath), options.param().abi(), telemetryparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ChangePriority(&self, priority: IsolatedWindowsEnvironmentCreationPriority) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironment4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ChangePriority)(windows_core::Interface::as_raw(this), priority).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateAsync<P0>(options: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>
    where
        P0: windows_core::Param<IsolatedWindowsEnvironmentOptions>,
    {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAsync)(windows_core::Interface::as_raw(this), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateWithTelemetryAsync<P0, P1>(options: P0, telemetryparameters: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>
    where
        P0: windows_core::Param<IsolatedWindowsEnvironmentOptions>,
        P1: windows_core::Param<IsolatedWindowsEnvironmentTelemetryParameters>,
    {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithTelemetryAsync)(windows_core::Interface::as_raw(this), options.param().abi(), telemetryparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn GetById(environmentid: &windows_core::HSTRING) -> windows_core::Result<IsolatedWindowsEnvironment> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetById)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(environmentid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindByOwnerId(environmentownerid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironment>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindByOwnerId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(environmentownerid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IIsolatedWindowsEnvironmentFactory<R, F: FnOnce(&IIsolatedWindowsEnvironmentFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IsolatedWindowsEnvironment, IIsolatedWindowsEnvironmentFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironment>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironment {
    type Vtable = IIsolatedWindowsEnvironment_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironment as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironment {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironment";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironment {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironment {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentCreateResult(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentCreateResult, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentCreateResult {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> windows_core::Result<IsolatedWindowsEnvironmentCreateStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Environment(&self) -> windows_core::Result<IsolatedWindowsEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Environment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ChangeCreationPriority(&self, priority: IsolatedWindowsEnvironmentCreationPriority) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironmentCreateResult2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ChangeCreationPriority)(windows_core::Interface::as_raw(this), priority).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentCreateResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentCreateResult>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentCreateResult {
    type Vtable = IIsolatedWindowsEnvironmentCreateResult_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentCreateResult as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentCreateResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentCreateResult {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentCreateResult {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentFile(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentFile, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentFile {
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn HostPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HostPath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn GuestPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironmentFile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GuestPath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsReadOnly(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironmentFile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentFile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentFile>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentFile {
    type Vtable = IIsolatedWindowsEnvironmentFile_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentFile as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentFile {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentFile";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentFile {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentFile {}
#[cfg(feature = "deprecated")]
pub struct IsolatedWindowsEnvironmentHost;
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentHost {
    #[cfg(feature = "deprecated")]
    pub fn IsReady() -> windows_core::Result<bool> {
        Self::IIsolatedWindowsEnvironmentHostStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReady)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn HostErrors() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironmentHostError>> {
        Self::IIsolatedWindowsEnvironmentHostStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HostErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IIsolatedWindowsEnvironmentHostStatics<R, F: FnOnce(&IIsolatedWindowsEnvironmentHostStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IsolatedWindowsEnvironmentHost, IIsolatedWindowsEnvironmentHostStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentHost {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentHost";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentLaunchFileResult(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentLaunchFileResult, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentLaunchFileResult {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> windows_core::Result<IsolatedWindowsEnvironmentLaunchFileStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn File(&self) -> windows_core::Result<IsolatedWindowsEnvironmentFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentLaunchFileResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentLaunchFileResult>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentLaunchFileResult {
    type Vtable = IIsolatedWindowsEnvironmentLaunchFileResult_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentLaunchFileResult as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentLaunchFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentLaunchFileResult {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentLaunchFileResult {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentOptions(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentOptions, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IsolatedWindowsEnvironmentOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "deprecated")]
    pub fn EnvironmentOwnerId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnvironmentOwnerId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetEnvironmentOwnerId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEnvironmentOwnerId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn AllowedClipboardFormats(&self) -> windows_core::Result<IsolatedWindowsEnvironmentAllowedClipboardFormats> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedClipboardFormats)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAllowedClipboardFormats(&self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowedClipboardFormats)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ClipboardCopyPasteDirections(&self) -> windows_core::Result<IsolatedWindowsEnvironmentClipboardCopyPasteDirections> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClipboardCopyPasteDirections)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetClipboardCopyPasteDirections(&self, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetClipboardCopyPasteDirections)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn AvailablePrinters(&self) -> windows_core::Result<IsolatedWindowsEnvironmentAvailablePrinters> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AvailablePrinters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAvailablePrinters(&self, value: IsolatedWindowsEnvironmentAvailablePrinters) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAvailablePrinters)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn SharedHostFolderPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharedHostFolderPath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SharedFolderNameInEnvironment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharedFolderNameInEnvironment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShareHostFolderForUntrustedItems(&self, sharedhostfolderpath: &windows_core::HSTRING, sharefoldernameinenvironment: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShareHostFolderForUntrustedItems)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sharedhostfolderpath), core::mem::transmute_copy(sharefoldernameinenvironment)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn PersistUserProfile(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PersistUserProfile)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetPersistUserProfile(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPersistUserProfile)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn AllowGraphicsHardwareAcceleration(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowGraphicsHardwareAcceleration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAllowGraphicsHardwareAcceleration(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowGraphicsHardwareAcceleration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn AllowCameraAndMicrophoneAccess(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowCameraAndMicrophoneAccess)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAllowCameraAndMicrophoneAccess(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowCameraAndMicrophoneAccess)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn WindowAnnotationOverride(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironmentOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WindowAnnotationOverride)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetWindowAnnotationOverride(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironmentOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWindowAnnotationOverride)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn AllowedClipboardFormatsToEnvironment(&self) -> windows_core::Result<IsolatedWindowsEnvironmentAllowedClipboardFormats> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironmentOptions3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedClipboardFormatsToEnvironment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAllowedClipboardFormatsToEnvironment(&self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironmentOptions3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowedClipboardFormatsToEnvironment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn AllowedClipboardFormatsToHost(&self) -> windows_core::Result<IsolatedWindowsEnvironmentAllowedClipboardFormats> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironmentOptions3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedClipboardFormatsToHost)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAllowedClipboardFormatsToHost(&self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironmentOptions3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowedClipboardFormatsToHost)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreationPriority(&self) -> windows_core::Result<IsolatedWindowsEnvironmentCreationPriority> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironmentOptions3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreationPriority)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetCreationPriority(&self, value: IsolatedWindowsEnvironmentCreationPriority) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironmentOptions3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCreationPriority)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentOptions>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentOptions {
    type Vtable = IIsolatedWindowsEnvironmentOptions_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentOptions as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentOptions {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentOptions {}
#[cfg(feature = "deprecated")]
pub struct IsolatedWindowsEnvironmentOwnerRegistration;
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentOwnerRegistration {
    #[cfg(feature = "deprecated")]
    pub fn Register<P0>(ownername: &windows_core::HSTRING, ownerregistrationdata: P0) -> windows_core::Result<IsolatedWindowsEnvironmentOwnerRegistrationResult>
    where
        P0: windows_core::Param<IsolatedWindowsEnvironmentOwnerRegistrationData>,
    {
        Self::IIsolatedWindowsEnvironmentOwnerRegistrationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Register)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(ownername), ownerregistrationdata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn Unregister(ownername: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IIsolatedWindowsEnvironmentOwnerRegistrationStatics(|this| unsafe { (windows_core::Interface::vtable(this).Unregister)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(ownername)).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IIsolatedWindowsEnvironmentOwnerRegistrationStatics<R, F: FnOnce(&IIsolatedWindowsEnvironmentOwnerRegistrationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IsolatedWindowsEnvironmentOwnerRegistration, IIsolatedWindowsEnvironmentOwnerRegistrationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistration {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistration";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationData(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentOwnerRegistrationData, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentOwnerRegistrationData {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IsolatedWindowsEnvironmentOwnerRegistrationData, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ShareableFolders(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShareableFolders)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ProcessesRunnableAsSystem(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessesRunnableAsSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ProcessesRunnableAsUser(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessesRunnableAsUser)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ActivationFileExtensions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivationFileExtensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentOwnerRegistrationData>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentOwnerRegistrationData {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationData_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentOwnerRegistrationData as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentOwnerRegistrationData {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentOwnerRegistrationData {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationResult(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentOwnerRegistrationResult, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentOwnerRegistrationResult {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> windows_core::Result<IsolatedWindowsEnvironmentOwnerRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentOwnerRegistrationResult>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationResult_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentOwnerRegistrationResult as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentPostMessageResult(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentPostMessageResult, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentPostMessageResult {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> windows_core::Result<IsolatedWindowsEnvironmentPostMessageStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentPostMessageResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentPostMessageResult>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentPostMessageResult {
    type Vtable = IIsolatedWindowsEnvironmentPostMessageResult_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentPostMessageResult as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentPostMessageResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentPostMessageResult {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentPostMessageResult {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentProcess(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentProcess, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentProcess {
    #[cfg(feature = "deprecated")]
    pub fn State(&self) -> windows_core::Result<IsolatedWindowsEnvironmentProcessState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ExitCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn WaitForExit(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WaitForExit)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn WaitForExitWithTimeout(&self, timeoutmilliseconds: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WaitForExitWithTimeout)(windows_core::Interface::as_raw(this), timeoutmilliseconds).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn WaitForExitAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WaitForExitAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentProcess {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentProcess>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentProcess {
    type Vtable = IIsolatedWindowsEnvironmentProcess_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentProcess as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentProcess {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentProcess {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentProcess {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentShareFileRequestOptions(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentShareFileRequestOptions, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentShareFileRequestOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IsolatedWindowsEnvironmentShareFileRequestOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "deprecated")]
    pub fn AllowWrite(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowWrite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAllowWrite(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowWrite)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentShareFileRequestOptions>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentShareFileRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFileRequestOptions_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentShareFileRequestOptions as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentShareFileRequestOptions {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentShareFileRequestOptions {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentShareFileResult(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentShareFileResult, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentShareFileResult {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> windows_core::Result<IsolatedWindowsEnvironmentShareFileStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn File(&self) -> windows_core::Result<IsolatedWindowsEnvironmentFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentShareFileResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentShareFileResult>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentShareFileResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFileResult_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentShareFileResult as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentShareFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentShareFileResult {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentShareFileResult {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentShareFolderRequestOptions(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentShareFolderRequestOptions, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentShareFolderRequestOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IsolatedWindowsEnvironmentShareFolderRequestOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "deprecated")]
    pub fn AllowWrite(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowWrite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAllowWrite(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowWrite)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentShareFolderRequestOptions>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderRequestOptions_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentShareFolderRequestOptions as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentShareFolderResult(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentShareFolderResult, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentShareFolderResult {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> windows_core::Result<IsolatedWindowsEnvironmentShareFolderStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentShareFolderResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentShareFolderResult>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentShareFolderResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderResult_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentShareFolderResult as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentShareFolderResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentShareFolderResult {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentShareFolderResult {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentStartProcessResult(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentStartProcessResult, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentStartProcessResult {
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> windows_core::Result<IsolatedWindowsEnvironmentStartProcessStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Process(&self) -> windows_core::Result<IsolatedWindowsEnvironmentProcess> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Process)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentStartProcessResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentStartProcessResult>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentStartProcessResult {
    type Vtable = IIsolatedWindowsEnvironmentStartProcessResult_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentStartProcessResult as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentStartProcessResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentStartProcessResult {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentStartProcessResult {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentTelemetryParameters(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentTelemetryParameters, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentTelemetryParameters {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IsolatedWindowsEnvironmentTelemetryParameters, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "deprecated")]
    pub fn CorrelationId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CorrelationId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetCorrelationId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCorrelationId)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentTelemetryParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentTelemetryParameters>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentTelemetryParameters {
    type Vtable = IIsolatedWindowsEnvironmentTelemetryParameters_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentTelemetryParameters as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentTelemetryParameters {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentTelemetryParameters {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentTelemetryParameters {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsolatedWindowsEnvironmentUserInfo(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(IsolatedWindowsEnvironmentUserInfo, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentUserInfo {
    #[cfg(feature = "deprecated")]
    pub fn EnvironmentUserSid(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnvironmentUserSid)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn EnvironmentUserName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnvironmentUserName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn TryWaitForSignInAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryWaitForSignInAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn TryWaitForSignInWithProgressAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, IsolatedWindowsEnvironmentSignInProgress>> {
        let this = &windows_core::Interface::cast::<IIsolatedWindowsEnvironmentUserInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryWaitForSignInWithProgressAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentUserInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsolatedWindowsEnvironmentUserInfo>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for IsolatedWindowsEnvironmentUserInfo {
    type Vtable = IIsolatedWindowsEnvironmentUserInfo_Vtbl;
    const IID: windows_core::GUID = <IIsolatedWindowsEnvironmentUserInfo as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsEnvironmentUserInfo {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for IsolatedWindowsEnvironmentUserInfo {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for IsolatedWindowsEnvironmentUserInfo {}
#[cfg(feature = "deprecated")]
pub struct IsolatedWindowsHostMessenger;
#[cfg(feature = "deprecated")]
impl IsolatedWindowsHostMessenger {
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn PostMessageToReceiver<P0>(receiverid: windows_core::GUID, message: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<windows_core::IInspectable>>,
    {
        Self::IIsolatedWindowsHostMessengerStatics(|this| unsafe { (windows_core::Interface::vtable(this).PostMessageToReceiver)(windows_core::Interface::as_raw(this), receiverid, message.param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn GetFileId(filepath: &windows_core::HSTRING) -> windows_core::Result<windows_core::GUID> {
        Self::IIsolatedWindowsHostMessengerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(filepath), &mut result__).map(|| result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RegisterHostMessageReceiver<P0>(receiverid: windows_core::GUID, hostmessagereceivedcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HostMessageReceivedCallback>,
    {
        Self::IIsolatedWindowsHostMessengerStatics2(|this| unsafe { (windows_core::Interface::vtable(this).RegisterHostMessageReceiver)(windows_core::Interface::as_raw(this), receiverid, hostmessagereceivedcallback.param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn UnregisterHostMessageReceiver(receiverid: windows_core::GUID) -> windows_core::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics2(|this| unsafe { (windows_core::Interface::vtable(this).UnregisterHostMessageReceiver)(windows_core::Interface::as_raw(this), receiverid).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IIsolatedWindowsHostMessengerStatics<R, F: FnOnce(&IIsolatedWindowsHostMessengerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IsolatedWindowsHostMessenger, IIsolatedWindowsHostMessengerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IIsolatedWindowsHostMessengerStatics2<R, F: FnOnce(&IIsolatedWindowsHostMessengerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IsolatedWindowsHostMessenger, IIsolatedWindowsHostMessengerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IsolatedWindowsHostMessenger {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsHostMessenger";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentActivator(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentActivator {
    pub const System: Self = Self(0i32);
    pub const User: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentActivator {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentActivator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentActivator").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentActivator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentActivator;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentAllowedClipboardFormats(pub u32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentAllowedClipboardFormats {
    pub const None: Self = Self(0u32);
    pub const Text: Self = Self(1u32);
    pub const Image: Self = Self(2u32);
    pub const Rtf: Self = Self(4u32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentAllowedClipboardFormats").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentAllowedClipboardFormats {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::BitOr for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::BitAnd for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::BitOrAssign for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::BitAndAssign for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::Not for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentAllowedClipboardFormats;u4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentAvailablePrinters(pub u32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentAvailablePrinters {
    pub const None: Self = Self(0u32);
    pub const Local: Self = Self(1u32);
    pub const Network: Self = Self(2u32);
    pub const SystemPrintToPdf: Self = Self(4u32);
    pub const SystemPrintToXps: Self = Self(8u32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentAvailablePrinters {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentAvailablePrinters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentAvailablePrinters").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentAvailablePrinters {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::BitOr for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::BitAnd for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::BitOrAssign for IsolatedWindowsEnvironmentAvailablePrinters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::BitAndAssign for IsolatedWindowsEnvironmentAvailablePrinters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::Not for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentAvailablePrinters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentAvailablePrinters;u4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentClipboardCopyPasteDirections(pub u32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    pub const None: Self = Self(0u32);
    pub const HostToIsolatedWindowsEnvironment: Self = Self(1u32);
    pub const IsolatedWindowsEnvironmentToHost: Self = Self(2u32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentClipboardCopyPasteDirections").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::BitOr for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::BitAnd for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::BitOrAssign for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::BitAndAssign for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
#[cfg(feature = "deprecated")]
impl core::ops::Not for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentClipboardCopyPasteDirections;u4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentCreateStatus(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentCreateStatus {
    pub const Success: Self = Self(0i32);
    pub const FailureByPolicy: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentCreateStatus {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentCreateStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentCreateStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentCreateStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateStatus;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentCreationPriority(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentCreationPriority {
    pub const Low: Self = Self(0i32);
    pub const Normal: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentCreationPriority {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentCreationPriority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentCreationPriority").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentCreationPriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreationPriority;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentHostError(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentHostError {
    pub const AdminPolicyIsDisabledOrNotPresent: Self = Self(0i32);
    pub const FeatureNotInstalled: Self = Self(1i32);
    pub const HardwareRequirementsNotMet: Self = Self(2i32);
    pub const RebootRequired: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentHostError {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentHostError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentHostError").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentHostError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentHostError;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentLaunchFileStatus(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentLaunchFileStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FileNotFound: Self = Self(3i32);
    pub const TimedOut: Self = Self(4i32);
    pub const AlreadySharedWithConflictingOptions: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentLaunchFileStatus {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentLaunchFileStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentLaunchFileStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentLaunchFileStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileStatus;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationStatus(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidArgument: Self = Self(1i32);
    pub const AccessDenied: Self = Self(2i32);
    pub const InsufficientMemory: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOwnerRegistrationStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationStatus;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentPostMessageStatus(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentPostMessageStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentPostMessageStatus {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentPostMessageStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentPostMessageStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentPostMessageStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageStatus;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentProcessState(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentProcessState {
    pub const Running: Self = Self(1i32);
    pub const Aborted: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentProcessState {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentProcessState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentProcessState").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentProcessState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProcessState;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentProgressState(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentProgressState {
    pub const Queued: Self = Self(0i32);
    pub const Processing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
    pub const Creating: Self = Self(3i32);
    pub const Retrying: Self = Self(4i32);
    pub const Starting: Self = Self(5i32);
    pub const Finalizing: Self = Self(6i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentProgressState {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentProgressState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentProgressState").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentProgressState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentShareFileStatus(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentShareFileStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const AlreadySharedWithConflictingOptions: Self = Self(3i32);
    pub const FileNotFound: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentShareFileStatus {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentShareFileStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFileStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentShareFileStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileStatus;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentShareFolderStatus(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentShareFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FolderNotFound: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentShareFolderStatus {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentShareFolderStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFolderStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentShareFolderStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderStatus;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentSignInProgress(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentSignInProgress {
    pub const Connecting: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Authenticating: Self = Self(2i32);
    pub const SettingUpAccount: Self = Self(3i32);
    pub const Finalizing: Self = Self(4i32);
    pub const Completed: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentSignInProgress {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentSignInProgress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentSignInProgress").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentSignInProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentSignInProgress;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsolatedWindowsEnvironmentStartProcessStatus(pub i32);
#[cfg(feature = "deprecated")]
impl IsolatedWindowsEnvironmentStartProcessStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FileNotFound: Self = Self(3i32);
    pub const AppNotRegistered: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsolatedWindowsEnvironmentStartProcessStatus {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsolatedWindowsEnvironmentStartProcessStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentStartProcessStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentStartProcessStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessStatus;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IsolatedWindowsEnvironmentCreateProgress {
    pub State: IsolatedWindowsEnvironmentProgressState,
    pub PercentComplete: u32,
}
impl windows_core::TypeKind for IsolatedWindowsEnvironmentCreateProgress {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for IsolatedWindowsEnvironmentCreateProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateProgress;enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState;i4);u4)");
}
impl Default for IsolatedWindowsEnvironmentCreateProgress {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
windows_core::imp::define_interface!(HostMessageReceivedCallback, HostMessageReceivedCallback_Vtbl, 0xfaf26ffa_8ce1_4cc1_b278_322d31a5e4a3);
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl HostMessageReceivedCallback {
    pub fn new<F: FnMut(&windows_core::GUID, Option<&super::super::Foundation::Collections::IVectorView<windows_core::IInspectable>>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = HostMessageReceivedCallbackBox::<F> { vtable: &HostMessageReceivedCallbackBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Invoke<P0>(&self, receiverid: windows_core::GUID, message: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<windows_core::IInspectable>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), receiverid, message.param().abi()).ok() }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
#[repr(C)]
struct HostMessageReceivedCallbackBox<F: FnMut(&windows_core::GUID, Option<&super::super::Foundation::Collections::IVectorView<windows_core::IInspectable>>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const HostMessageReceivedCallback_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl<F: FnMut(&windows_core::GUID, Option<&super::super::Foundation::Collections::IVectorView<windows_core::IInspectable>>) -> windows_core::Result<()> + Send + 'static> HostMessageReceivedCallbackBox<F> {
    const VTABLE: HostMessageReceivedCallback_Vtbl = HostMessageReceivedCallback_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <HostMessageReceivedCallback as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, receiverid: windows_core::GUID, message: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(core::mem::transmute(&receiverid), windows_core::from_raw_borrowed(&message)).into()
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl windows_core::RuntimeType for HostMessageReceivedCallback {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
#[repr(C)]
pub struct HostMessageReceivedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Invoke: usize,
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
windows_core::imp::define_interface!(MessageReceivedCallback, MessageReceivedCallback_Vtbl, 0xf5b4c8ff_1d9d_4995_9fea_4d15257c0757);
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl MessageReceivedCallback {
    pub fn new<F: FnMut(&windows_core::GUID, Option<&super::super::Foundation::Collections::IVectorView<windows_core::IInspectable>>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = MessageReceivedCallbackBox::<F> { vtable: &MessageReceivedCallbackBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Invoke<P0>(&self, receiverid: windows_core::GUID, message: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<windows_core::IInspectable>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), receiverid, message.param().abi()).ok() }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
#[repr(C)]
struct MessageReceivedCallbackBox<F: FnMut(&windows_core::GUID, Option<&super::super::Foundation::Collections::IVectorView<windows_core::IInspectable>>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const MessageReceivedCallback_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl<F: FnMut(&windows_core::GUID, Option<&super::super::Foundation::Collections::IVectorView<windows_core::IInspectable>>) -> windows_core::Result<()> + Send + 'static> MessageReceivedCallbackBox<F> {
    const VTABLE: MessageReceivedCallback_Vtbl = MessageReceivedCallback_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <MessageReceivedCallback as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, receiverid: windows_core::GUID, message: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(core::mem::transmute(&receiverid), windows_core::from_raw_borrowed(&message)).into()
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl windows_core::RuntimeType for MessageReceivedCallback {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
#[repr(C)]
pub struct MessageReceivedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Invoke: usize,
}
