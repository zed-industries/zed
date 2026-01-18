use project::ProjectPath;
// use trash::FileInTrash;

/// Operation done in the project panel that can be undone.
///
/// There is no variant for creating a file or copying a file because their
/// reverse is `Trash`.
///
/// - `Trash` and `Restore` are the reverse of each other.
/// - `Rename` is its own reverse.
pub enum ProjectPanelOperation {
    // Trash(RelPath),
    // Restore(FileInTrashId),
    Rename {
        old_path: ProjectPath,
        new_path: ProjectPath,
    },
}

// pub struct FileInTrashId(u32);

// proto::Trash -> opaque integer
