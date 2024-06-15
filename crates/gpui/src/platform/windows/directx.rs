use std::sync::Arc;

use crate::{DirectXAtlas, PlatformAtlas};

pub(crate) struct DirectXRenderer {
    atlas: Arc<DirectXAtlas>,
}

impl DirectXRenderer {
    pub(crate) fn new() -> Self {
        DirectXRenderer {
            atlas: Arc::new(DirectXAtlas::new()),
        }
    }

    pub(crate) fn spirite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.atlas.clone()
    }
}
