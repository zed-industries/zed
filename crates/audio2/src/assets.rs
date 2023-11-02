use std::{io::Cursor, sync::Arc};

use anyhow::Result;
use collections::HashMap;
use gpui::{AppContext, AssetSource};
use rodio::{
    source::{Buffered, SamplesConverter},
    Decoder, Source,
};

type Sound = Buffered<SamplesConverter<Decoder<Cursor<Vec<u8>>>, f32>>;

pub struct SoundRegistry {
    cache: Arc<parking_lot::Mutex<HashMap<String, Sound>>>,
    assets: Box<dyn AssetSource>,
}

impl SoundRegistry {
    pub fn new(source: impl AssetSource) -> Arc<Self> {
        Arc::new(Self {
            cache: Default::default(),
            assets: Box::new(source),
        })
    }

    pub fn global(cx: &AppContext) -> Arc<Self> {
        cx.global::<Arc<Self>>().clone()
    }

    pub fn get(&self, name: &str) -> Result<impl Source<Item = f32>> {
        if let Some(wav) = self.cache.lock().get(name) {
            return Ok(wav.clone());
        }

        let path = format!("sounds/{}.wav", name);
        let bytes = self.assets.load(&path)?.into_owned();
        let cursor = Cursor::new(bytes);
        let source = Decoder::new(cursor)?.convert_samples::<f32>().buffered();

        self.cache.lock().insert(name.to_string(), source.clone());

        Ok(source)
    }
}
