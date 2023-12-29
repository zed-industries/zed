use std::sync::Arc;

use anyhow::anyhow;
use gpui::{AppContext, AssetSource, Result, SharedString};
use parking_lot::Mutex;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "fonts/**/*"]
#[include = "icons/**/*"]
#[include = "themes/**/*"]
#[exclude = "themes/src/*"]
#[include = "sounds/**/*"]
#[include = "*.md"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<std::borrow::Cow<[u8]>> {
        Self::get(path)
            .map(|f| f.data)
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| {
                if p.starts_with(path) {
                    Some(p.into())
                } else {
                    None
                }
            })
            .collect())
    }
}

impl Assets {
    pub fn load_embedded_fonts(&self, app: &AppContext) {
        let font_paths = self.list("fonts").unwrap();
        let embedded_fonts = Mutex::new(Vec::new());

        app.background_executor()
            .block(app.background_executor().scoped(|scope| {
                for font_path in &font_paths {
                    if !font_path.ends_with(".ttf") {
                        continue;
                    }

                    scope.spawn(async {
                        let font_path = &*font_path;
                        let font_bytes = Assets.load(font_path).unwrap().to_vec();
                        embedded_fonts.lock().push(Arc::from(font_bytes));
                    });
                }
            }));

        app.text_system()
            .add_fonts(&embedded_fonts.into_inner())
            .unwrap();
    }
}
