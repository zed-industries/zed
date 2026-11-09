use std::{fs::File, io::BufReader, path::Path, sync::Arc};

use anyhow::Result;
use collections::HashSet;
use gpui::{
    App, Img, ImgResourceLoader, InteractiveElement, Resource, Stateful, Styled, StyledImage, img,
};
use image::{
    AnimationDecoder, ImageFormat, ImageReader,
    codecs::{gif::GifDecoder, webp::WebPDecoder},
};
use settings::{
    BackgroundImageFit, BackgroundImageLayer, BackgroundImageOpacity, IntoGpui, RegisterSetting,
    Settings, SettingsContent, SettingsStore,
};

const BYTES_PER_PIXEL: u64 = 4;
const MEMORY_PER_DECODED_BYTE: u64 = 4;

struct BackgroundImage {
    path: Arc<Path>,
    opacity: f32,
    fit: BackgroundImageFit,
    layer: BackgroundImageLayer,
}

#[derive(RegisterSetting)]
pub struct BackgroundImageSettings {
    images: Vec<(String, BackgroundImage)>,
}

pub struct BackgroundLayers {
    pub below: Option<Stateful<Img>>,
    pub above: Option<Stateful<Img>>,
}

impl Settings for BackgroundImageSettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let images = content
            .workspace
            .background_images
            .iter()
            .filter_map(|(target, image)| {
                let opacity = image.opacity.unwrap_or(BackgroundImageOpacity::DEFAULT).0;
                let opacity = opacity.clamp(0., 1.);
                if image.enabled == Some(false) || opacity == 0. {
                    return None;
                }
                let background_image = BackgroundImage {
                    path: resolve_path(&image.path.as_ref()?.0),
                    opacity,
                    fit: image.fit.unwrap_or_default(),
                    layer: image.layer.unwrap_or_default(),
                };
                Some((normalized(target).collect(), background_image))
            })
            .collect();
        Self { images }
    }
}

impl BackgroundImageSettings {
    fn find<'a>(&self, targets: impl IntoIterator<Item = &'a str>) -> Option<&BackgroundImage> {
        targets.into_iter().find_map(|target| {
            self.images
                .iter()
                .find(|(key, _)| key.chars().eq(normalized(target)))
                .map(|(_, image)| image)
        })
    }

    fn paths(&self) -> HashSet<Arc<Path>> {
        self.images
            .iter()
            .map(|(_, image)| image.path.clone())
            .collect()
    }
}

impl BackgroundImage {
    fn render(&self) -> Stateful<Img> {
        img(self.path.clone())
            .id("background-image")
            .absolute()
            .inset_0()
            .size_full()
            .object_fit(self.fit.into_gpui())
            .opacity(self.opacity)
    }
}

pub fn background_layers<'a>(
    targets: impl IntoIterator<Item = &'a str>,
    cx: &App,
) -> BackgroundLayers {
    let image = BackgroundImageSettings::get_global(cx).find(targets);
    let render = |layer| {
        image
            .filter(|image| image.layer == layer)
            .map(BackgroundImage::render)
    };
    BackgroundLayers {
        below: render(BackgroundImageLayer::Below),
        above: render(BackgroundImageLayer::Above),
    }
}

pub fn init(cx: &mut App) {
    let mut previous_paths = BackgroundImageSettings::get_global(cx).paths();
    cx.observe_global::<SettingsStore>(move |cx| {
        let current_paths = BackgroundImageSettings::get_global(cx).paths();
        for path in previous_paths.difference(&current_paths) {
            release(path.clone(), cx);
        }
        previous_paths = current_paths;
    })
    .detach();
}

fn release(path: Arc<Path>, cx: &mut App) {
    let resource = Resource::Path(path);
    if !cx.has_asset::<ImgResourceLoader>(&resource) {
        return;
    }
    if let Some(Ok(image)) = cx.fetch_asset::<ImgResourceLoader>(&resource) {
        cx.drop_image(image, None);
    }
    cx.remove_asset::<ImgResourceLoader>(&resource);
}

pub fn estimate_memory_usage(path: &Path) -> Result<u64> {
    let path = resolve_path(path);
    let reader = ImageReader::open(&path)?.with_guessed_format()?;
    let format = reader.format();
    let (width, height) = reader.into_dimensions()?;
    let frame_count = match format {
        Some(ImageFormat::Gif) => GifDecoder::new(BufReader::new(File::open(&path)?))?
            .into_frames()
            .count(),
        Some(ImageFormat::WebP) => {
            let decoder = WebPDecoder::new(BufReader::new(File::open(&path)?))?;
            if decoder.has_animation() {
                decoder.into_frames().count()
            } else {
                1
            }
        }
        _ => 1,
    };
    Ok(u64::from(width)
        * u64::from(height)
        * BYTES_PER_PIXEL
        * frame_count as u64
        * MEMORY_PER_DECODED_BYTE)
}

fn normalized(target: &str) -> impl Iterator<Item = char> + '_ {
    target
        .chars()
        .filter(|character| *character != '_')
        .map(|character| character.to_ascii_lowercase())
}

fn resolve_path(path: &Path) -> Arc<Path> {
    match path.strip_prefix("~") {
        Ok(relative_to_home) => paths::home_dir().join(relative_to_home),
        Err(_) => paths::config_dir().join(path),
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::UpdateGlobal;
    use image::{Delay, Frame, RgbaImage, codecs::gif::GifEncoder};
    use settings::{BackgroundImageContent, BackgroundImagePath};

    #[test]
    fn test_resolve_path() {
        assert_eq!(
            resolve_path(Path::new("~/wallpaper.png")).as_ref(),
            paths::home_dir().join("wallpaper.png")
        );
        assert_eq!(
            resolve_path(Path::new("backgrounds/wallpaper.gif")).as_ref(),
            paths::config_dir().join("backgrounds/wallpaper.gif")
        );
        let absolute = if cfg!(windows) {
            Path::new("C:\\wallpaper.jpg")
        } else {
            Path::new("/wallpaper.jpg")
        };
        assert_eq!(resolve_path(absolute).as_ref(), absolute);
    }

    #[test]
    fn test_estimate_memory_usage() {
        let directory = tempfile::tempdir().unwrap();

        let still_path = directory.path().join("still.png");
        RgbaImage::new(5, 2).save(&still_path).unwrap();
        assert_eq!(estimate_memory_usage(&still_path).unwrap(), 5 * 2 * 4 * 4);

        let animated_path = directory.path().join("animated.gif");
        let frames = (0..3).map(|_| {
            Frame::from_parts(
                RgbaImage::new(4, 3),
                0,
                0,
                Delay::from_numer_denom_ms(100, 1),
            )
        });
        GifEncoder::new(File::create(&animated_path).unwrap())
            .encode_frames(frames)
            .unwrap();
        assert_eq!(
            estimate_memory_usage(&animated_path).unwrap(),
            4 * 3 * 4 * 3 * 4
        );

        assert!(estimate_memory_usage(&directory.path().join("missing.png")).is_err());
    }

    #[gpui::test]
    fn test_background_image_settings(cx: &mut App) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                let images = &mut settings.workspace.background_images;
                images.insert(
                    "left_dock".into(),
                    BackgroundImageContent {
                        path: Some(BackgroundImagePath("dock.png".into())),
                        ..Default::default()
                    },
                );
                images.insert(
                    "project_panel".into(),
                    BackgroundImageContent {
                        path: Some(BackgroundImagePath("panel.gif".into())),
                        opacity: Some(BackgroundImageOpacity(4.)),
                        layer: Some(BackgroundImageLayer::Below),
                        ..Default::default()
                    },
                );
                images.insert(
                    "status_bar".into(),
                    BackgroundImageContent {
                        opacity: Some(BackgroundImageOpacity(0.5)),
                        ..Default::default()
                    },
                );
                images.insert(
                    "terminal_panel".into(),
                    BackgroundImageContent {
                        path: Some(BackgroundImagePath("terminal.png".into())),
                        opacity: Some(BackgroundImageOpacity(0.)),
                        ..Default::default()
                    },
                );
                images.insert(
                    "center".into(),
                    BackgroundImageContent {
                        enabled: Some(false),
                        path: Some(BackgroundImagePath("center.png".into())),
                        ..Default::default()
                    },
                );
            })
        });

        let settings = BackgroundImageSettings::get_global(cx);

        let panel_image = settings.find(["ProjectPanel", "left_dock"]).unwrap();
        assert_eq!(
            panel_image.path.as_ref(),
            paths::config_dir().join("panel.gif")
        );
        assert_eq!(panel_image.opacity, 1.);
        assert_eq!(panel_image.layer, BackgroundImageLayer::Below);

        let dock_image = settings.find(["GitPanel", "left_dock"]).unwrap();
        assert_eq!(
            dock_image.path.as_ref(),
            paths::config_dir().join("dock.png")
        );
        assert_eq!(dock_image.opacity, BackgroundImageOpacity::DEFAULT.0);
        assert_eq!(dock_image.layer, BackgroundImageLayer::Above);

        assert!(settings.find(["status_bar"]).is_none());
        assert!(settings.find(["TerminalPanel"]).is_none());
        assert!(settings.find(["center"]).is_none());
    }
}
