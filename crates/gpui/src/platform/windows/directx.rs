use std::sync::Arc;

use crate::{DevicePixels, DirectXAtlas, PlatformAtlas, Scene, Size, WindowBackgroundAppearance};

pub(crate) struct DirectXRenderer {
    atlas: Arc<DirectXAtlas>,
}

struct DirectXContext {}

impl DirectXRenderer {
    pub(crate) fn new() -> Self {
        DirectXRenderer {
            atlas: Arc::new(DirectXAtlas::new()),
        }
    }

    pub(crate) fn spirite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.atlas.clone()
    }

    pub(crate) fn draw(&mut self, scene: &Scene) {
        // TODO:
    }

    pub(crate) fn resize(&mut self, new_size: Size<DevicePixels>) {
        // TODO:
    }

    pub(crate) fn update_transparency(
        &mut self,
        background_appearance: WindowBackgroundAppearance,
    ) {
        match background_appearance {
            WindowBackgroundAppearance::Opaque => {
                // TODO:
            }
            WindowBackgroundAppearance::Transparent => {
                // TODO:
            }
            WindowBackgroundAppearance::Blurred => {
                // TODO:
            }
        }
    }
}
