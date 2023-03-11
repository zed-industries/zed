use anyhow::{anyhow, Result};
use image::ImageFormat;
use std::{borrow::Cow, cell::RefCell, collections::HashMap, sync::Arc};

use crate::ImageData;

pub trait AssetSource: 'static + Send + Sync {
    fn load(&self, path: &str) -> Result<Cow<[u8]>>;
    fn list(&self, path: &str) -> Vec<Cow<'static, str>>;
}

impl AssetSource for () {
    fn load(&self, path: &str) -> Result<Cow<[u8]>> {
        Err(anyhow!(
            "get called on empty asset provider with \"{}\"",
            path
        ))
    }

    fn list(&self, _: &str) -> Vec<Cow<'static, str>> {
        vec![]
    }
}

pub struct AssetCache {
    source: Box<dyn AssetSource>,
    svgs: RefCell<HashMap<String, usvg::Tree>>,
    pngs: RefCell<HashMap<String, Arc<ImageData>>>,
}

impl AssetCache {
    pub fn new(source: impl AssetSource) -> Self {
        Self {
            source: Box::new(source),
            svgs: RefCell::new(HashMap::new()),
            pngs: RefCell::new(HashMap::new()),
        }
    }

    pub fn svg(&self, path: &str) -> Result<usvg::Tree> {
        let mut svgs = self.svgs.borrow_mut();
        if let Some(svg) = svgs.get(path) {
            Ok(svg.clone())
        } else {
            let bytes = self.source.load(path)?;
            let svg = usvg::Tree::from_data(&bytes, &usvg::Options::default())?;
            svgs.insert(path.to_string(), svg.clone());
            Ok(svg)
        }
    }

    pub fn png(&self, path: &str) -> Result<Arc<ImageData>> {
        let mut pngs = self.pngs.borrow_mut();
        if let Some(png) = pngs.get(path) {
            Ok(png.clone())
        } else {
            let bytes = self.source.load(path)?;
            let image = ImageData::new(
                image::load_from_memory_with_format(&bytes, ImageFormat::Png)?.into_bgra8(),
            );
            pngs.insert(path.to_string(), image.clone());
            Ok(image)
        }
    }
}
