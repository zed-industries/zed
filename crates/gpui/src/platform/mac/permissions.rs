use crate::{PermissionKind, PermissionStatus, PlatformPermissionsHandler};

pub struct MacPermissionsHandler;

impl PlatformPermissionsHandler for MacPermissionsHandler {
    fn status(&self, kind: PermissionKind) -> PermissionStatus {
        todo!()
    }

    fn request(&self, kind: PermissionKind) {
        todo!()
    }

    fn open_settings(&self, kind: PermissionKind) {
        todo!()
    }
}
