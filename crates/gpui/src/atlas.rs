use crate::{AtlasKey, AtlasTile, DevicePixels, PlatformAtlas, Size};
use anyhow::Result;
use collections::FxHashMap;
use std::{borrow::Cow, sync::Arc};

pub struct Atlas {
    platform_atlas: Arc<dyn PlatformAtlas>,
    tiles: FxHashMap<AtlasKey, AtlasTile>,
}

impl Atlas {
    pub fn new(platform_atlas: Arc<dyn PlatformAtlas>) -> Self {
        Self {
            platform_atlas,
            tiles: FxHashMap::default(),
        }
    }

    pub fn get_or_insert_with<'a>(
        &mut self,
        key: &AtlasKey,
        build: impl FnOnce() -> Result<(Size<DevicePixels>, Cow<'a, [u8]>)>,
    ) -> Result<AtlasTile> {
        if let Some(tile) = self.tiles.get(key) {
            Ok(tile.clone())
        } else {
            let (size, bytes) = build()?;
            let tile = self.platform_atlas.insert(key, size, bytes);
            self.tiles.insert(key.clone(), tile.clone());
            Ok(tile)
        }
    }
}
