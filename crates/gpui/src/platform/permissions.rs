/// The current status of a system permission.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Hash)]
pub enum PermissionStatus {
    /// The user has granted permission.
    Granted,
    /// The user has explicitly denied permission.
    Denied,
    /// The user has not yet been prompted for this permission.
    NotDetermined,
    /// Permission is restricted by system policy (e.g., parental controls, MDM).
    Restricted,
    /// This permission type is not supported on the current platform.
    #[default]
    Unsupported,
}

/// Types of system permissions the application may request.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PermissionKind {
    /// Permission to capture the screen content.
    ScreenCapture,
}

/// Platform-specific handler for checking and requesting system permissions.
pub trait PlatformPermissionsHandler {
    /// Returns the current status of the specified permission.
    /// This check does not prompt the user.
    fn status(&self, kind: PermissionKind) -> PermissionStatus;

    /// Requests the specified permission from the user.
    /// This may show a system dialog prompting the user to grant access.
    fn request(&self, kind: PermissionKind);

    /// Opens the system settings page for the specified permission type.
    fn open_settings(&self, kind: PermissionKind);
}

/// A no-op permissions handler for platforms that don't support permission management.
/// All permission checks return [`PermissionStatus::Unsupported`].
pub struct DummyPermissionsHandler;

impl PlatformPermissionsHandler for DummyPermissionsHandler {
    fn status(&self, _kind: PermissionKind) -> PermissionStatus {
        PermissionStatus::Unsupported
    }

    fn request(&self, _kind: PermissionKind) {}

    fn open_settings(&self, _kind: PermissionKind) {}
}
