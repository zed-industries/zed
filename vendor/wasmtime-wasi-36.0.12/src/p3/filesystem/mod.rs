use crate::{DirPerms, FilePerms, OpenMode};

// TODO: implement
pub struct Dir;

impl Dir {
    #[expect(unused)]
    pub fn new(
        dir: cap_std::fs::Dir,
        perms: DirPerms,
        file_perms: FilePerms,
        open_mode: OpenMode,
        allow_blocking_current_thread: bool,
    ) -> Self {
        Self
    }
}
