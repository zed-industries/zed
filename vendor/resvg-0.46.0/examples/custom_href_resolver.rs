// Copyright 2022 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

fn main() {
    let mut opt = usvg::Options::default();

    let ferris_image = std::sync::Arc::new(std::fs::read("./examples/ferris.png").unwrap());

    // We know that our SVG won't have DataUrl hrefs, just return None for such case.
    let resolve_data = Box::new(|_: &str, _: std::sync::Arc<Vec<u8>>, _: &usvg::Options| None);

    // Here we handle xlink:href attribute as string,
    // let's use already loaded Ferris image to match that string.
    let resolve_string = Box::new(move |href: &str, _: &usvg::Options| match href {
        "ferris_image" => Some(usvg::ImageKind::PNG(ferris_image.clone())),
        _ => None,
    });

    // Assign new ImageHrefResolver option using our closures.
    opt.image_href_resolver = usvg::ImageHrefResolver {
        resolve_data,
        resolve_string,
    };

    let svg_data = std::fs::read("./examples/custom_href_resolver.svg").unwrap();
    let tree = usvg::Tree::from_data(&svg_data, &opt).unwrap();

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    pixmap.save_png("custom_href_resolver.png").unwrap();
}
