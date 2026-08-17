// Copyright 2017 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !(args.len() == 3 || args.len() == 5) {
        println!(
            "Usage:\n\
             \tdraw_bboxes <in-svg> <out-png>\n\
             \tdraw_bboxes <in-svg> <out-png> -z ZOOM"
        );
        return;
    }

    let zoom = if args.len() == 5 {
        args[4].parse::<f32>().expect("not a float")
    } else {
        1.0
    };

    let mut opt = usvg::Options {
        // Get file's absolute directory.
        resources_dir: std::fs::canonicalize(&args[1])
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf())),
        ..usvg::Options::default()
    };

    opt.fontdb_mut().load_system_fonts();

    let svg_data = std::fs::read(&args[1]).unwrap();
    let tree = usvg::Tree::from_data(&svg_data, &opt).unwrap();

    let mut bboxes = Vec::new();
    let mut stroke_bboxes = Vec::new();
    collect_bboxes(tree.root(), &mut bboxes, &mut stroke_bboxes);

    let pixmap_size = tree.size().to_int_size().scale_by(zoom).unwrap();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    let render_ts = tiny_skia::Transform::from_scale(zoom, zoom);
    resvg::render(&tree, render_ts, &mut pixmap.as_mut());

    let stroke = tiny_skia::Stroke {
        width: 1.0 / zoom, // prevent stroke scaling as well
        ..tiny_skia::Stroke::default()
    };

    let mut paint1 = tiny_skia::Paint::default();
    paint1.set_color_rgba8(255, 0, 0, 127);

    let mut paint2 = tiny_skia::Paint::default();
    paint2.set_color_rgba8(0, 200, 0, 127);

    for bbox in bboxes {
        let path = tiny_skia::PathBuilder::from_rect(bbox);
        pixmap.stroke_path(&path, &paint1, &stroke, render_ts, None);
    }

    for bbox in stroke_bboxes {
        let path = tiny_skia::PathBuilder::from_rect(bbox);
        pixmap.stroke_path(&path, &paint2, &stroke, render_ts, None);
    }

    pixmap.save_png(&args[2]).unwrap();
}

fn collect_bboxes(
    parent: &usvg::Group,
    bboxes: &mut Vec<usvg::Rect>,
    stroke_bboxes: &mut Vec<usvg::Rect>,
) {
    for node in parent.children() {
        if let usvg::Node::Group(group) = node {
            collect_bboxes(group, bboxes, stroke_bboxes);
        }

        let bbox = node.abs_bounding_box();
        bboxes.push(bbox);

        let stroke_bbox = node.abs_stroke_bounding_box();
        if bbox != stroke_bbox {
            stroke_bboxes.push(stroke_bbox);
        }
    }
}
