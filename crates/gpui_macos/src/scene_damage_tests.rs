#[cfg(test)]
mod tests {
    use crate::metal_renderer::MetalHeadlessRenderer;
    use cocoa::{base::nil, foundation::NSAutoreleasePool};
    use gpui::{
        AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTile, Background, Bounds, ContentMask,
        Corners, DevicePixels, Edges, Hsla, ImageId, MonochromeSprite, PaddedBool32, Path, Pixels,
        PlatformAtlas, PlatformHeadlessRenderer, Point, PolychromeSprite, Primitive, Quad,
        RenderImageParams, RenderSvgParams, ScaledPixels, Scene, SceneDamage, Shadow, SharedString,
        Size, TileId, TransformationMatrix, Underline, px,
    };
    use image::RgbaImage;
    use proptest::prelude::*;
    use std::{borrow::Cow, cell::RefCell};

    const IMAGE_SIZE: u32 = 96;
    const MAX_PRIMITIVES: usize = 64;
    const PROPTEST_CASES: u32 = 1024;

    thread_local! {
        static RENDERER: RefCell<MetalHeadlessRenderer> = RefCell::new(MetalHeadlessRenderer::new());
    }

    #[derive(Clone, Debug)]
    enum Mutation {
        ReplaceAll(PrimitiveList),
        Insert { index: usize, primitive: Primitive },
        Remove { index: usize },
        Replace { index: usize, primitive: Primitive },
        Edit { index: usize, edit: PrimitiveEdit },
        Swap { first: usize, second: usize },
        Reverse,
        Noop,
    }

    #[derive(Clone, Debug)]
    enum PrimitiveEdit {
        Translate { dx: i32, dy: i32 },
        SetBounds(Bounds<ScaledPixels>),
        SetContentMask(Bounds<ScaledPixels>),
        SetColor(Hsla),
        ToggleStyle,
    }

    #[derive(Clone, Debug)]
    struct PrimitiveList(Vec<Primitive>);

    fn point(x: f32, y: f32) -> Point<Pixels> {
        Point { x: px(x), y: px(y) }
    }

    fn texture_point(x: f32, y: f32) -> Point<f32> {
        Point { x, y }
    }

    fn rect(x: f32, y: f32, width: f32, height: f32) -> Bounds<ScaledPixels> {
        Bounds {
            origin: Point {
                x: ScaledPixels(x),
                y: ScaledPixels(y),
            },
            size: Size {
                width: ScaledPixels(width),
                height: ScaledPixels(height),
            },
        }
    }

    fn device_rect(x: i32, y: i32, width: i32, height: i32) -> Bounds<DevicePixels> {
        Bounds {
            origin: Point {
                x: DevicePixels(x),
                y: DevicePixels(y),
            },
            size: Size {
                width: DevicePixels(width),
                height: DevicePixels(height),
            },
        }
    }

    fn full_mask() -> ContentMask<ScaledPixels> {
        ContentMask {
            bounds: rect(0.0, 0.0, IMAGE_SIZE as f32, IMAGE_SIZE as f32),
        }
    }

    fn full_pixel_mask() -> ContentMask<Pixels> {
        ContentMask {
            bounds: Bounds {
                origin: point(0.0, 0.0),
                size: Size {
                    width: px(IMAGE_SIZE as f32),
                    height: px(IMAGE_SIZE as f32),
                },
            },
        }
    }

    fn corners(radius: ScaledPixels) -> Corners<ScaledPixels> {
        Corners {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }

    fn edges(width: ScaledPixels) -> Edges<ScaledPixels> {
        Edges {
            top: width,
            right: width,
            bottom: width,
            left: width,
        }
    }

    fn placeholder_tile(tile_index: usize, kind: AtlasTextureKind) -> AtlasTile {
        AtlasTile {
            texture_id: AtlasTextureId {
                index: tile_index as u32,
                kind,
            },
            tile_id: TileId(tile_index as u32),
            padding: 0,
            bounds: device_rect(0, 0, 8, 8),
        }
    }

    fn polychrome_sprite_tile(atlas: &dyn PlatformAtlas, tile_index: usize) -> AtlasTile {
        let key = AtlasKey::Image(RenderImageParams {
            image_id: ImageId(tile_index),
            frame_index: 0,
        });
        atlas
            .get_or_insert_with(&key, &mut || {
                let size = Size {
                    width: DevicePixels(8),
                    height: DevicePixels(8),
                };
                let channel = (tile_index as u8).wrapping_mul(47).max(32);
                let mut bytes =
                    Vec::with_capacity(size.width.0 as usize * size.height.0 as usize * 4);
                for _ in 0..size.width.0 * size.height.0 {
                    // Metal atlas textures for images are BGRA.
                    bytes.extend_from_slice(&[channel, 255u8.wrapping_sub(channel), 192, 255]);
                }
                Ok(Some((size, Cow::Owned(bytes))))
            })
            .expect("failed to allocate test sprite tile")
            .expect("test sprite tile builder returned None")
    }

    fn monochrome_sprite_tile(atlas: &dyn PlatformAtlas, tile_index: usize) -> AtlasTile {
        let key = AtlasKey::Svg(RenderSvgParams {
            path: SharedString::new(format!("scene-damage-test-{tile_index}")),
            size: Size {
                width: DevicePixels(8),
                height: DevicePixels(8),
            },
        });
        atlas
            .get_or_insert_with(&key, &mut || {
                let size = Size {
                    width: DevicePixels(8),
                    height: DevicePixels(8),
                };
                Ok(Some((
                    size,
                    Cow::Owned(vec![255; size.width.0 as usize * size.height.0 as usize]),
                )))
            })
            .expect("failed to allocate test monochrome sprite tile")
            .expect("test monochrome sprite tile builder returned None")
    }

    fn resolve_atlas_tiles(primitive: &Primitive, atlas: &dyn PlatformAtlas) -> Primitive {
        match primitive {
            Primitive::MonochromeSprite(sprite) => {
                let mut sprite = *sprite;
                sprite.tile = monochrome_sprite_tile(atlas, sprite.tile.tile_id.0 as usize);
                Primitive::MonochromeSprite(sprite)
            }
            Primitive::PolychromeSprite(sprite) => {
                let mut sprite = *sprite;
                sprite.tile = polychrome_sprite_tile(atlas, sprite.tile.tile_id.0 as usize);
                Primitive::PolychromeSprite(sprite)
            }
            primitive => primitive.clone(),
        }
    }

    fn scene_of(primitives: &[Primitive], atlas: &dyn PlatformAtlas) -> Scene {
        let mut scene = Scene::default();
        for primitive in primitives {
            scene.insert_primitive(resolve_atlas_tiles(primitive, atlas));
        }
        scene.finish();
        scene
    }

    fn color_strategy() -> impl Strategy<Value = Hsla> {
        (0.0f32..=1.0, 0.0f32..=1.0, 0.05f32..=0.95, 0.25f32..=1.0).prop_map(|(h, s, l, a)| Hsla {
            h,
            s,
            l,
            a,
        })
    }

    fn bounds_strategy() -> impl Strategy<Value = Bounds<ScaledPixels>> {
        (0i32..80, 0i32..80, 1i32..36, 1i32..36).prop_map(|(x, y, width, height)| {
            let width = width.min(IMAGE_SIZE as i32 - x);
            let height = height.min(IMAGE_SIZE as i32 - y);
            rect(x as f32, y as f32, width as f32, height as f32)
        })
    }

    fn path_points_strategy() -> impl Strategy<Value = [Point<Pixels>; 3]> {
        (0i32..80, 0i32..80, 4i32..32, 4i32..32).prop_map(|(x, y, width, height)| {
            let width = width.min(IMAGE_SIZE as i32 - x);
            let height = height.min(IMAGE_SIZE as i32 - y);
            [
                point(x as f32, y as f32),
                point((x + width) as f32, y as f32),
                point(x as f32, (y + height) as f32),
            ]
        })
    }

    fn path_primitive(points: [Point<Pixels>; 3], color: Hsla) -> Primitive {
        let mut path = Path::new(points[0]);
        path.color = Background::from(color);
        path.content_mask = full_pixel_mask();
        path.push_triangle(
            (points[0], points[1], points[2]),
            (
                texture_point(0.0, 0.0),
                texture_point(0.5, 0.0),
                texture_point(1.0, 1.0),
            ),
        );
        for vertex in &mut path.vertices {
            vertex.content_mask = full_pixel_mask();
        }
        Primitive::Path(path.scale(1.0))
    }

    fn primitive_strategy() -> impl Strategy<Value = Primitive> {
        prop_oneof![
            (
                bounds_strategy(),
                color_strategy(),
                0i32..=4,
                color_strategy(),
                0i32..=8,
            )
                .prop_map(
                    |(bounds, color, border_width, border_color, corner_radius)| {
                        Primitive::Quad(Quad {
                            bounds,
                            content_mask: full_mask(),
                            background: Background::from(color),
                            border_widths: edges(ScaledPixels(border_width as f32)),
                            border_color,
                            corner_radii: corners(ScaledPixels(corner_radius as f32)),
                            ..Default::default()
                        })
                    }
                ),
            (bounds_strategy(), color_strategy(), 1i32..=4, any::<bool>()).prop_map(
                |(bounds, color, thickness, wavy)| Primitive::Underline(Underline {
                    order: 0,
                    pad: 0,
                    bounds,
                    content_mask: full_mask(),
                    color,
                    thickness: ScaledPixels(thickness as f32),
                    wavy: wavy.into(),
                })
            ),
            (bounds_strategy(), color_strategy(), 0i32..=6, any::<bool>()).prop_map(
                |(bounds, color, blur_radius, inset)| Primitive::Shadow(Shadow {
                    order: 0,
                    blur_radius: ScaledPixels(blur_radius as f32),
                    bounds,
                    corner_radii: Corners::default(),
                    content_mask: full_mask(),
                    color,
                    element_bounds: bounds,
                    element_corner_radii: Corners::default(),
                    inset: u32::from(inset),
                    pad: 0,
                })
            ),
            (path_points_strategy(), color_strategy())
                .prop_map(|(points, color)| path_primitive(points, color)),
            (bounds_strategy(), 0usize..4, 0.25f32..=1.0, any::<bool>()).prop_map(
                |(bounds, tile_index, opacity, grayscale)| {
                    Primitive::PolychromeSprite(PolychromeSprite {
                        order: 0,
                        pad: 0,
                        grayscale: grayscale.into(),
                        opacity,
                        bounds,
                        content_mask: full_mask(),
                        corner_radii: Corners::default(),
                        tile: placeholder_tile(tile_index, AtlasTextureKind::Polychrome),
                    })
                }
            ),
            (bounds_strategy(), 0usize..4, color_strategy()).prop_map(
                |(bounds, tile_index, color)| Primitive::MonochromeSprite(MonochromeSprite {
                    order: 0,
                    pad: 0,
                    bounds,
                    content_mask: full_mask(),
                    color,
                    tile: placeholder_tile(tile_index, AtlasTextureKind::Monochrome),
                    transformation: TransformationMatrix::unit(),
                })
            ),
        ]
    }

    fn primitive_list_strategy() -> impl Strategy<Value = PrimitiveList> {
        prop::collection::vec(primitive_strategy(), 0..=MAX_PRIMITIVES).prop_map(PrimitiveList)
    }

    fn primitive_edit_strategy() -> impl Strategy<Value = PrimitiveEdit> {
        prop_oneof![
            (-24i32..=24, -24i32..=24).prop_map(|(dx, dy)| PrimitiveEdit::Translate { dx, dy }),
            bounds_strategy().prop_map(PrimitiveEdit::SetBounds),
            bounds_strategy().prop_map(PrimitiveEdit::SetContentMask),
            color_strategy().prop_map(PrimitiveEdit::SetColor),
            Just(PrimitiveEdit::ToggleStyle),
        ]
    }

    fn mutation_strategy() -> impl Strategy<Value = Mutation> {
        prop_oneof![
            // Keep full-scene replacement in the mix, but bias heavily toward
            // cases where `next` starts as `previous` and is then mutated.
            1 => primitive_list_strategy().prop_map(Mutation::ReplaceAll),
            4 => (0usize..=MAX_PRIMITIVES, primitive_strategy()).prop_map(
                |(index, primitive)| Mutation::Insert { index, primitive }
            ),
            3 => (0usize..MAX_PRIMITIVES).prop_map(|index| Mutation::Remove { index }),
            4 => (0usize..MAX_PRIMITIVES, primitive_strategy()).prop_map(
                |(index, primitive)| Mutation::Replace { index, primitive }
            ),
            6 => (0usize..MAX_PRIMITIVES, primitive_edit_strategy()).prop_map(
                |(index, edit)| Mutation::Edit { index, edit }
            ),
            2 => (0usize..MAX_PRIMITIVES, 0usize..MAX_PRIMITIVES).prop_map(
                |(first, second)| Mutation::Swap { first, second }
            ),
            2 => Just(Mutation::Reverse),
            1 => Just(Mutation::Noop),
        ]
    }

    fn changed_scene_strategy() -> impl Strategy<Value = (PrimitiveList, PrimitiveList)> {
        (primitive_list_strategy(), mutation_strategy()).prop_map(|(previous, mutation)| {
            let previous = PrimitiveList(scene_primitives_with_overlaps(previous.0));
            let mut next = previous.0.clone();
            apply_mutation(&mut next, mutation);

            (previous, PrimitiveList(next))
        })
    }

    fn apply_mutation(primitives: &mut Vec<Primitive>, mutation: Mutation) {
        match mutation {
            Mutation::ReplaceAll(replacement) => *primitives = replacement.0,
            Mutation::Insert { index, primitive } => {
                if primitives.len() < MAX_PRIMITIVES {
                    let index = index % (primitives.len() + 1);
                    primitives.insert(index, primitive);
                } else if !primitives.is_empty() {
                    let index = index % primitives.len();
                    primitives[index] = primitive;
                }
            }
            Mutation::Remove { index } => {
                if !primitives.is_empty() {
                    let index = index % primitives.len();
                    primitives.remove(index);
                }
            }
            Mutation::Replace { index, primitive } => {
                if primitives.is_empty() {
                    primitives.push(primitive);
                } else {
                    let index = index % primitives.len();
                    primitives[index] = primitive;
                }
            }
            Mutation::Edit { index, edit } => {
                if !primitives.is_empty() {
                    let index = index % primitives.len();
                    edit_primitive(&mut primitives[index], edit);
                }
            }
            Mutation::Swap { first, second } => {
                if primitives.len() > 1 {
                    let first = first % primitives.len();
                    let second = second % primitives.len();
                    primitives.swap(first, second);
                }
            }
            Mutation::Reverse => primitives.reverse(),
            Mutation::Noop => {}
        }
    }

    fn edit_primitive(primitive: &mut Primitive, edit: PrimitiveEdit) {
        match edit {
            PrimitiveEdit::Translate { dx, dy } => translate_primitive(primitive, dx, dy),
            PrimitiveEdit::SetBounds(bounds) => set_primitive_bounds(primitive, bounds),
            PrimitiveEdit::SetContentMask(bounds) => set_primitive_content_mask(primitive, bounds),
            PrimitiveEdit::SetColor(color) => set_primitive_color(primitive, color),
            PrimitiveEdit::ToggleStyle => toggle_primitive_style(primitive),
        }
    }

    fn translate_bounds(bounds: Bounds<ScaledPixels>, dx: i32, dy: i32) -> Bounds<ScaledPixels> {
        Bounds {
            origin: Point {
                x: ScaledPixels(bounds.origin.x.0 + dx as f32),
                y: ScaledPixels(bounds.origin.y.0 + dy as f32),
            },
            size: bounds.size,
        }
    }

    fn translate_point(point: Point<ScaledPixels>, dx: i32, dy: i32) -> Point<ScaledPixels> {
        Point {
            x: ScaledPixels(point.x.0 + dx as f32),
            y: ScaledPixels(point.y.0 + dy as f32),
        }
    }

    fn translate_path(path: &mut Path<ScaledPixels>, dx: i32, dy: i32) {
        path.bounds = translate_bounds(path.bounds, dx, dy);
        for vertex in &mut path.vertices {
            vertex.xy_position = translate_point(vertex.xy_position, dx, dy);
        }
    }

    fn remap_path_to_bounds(path: &mut Path<ScaledPixels>, bounds: Bounds<ScaledPixels>) {
        let old_bounds = path.bounds;
        let old_width = old_bounds.size.width.0;
        let old_height = old_bounds.size.height.0;

        for vertex in &mut path.vertices {
            let x_ratio = if old_width.abs() > f32::EPSILON {
                (vertex.xy_position.x.0 - old_bounds.origin.x.0) / old_width
            } else {
                0.0
            };
            let y_ratio = if old_height.abs() > f32::EPSILON {
                (vertex.xy_position.y.0 - old_bounds.origin.y.0) / old_height
            } else {
                0.0
            };
            vertex.xy_position = Point {
                x: ScaledPixels(bounds.origin.x.0 + bounds.size.width.0 * x_ratio),
                y: ScaledPixels(bounds.origin.y.0 + bounds.size.height.0 * y_ratio),
            };
        }

        path.bounds = bounds;
    }

    fn translate_primitive(primitive: &mut Primitive, dx: i32, dy: i32) {
        match primitive {
            Primitive::Quad(quad) => quad.bounds = translate_bounds(quad.bounds, dx, dy),
            Primitive::Underline(underline) => {
                underline.bounds = translate_bounds(underline.bounds, dx, dy)
            }
            Primitive::Shadow(shadow) => {
                shadow.bounds = translate_bounds(shadow.bounds, dx, dy);
                shadow.element_bounds = translate_bounds(shadow.element_bounds, dx, dy);
            }
            Primitive::Path(path) => translate_path(path, dx, dy),
            Primitive::MonochromeSprite(sprite) => {
                sprite.bounds = translate_bounds(sprite.bounds, dx, dy)
            }
            Primitive::SubpixelSprite(sprite) => {
                sprite.bounds = translate_bounds(sprite.bounds, dx, dy)
            }
            Primitive::PolychromeSprite(sprite) => {
                sprite.bounds = translate_bounds(sprite.bounds, dx, dy)
            }
            Primitive::Surface(surface) => {
                surface.bounds = translate_bounds(surface.bounds, dx, dy)
            }
        }
    }

    fn set_primitive_bounds(primitive: &mut Primitive, bounds: Bounds<ScaledPixels>) {
        match primitive {
            Primitive::Quad(quad) => quad.bounds = bounds,
            Primitive::Underline(underline) => underline.bounds = bounds,
            Primitive::Shadow(shadow) => {
                shadow.bounds = bounds;
                shadow.element_bounds = bounds;
            }
            Primitive::Path(path) => remap_path_to_bounds(path, bounds),
            Primitive::MonochromeSprite(sprite) => sprite.bounds = bounds,
            Primitive::SubpixelSprite(sprite) => sprite.bounds = bounds,
            Primitive::PolychromeSprite(sprite) => sprite.bounds = bounds,
            Primitive::Surface(surface) => surface.bounds = bounds,
        }
    }

    fn set_primitive_content_mask(primitive: &mut Primitive, bounds: Bounds<ScaledPixels>) {
        match primitive {
            Primitive::Quad(quad) => quad.content_mask.bounds = bounds,
            Primitive::Underline(underline) => underline.content_mask.bounds = bounds,
            Primitive::Shadow(shadow) => shadow.content_mask.bounds = bounds,
            Primitive::Path(path) => {
                path.content_mask.bounds = bounds;
                for vertex in &mut path.vertices {
                    vertex.content_mask.bounds = bounds;
                }
            }
            Primitive::MonochromeSprite(sprite) => sprite.content_mask.bounds = bounds,
            Primitive::SubpixelSprite(sprite) => sprite.content_mask.bounds = bounds,
            Primitive::PolychromeSprite(sprite) => sprite.content_mask.bounds = bounds,
            Primitive::Surface(surface) => surface.content_mask.bounds = bounds,
        }
    }

    fn set_primitive_color(primitive: &mut Primitive, color: Hsla) {
        match primitive {
            Primitive::Quad(quad) => {
                quad.background = Background::from(color);
                quad.border_color = color;
            }
            Primitive::Underline(underline) => underline.color = color,
            Primitive::Shadow(shadow) => shadow.color = color,
            Primitive::Path(path) => path.color = Background::from(color),
            Primitive::MonochromeSprite(sprite) => sprite.color = color,
            Primitive::SubpixelSprite(sprite) => sprite.color = color,
            Primitive::PolychromeSprite(sprite) => {
                sprite.opacity = color.a.max(0.1);
                sprite.grayscale = (color.s < 0.5).into();
            }
            Primitive::Surface(_) => {}
        }
    }

    fn toggle_primitive_style(primitive: &mut Primitive) {
        match primitive {
            Primitive::Quad(quad) => {
                let width = if quad.border_widths.top.0 == 0.0 {
                    2.0
                } else {
                    0.0
                };
                quad.border_widths = edges(ScaledPixels(width));
                quad.corner_radii = corners(ScaledPixels(width * 2.0));
            }
            Primitive::Underline(underline) => toggle_padded_bool(&mut underline.wavy),
            Primitive::Shadow(shadow) => {
                shadow.inset = 0;
                shadow.blur_radius = ScaledPixels(6.0);
                shadow.bounds.size.width = shadow.bounds.size.width.max(ScaledPixels(12.0));
                shadow.bounds.size.height = shadow.bounds.size.height.max(ScaledPixels(12.0));
                shadow.element_bounds = shadow.bounds;
                shadow.color = Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.0,
                    a: 0.8,
                };
            }
            Primitive::Path(path) => {
                path.color = Background::from(Hsla {
                    h: 0.75,
                    s: 0.8,
                    l: 0.5,
                    a: 1.0,
                });
            }
            Primitive::MonochromeSprite(sprite) => {
                sprite.color.a = if sprite.color.a < 1.0 { 1.0 } else { 0.5 };
            }
            Primitive::SubpixelSprite(sprite) => {
                sprite.color.a = if sprite.color.a < 1.0 { 1.0 } else { 0.5 };
            }
            Primitive::PolychromeSprite(sprite) => toggle_padded_bool(&mut sprite.grayscale),
            Primitive::Surface(_) => {}
        }
    }

    fn toggle_padded_bool(value: &mut PaddedBool32) {
        *value = (*value == PaddedBool32::from(false)).into();
    }

    fn overlapping_quad_primitive(bounds: Bounds<ScaledPixels>, color: Hsla) -> Primitive {
        Primitive::Quad(Quad {
            bounds,
            content_mask: full_mask(),
            background: Background::from(color),
            ..Default::default()
        })
    }

    fn scene_primitives_with_overlaps(mut primitives: Vec<Primitive>) -> Vec<Primitive> {
        if primitives.len() + 2 <= MAX_PRIMITIVES {
            primitives.push(overlapping_quad_primitive(
                rect(10.0, 10.0, 42.0, 42.0),
                Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.2,
                    a: 1.0,
                },
            ));
            primitives.push(overlapping_quad_primitive(
                rect(36.0, 36.0, 42.0, 42.0),
                Hsla {
                    h: 0.6,
                    s: 0.8,
                    l: 0.5,
                    a: 1.0,
                },
            ));
        }
        primitives
    }

    fn rendered_diff_bounds(
        previous: &RgbaImage,
        next: &RgbaImage,
    ) -> Option<(u32, u32, u32, u32)> {
        let mut min_x = u32::MAX;
        let mut min_y = u32::MAX;
        let mut max_x = 0;
        let mut max_y = 0;

        for y in 0..previous.height() {
            for x in 0..previous.width() {
                if previous.get_pixel(x, y) != next.get_pixel(x, y) {
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x + 1);
                    max_y = max_y.max(y + 1);
                }
            }
        }

        (min_x != u32::MAX).then_some((min_x, min_y, max_x, max_y))
    }

    fn damage_rect_bounds(damage: SceneDamage) -> Option<(u32, u32, u32, u32)> {
        match damage {
            SceneDamage::Full => Some((0, 0, IMAGE_SIZE, IMAGE_SIZE)),
            SceneDamage::Unchanged => None,
            SceneDamage::Rect(rect) => {
                let left = rect.origin.x.0.floor().max(0.0) as u32;
                let top = rect.origin.y.0.floor().max(0.0) as u32;
                let right = (rect.origin.x.0 + rect.size.width.0).ceil().max(0.0) as u32;
                let bottom = (rect.origin.y.0 + rect.size.height.0).ceil().max(0.0) as u32;
                Some((
                    left.min(IMAGE_SIZE),
                    top.min(IMAGE_SIZE),
                    right.min(IMAGE_SIZE),
                    bottom.min(IMAGE_SIZE),
                ))
            }
        }
    }

    fn damage_is_well_formed(damage: SceneDamage) -> bool {
        match damage {
            SceneDamage::Full | SceneDamage::Unchanged => true,
            SceneDamage::Rect(rect) => {
                rect.origin.x.0.is_finite()
                    && rect.origin.y.0.is_finite()
                    && rect.size.width.0.is_finite()
                    && rect.size.height.0.is_finite()
                    && rect.size.width.0 > 0.0
                    && rect.size.height.0 > 0.0
            }
        }
    }

    fn damage_contains_diff(damage: SceneDamage, diff: (u32, u32, u32, u32)) -> bool {
        match damage_rect_bounds(damage) {
            Some((left, top, right, bottom)) => {
                left <= diff.0 && top <= diff.1 && right >= diff.2 && bottom >= diff.3
            }
            None => false,
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: PROPTEST_CASES,
            failure_persistence: None,
            ..ProptestConfig::default()
        })]

        #[test]
        fn scene_damage_covers_rendered_pixel_difference((previous_primitives, next_primitives) in changed_scene_strategy()) {
            let _autorelease_pool = unsafe { NSAutoreleasePool::new(nil) };

            RENDERER.with(|renderer| {
                let mut renderer = renderer.borrow_mut();
                let atlas = renderer.sprite_atlas();
                let previous_scene = scene_of(&previous_primitives.0, atlas.as_ref());
                let next_scene = scene_of(&next_primitives.0, atlas.as_ref());
                let damage = SceneDamage::between(&previous_scene, &next_scene);
                let reverse_damage = SceneDamage::between(&next_scene, &previous_scene);

                prop_assert!(
                    damage_is_well_formed(damage),
                    "damage {damage:?} was not well formed; previous={previous_primitives:?}; next={next_primitives:?}"
                );
                prop_assert!(
                    damage_is_well_formed(reverse_damage),
                    "reverse damage {reverse_damage:?} was not well formed; previous={previous_primitives:?}; next={next_primitives:?}"
                );

                let image_size = Size {
                    width: DevicePixels(IMAGE_SIZE as i32),
                    height: DevicePixels(IMAGE_SIZE as i32),
                };
                let previous_image = renderer
                    .render_scene_to_image(&previous_scene, image_size)
                    .expect("failed to render previous scene");
                let next_image = renderer
                    .render_scene_to_image(&next_scene, image_size)
                    .expect("failed to render next scene");

                let rendered_diff = rendered_diff_bounds(&previous_image, &next_image);

                prop_assert!(
                    !matches!(damage, SceneDamage::Unchanged) || rendered_diff.is_none(),
                    "damage was unchanged even though rendered pixel diff was {rendered_diff:?}; previous={previous_primitives:?}; next={next_primitives:?}"
                );
                prop_assert!(
                    !matches!(reverse_damage, SceneDamage::Unchanged) || rendered_diff.is_none(),
                    "reverse damage was unchanged even though rendered pixel diff was {rendered_diff:?}; previous={previous_primitives:?}; next={next_primitives:?}"
                );

                match rendered_diff {
                    Some(diff) => {
                        prop_assert!(
                            damage_contains_diff(damage, diff),
                            "damage {damage:?} did not cover rendered pixel diff {diff:?}; previous={previous_primitives:?}; next={next_primitives:?}"
                        );
                        prop_assert!(
                            damage_contains_diff(reverse_damage, diff),
                            "reverse damage {reverse_damage:?} did not cover rendered pixel diff {diff:?}; previous={previous_primitives:?}; next={next_primitives:?}"
                        );

                        // Future optimal damage tracking should be able to assert that the reported
                        // damage is exactly the rendered pixel diff, not just a conservative superset.
                        // prop_assert_eq!(damage_rect_bounds(damage), Some(diff));
                    }
                    None => {
                        // Future visibility-aware damage tracking should be able to assert the other
                        // direction too: if the final rendered pixels are identical, no damage should
                        // be reported. The current naive scene diff may over-report for occluded or
                        // visually equivalent scene changes.
                        // prop_assert!(matches!(damage, SceneDamage::Unchanged));
                    }
                }

                Ok(())
            })?
        }
    }
}
