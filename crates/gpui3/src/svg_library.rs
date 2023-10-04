use crate::{AssetSource, DevicePixels, ImageData, IsZero, Result, SharedString, Size};
use anyhow::anyhow;
use collections::HashMap;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use std::hash::Hash;
use std::sync::Arc;
use usvg::Tree as SvgTree;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct SvgRenderParams {
    path: SharedString,
    size: Size<DevicePixels>,
}

pub struct SvgRenderer {
    asset_source: Arc<dyn AssetSource>,
    trees_by_path: RwLock<HashMap<SharedString, SvgTree>>,
    rendered: RwLock<HashMap<SvgRenderParams, Arc<ImageData>>>,
}

impl SvgRenderer {
    pub fn render(&self, params: SvgRenderParams) -> Result<Arc<ImageData>> {
        if params.size.is_zero() {
            return Err(anyhow!("can't render at a zero size"));
        }

        let rendered = self.rendered.upgradable_read();
        if let Some(image_data) = rendered.get(&params) {
            Ok(image_data.clone())
        } else {
            // There's no rendered SVG for the path at the requested size.
            // Have we already loaded a tree for the path?
            let trees_by_path = self.trees_by_path.upgradable_read();
            let tree = if let Some(tree) = trees_by_path.get(&params.path) {
                tree.clone()
            } else {
                // Load the tree
                let bytes = self.asset_source.load(&params.path)?;
                let tree = usvg::Tree::from_data(&bytes, &usvg::Options::default())?;
                let mut trees_by_path = RwLockUpgradableReadGuard::upgrade(trees_by_path);
                trees_by_path.insert(params.path.clone(), tree.clone());
                tree
            };

            // Render the SVG to a pixmap with the specified width and height.
            // Convert the pixmap's pixels into an image data and cache it in `rendered`.
            let mut pixmap =
                tiny_skia::Pixmap::new(params.size.width.into(), params.size.height.into())
                    .unwrap();
            resvg::render(
                &tree,
                usvg::FitTo::Width(params.size.width.into()),
                pixmap.as_mut(),
            );
            let alpha_mask = pixmap
                .pixels()
                .iter()
                .map(|p| p.alpha())
                .collect::<Vec<_>>();
            let mut rendered = RwLockUpgradableReadGuard::upgrade(rendered);
            let image_data = Arc::new(ImageData::from_raw(params.size, alpha_mask));
            rendered.insert(params, image_data.clone());

            Ok(image_data)
        }
    }
}

// impl SvgRenderer {
//     pub fn render_svg(
//         &mut self,
//         size: Vector2I,
//         path: Cow<'static, str>,
//         svg: usvg::Tree,
//     ) -> Option<IconSprite> {
//         let mut pixmap = tiny_skia::Pixmap::new(size.x() as u32, size.y() as u32)?;
//         resvg::render(&svg, usvg::FitTo::Width(size.x() as u32), pixmap.as_mut());

//         let atlases = &mut self.atlases;
//         match self.icons.entry(IconDescriptor {
//             path,
//             width: size.x(),
//             height: size.y(),
//         }) {
//             Entry::Occupied(entry) => Some(entry.get().clone()),
//             Entry::Vacant(entry) => {
//                 let mask = pixmap
//                     .pixels()
//                     .iter()
//                     .map(|a| a.alpha())
//                     .collect::<Vec<_>>();
//                 let (alloc_id, atlas_bounds) = atlases.upload(size, &mask)?;
//                 let icon_sprite = IconSprite {
//                     atlas_id: alloc_id.atlas_id,
//                     atlas_origin: atlas_bounds.origin(),
//                     size,
//                 };
//                 Some(entry.insert(icon_sprite).clone())
//             }
//         }
//     }
// }
