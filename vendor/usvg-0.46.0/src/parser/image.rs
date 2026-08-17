// Copyright 2018 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::sync::Arc;

use svgtypes::{AspectRatio, Length};

use super::svgtree::{AId, SvgNode};
use super::{OptionLog, Options, converter};
use crate::{
    ClipPath, Group, Image, ImageKind, ImageRendering, Node, NonZeroRect, Path, Size, Transform,
    Tree, Visibility,
};

/// A shorthand for [ImageHrefResolver]'s data function.
pub type ImageHrefDataResolverFn<'a> =
    Box<dyn Fn(&str, Arc<Vec<u8>>, &Options) -> Option<ImageKind> + Send + Sync + 'a>;

/// A shorthand for [ImageHrefResolver]'s string function.
pub type ImageHrefStringResolverFn<'a> =
    Box<dyn Fn(&str, &Options) -> Option<ImageKind> + Send + Sync + 'a>;

/// An `xlink:href` resolver for `<image>` elements.
///
/// This type can be useful if you want to have an alternative `xlink:href` handling
/// to the default one. For example, you can forbid access to local files (which is allowed by default)
/// or add support for resolving actual URLs (usvg doesn't do any network requests).
pub struct ImageHrefResolver<'a> {
    /// Resolver function that will be used when `xlink:href` contains a
    /// [Data URL](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs).
    ///
    /// A function would be called with mime, decoded base64 data and parsing options.
    pub resolve_data: ImageHrefDataResolverFn<'a>,

    /// Resolver function that will be used to handle an arbitrary string in `xlink:href`.
    pub resolve_string: ImageHrefStringResolverFn<'a>,
}

impl Default for ImageHrefResolver<'_> {
    fn default() -> Self {
        ImageHrefResolver {
            resolve_data: ImageHrefResolver::default_data_resolver(),
            resolve_string: ImageHrefResolver::default_string_resolver(),
        }
    }
}

impl ImageHrefResolver<'_> {
    /// Creates a default
    /// [Data URL](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs)
    /// resolver closure.
    ///
    /// base64 encoded data is already decoded.
    ///
    /// The default implementation would try to load JPEG, PNG, GIF, WebP, SVG and SVGZ types.
    /// Note that it will simply match the `mime` or data's magic.
    /// The actual images would not be decoded. It's up to the renderer.
    pub fn default_data_resolver() -> ImageHrefDataResolverFn<'static> {
        Box::new(
            move |mime: &str, data: Arc<Vec<u8>>, opts: &Options| match mime {
                "image/jpg" | "image/jpeg" => Some(ImageKind::JPEG(data)),
                "image/png" => Some(ImageKind::PNG(data)),
                "image/gif" => Some(ImageKind::GIF(data)),
                "image/webp" => Some(ImageKind::WEBP(data)),
                "image/svg+xml" => load_sub_svg(&data, opts),
                "text/plain" => match get_image_data_format(&data) {
                    Some(ImageFormat::JPEG) => Some(ImageKind::JPEG(data)),
                    Some(ImageFormat::PNG) => Some(ImageKind::PNG(data)),
                    Some(ImageFormat::GIF) => Some(ImageKind::GIF(data)),
                    Some(ImageFormat::WEBP) => Some(ImageKind::WEBP(data)),
                    _ => load_sub_svg(&data, opts),
                },
                _ => None,
            },
        )
    }

    /// Creates a default string resolver.
    ///
    /// The default implementation treats an input string as a file path and tries to open.
    /// If a string is an URL or something else it would be ignored.
    ///
    /// Paths have to be absolute or relative to the input SVG file or relative to
    /// [Options::resources_dir](crate::Options::resources_dir).
    pub fn default_string_resolver() -> ImageHrefStringResolverFn<'static> {
        Box::new(move |href: &str, opts: &Options| {
            let path = opts.get_abs_path(std::path::Path::new(href));

            if path.exists() {
                let data = match std::fs::read(&path) {
                    Ok(data) => data,
                    Err(_) => {
                        log::warn!("Failed to load '{}'. Skipped.", href);
                        return None;
                    }
                };

                match get_image_file_format(&path, &data) {
                    Some(ImageFormat::JPEG) => Some(ImageKind::JPEG(Arc::new(data))),
                    Some(ImageFormat::PNG) => Some(ImageKind::PNG(Arc::new(data))),
                    Some(ImageFormat::GIF) => Some(ImageKind::GIF(Arc::new(data))),
                    Some(ImageFormat::WEBP) => Some(ImageKind::WEBP(Arc::new(data))),
                    Some(ImageFormat::SVG) => load_sub_svg(&data, opts),
                    _ => {
                        log::warn!("'{}' is not a PNG, JPEG, GIF, WebP or SVG(Z) image.", href);
                        None
                    }
                }
            } else {
                log::warn!("'{}' is not a path to an image.", href);
                None
            }
        })
    }
}

impl std::fmt::Debug for ImageHrefResolver<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ImageHrefResolver { .. }")
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ImageFormat {
    PNG,
    JPEG,
    GIF,
    WEBP,
    SVG,
}

pub(crate) fn convert(
    node: SvgNode,
    state: &converter::State,
    cache: &mut converter::Cache,
    parent: &mut Group,
) -> Option<()> {
    let href = node
        .try_attribute(AId::Href)
        .log_none(|| log::warn!("Image lacks the 'xlink:href' attribute. Skipped."))?;

    let kind = get_href_data(href, state)?;

    let visibility: Visibility = node.find_attribute(AId::Visibility).unwrap_or_default();
    let visible = visibility == Visibility::Visible;

    let rendering_mode = node
        .find_attribute(AId::ImageRendering)
        .unwrap_or(state.opt.image_rendering);

    // Nodes generated by markers must not have an ID. Otherwise we would have duplicates.
    let id = if state.parent_markers.is_empty() {
        node.element_id().to_string()
    } else {
        String::new()
    };

    let actual_size = kind.actual_size()?;

    let x = node.convert_user_length(AId::X, state, Length::zero());
    let y = node.convert_user_length(AId::Y, state, Length::zero());
    let mut width = node.convert_user_length(
        AId::Width,
        state,
        Length::new_number(actual_size.width() as f64),
    );
    let mut height = node.convert_user_length(
        AId::Height,
        state,
        Length::new_number(actual_size.height() as f64),
    );

    match (
        node.attribute::<Length>(AId::Width),
        node.attribute::<Length>(AId::Height),
    ) {
        (Some(_), None) => {
            // Only width was defined, so we need to scale height accordingly.
            height = actual_size.height() * (width / actual_size.width());
        }
        (None, Some(_)) => {
            // Only height was defined, so we need to scale width accordingly.
            width = actual_size.width() * (height / actual_size.height());
        }
        _ => {}
    };

    let aspect: AspectRatio = node.attribute(AId::PreserveAspectRatio).unwrap_or_default();

    let rect = NonZeroRect::from_xywh(x, y, width, height);
    let rect = rect.log_none(|| log::warn!("Image has an invalid size. Skipped."))?;

    convert_inner(
        kind,
        id,
        visible,
        rendering_mode,
        aspect,
        actual_size,
        rect,
        cache,
        parent,
    )
}

pub(crate) fn convert_inner(
    kind: ImageKind,
    id: String,
    visible: bool,
    rendering_mode: ImageRendering,
    aspect: AspectRatio,
    actual_size: Size,
    rect: NonZeroRect,
    cache: &mut converter::Cache,
    parent: &mut Group,
) -> Option<()> {
    let aligned_size = fit_view_box(actual_size, rect, aspect);
    let (aligned_x, aligned_y) = crate::aligned_pos(
        aspect.align,
        rect.x(),
        rect.y(),
        rect.width() - aligned_size.width(),
        rect.height() - aligned_size.height(),
    );
    let view_box = aligned_size.to_non_zero_rect(aligned_x, aligned_y);

    let image_ts = Transform::from_row(
        view_box.width() / actual_size.width(),
        0.0,
        0.0,
        view_box.height() / actual_size.height(),
        view_box.x(),
        view_box.y(),
    );

    let abs_transform = parent.abs_transform.pre_concat(image_ts);
    let abs_bounding_box = view_box.transform(parent.abs_transform)?;

    let mut g = Group::empty();
    g.id = id;
    g.children.push(Node::Image(Box::new(Image {
        id: String::new(),
        visible,
        size: actual_size,
        rendering_mode,
        kind,
        abs_transform,
        abs_bounding_box,
    })));
    g.transform = image_ts;
    g.abs_transform = abs_transform;
    g.calculate_bounding_boxes();

    if aspect.slice {
        // Image slice acts like a rectangular clip.
        let mut path = Path::new_simple(Arc::new(tiny_skia_path::PathBuilder::from_rect(
            rect.to_rect(),
        )))
        .unwrap();
        path.fill = Some(crate::Fill::default());

        let mut clip = ClipPath::empty(cache.gen_clip_path_id());
        clip.root.children.push(Node::Path(Box::new(path)));

        // Clip path should not be affected by the image viewbox transform.
        // The final structure should look like:
        // <g clip-path="url(#clipPath1)">
        //     <g transform="matrix(1 0 0 1 10 20)">
        //         <image/>
        //     </g>
        // </g>

        let mut g2 = Group::empty();
        std::mem::swap(&mut g.id, &mut g2.id);
        g2.abs_transform = parent.abs_transform;
        g2.clip_path = Some(Arc::new(clip));
        g2.children.push(Node::Group(Box::new(g)));
        g2.calculate_bounding_boxes();

        parent.children.push(Node::Group(Box::new(g2)));
    } else {
        parent.children.push(Node::Group(Box::new(g)));
    }

    Some(())
}

pub(crate) fn get_href_data(href: &str, state: &converter::State) -> Option<ImageKind> {
    if let Ok(url) = data_url::DataUrl::process(href) {
        let (data, _) = url.decode_to_vec().ok()?;

        let mime = format!(
            "{}/{}",
            url.mime_type().type_.as_str(),
            url.mime_type().subtype.as_str()
        );

        (state.opt.image_href_resolver.resolve_data)(&mime, Arc::new(data), state.opt)
    } else {
        (state.opt.image_href_resolver.resolve_string)(href, state.opt)
    }
}

/// Checks that file has a PNG, a GIF, a JPEG or a WebP magic bytes.
/// Or an SVG(Z) extension.
fn get_image_file_format(path: &std::path::Path, data: &[u8]) -> Option<ImageFormat> {
    let ext = path.extension().and_then(|e| e.to_str())?.to_lowercase();
    if ext == "svg" || ext == "svgz" {
        return Some(ImageFormat::SVG);
    }

    get_image_data_format(data)
}

/// Checks that file has a PNG, a GIF, a JPEG or a WebP magic bytes.
fn get_image_data_format(data: &[u8]) -> Option<ImageFormat> {
    match imagesize::image_type(data).ok()? {
        imagesize::ImageType::Gif => Some(ImageFormat::GIF),
        imagesize::ImageType::Jpeg => Some(ImageFormat::JPEG),
        imagesize::ImageType::Png => Some(ImageFormat::PNG),
        imagesize::ImageType::Webp => Some(ImageFormat::WEBP),
        _ => None,
    }
}

/// Tries to load the `ImageData` content as an SVG image or emits a warning and returns `None`.
pub(crate) fn load_sub_svg(data: &[u8], opt: &Options) -> Option<ImageKind> {
    match Tree::from_data_nested(data, opt) {
        Ok(tree) => Some(ImageKind::SVG(tree)),
        Err(_) => {
            log::warn!("Failed to load nested SVG image.");
            None
        }
    }
}

/// Fits size into a viewbox.
fn fit_view_box(size: Size, rect: NonZeroRect, aspect: AspectRatio) -> Size {
    let s = rect.size();

    if aspect.align == svgtypes::Align::None {
        s
    } else if aspect.slice {
        size.expand_to(s)
    } else {
        size.scale_to(s)
    }
}
