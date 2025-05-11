use std::{io::Cursor, sync::Arc};

use anyhow::Result;
use collections::HashMap;
use gpui::{App, AssetSource, Global};
use rodio::{
    Decoder, Source,
    source::{Buffered, SamplesConverter},
};

type Sound = Buffered<SamplesConverter<Decoder<Cursor<Vec<u8>>>, f32>>;

pub struct SoundRegistry {
    cache: Arc<parking_lot::Mutex<HashMap<String, Sound>>>,
    assets: Box<dyn AssetSource>,
}

struct GlobalSoundRegistry(Arc<SoundRegistry>);

impl Global for GlobalSoundRegistry {}

impl SoundRegistry {
    pub fn new(source: impl AssetSource) -> Arc<Self> {
        Arc::new(Self {
            cache: Default::default(),
            assets: Box::new(source),
        })
    }

    pub fn global(cx: &App) -> Arc<Self> {
        cx.global::<GlobalSoundRegistry>().0.clone()
    }

    pub(crate) fn set_global(source: impl AssetSource, cx: &mut App) {
        cx.set_global(GlobalSoundRegistry(SoundRegistry::new(source)));
    }

    pub fn get(&self, name: &str) -> Result<impl Source<Item = f32> + use<>> {
        if let Some(wav) = self.cache.lock().get(name) {
            return Ok(wav.clone());
        }

        let path = format!("sounds/{}.wav", name);
        let bytes = self
            .assets
            .load(&path)?
            .map(Ok)
            .unwrap_or_else(|| Err(anyhow::anyhow!("No such asset available")))?
            .into_owned();
        let cursor = Cursor::new(bytes);
        let source = Decoder::new(cursor)?.convert_samples::<f32>().buffered();

        self.cache.lock().insert(name.to_string(), source.clone());

        Ok(source)
    }
}
