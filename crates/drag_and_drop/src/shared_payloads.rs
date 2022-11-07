use std::{path::Path, sync::Arc};

#[derive(Debug, Clone)]
pub struct DraggedProjectEntry {
    pub path: Arc<Path>,
}
