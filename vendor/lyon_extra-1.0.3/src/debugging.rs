use path::math::Point;
use path::PathEvent;
use path::{Path, PathSlice};

pub type Polygons = Vec<Vec<Point>>;
pub type PolygonsRef<'a> = &'a [Vec<Point>];

pub fn path_to_polygons(path: PathSlice) -> Polygons {
    let mut polygons = Vec::new();
    let mut poly = Vec::new();
    for evt in path {
        match evt {
            PathEvent::Begin { at } => {
                if !poly.is_empty() {
                    polygons.push(poly);
                }
                poly = vec![at];
            }
            PathEvent::Line { to, .. } => {
                poly.push(to);
            }
            PathEvent::End { .. } => {
                if !poly.is_empty() {
                    polygons.push(poly);
                }
                poly = Vec::new();
            }
            _ => {
                println!(
                    " -- path_to_polygons: warning! Unsupported event type {evt:?}"
                );
            }
        }
    }
    polygons
}

pub fn polygons_to_path(polygons: PolygonsRef) -> Path {
    let mut builder = Path::builder().flattened(0.05);
    for poly in polygons.iter() {
        let mut poly_iter = poly.iter();
        builder.begin(*poly_iter.next().unwrap());
        for v in poly_iter {
            builder.line_to(*v);
        }
        builder.close();
    }
    builder.build()
}

pub fn find_reduced_test_case<F: Fn(Path) -> bool + panic::UnwindSafe + panic::RefUnwindSafe>(
    path: PathSlice,
    cb: &F,
) -> Path {
    let mut polygons = path_to_polygons(path);

    println!(" -- removing sub-paths...");

    polygons = find_reduced_test_case_sp(polygons, cb);

    println!(" -- removing vertices...");

    for p in 0..polygons.len() {
        let mut v = 0;
        loop {
            if v >= polygons[p].len() || polygons[p].len() <= 3 {
                break;
            }

            let mut cloned = polygons.clone();
            cloned[p].remove(v);

            let path = polygons_to_path(&cloned);

            let failed = panic::catch_unwind(|| cb(path)).unwrap_or(true);

            if failed {
                polygons = cloned;
                continue;
            }

            v += 1;
        }
    }

    let path = polygons_to_path(&polygons);
    println!(" ----------- reduced test case: -----------\n\n");
    println!("#[test]");
    println!("fn reduced_test_case() {{");
    println!("    let mut builder = Path::builder();\n");
    for poly in &polygons {
        let mut poly_iter = poly.iter();
        let pos = *poly_iter.next().unwrap();
        println!("    builder.begin(point({}, {}));", pos.x, pos.y);
        for pos in poly_iter {
            println!("    builder.line_to(point({}, {}));", pos.x, pos.y);
        }
        println!("    builder.close();\n");
    }
    println!("    test_path(builder.build().as_slice());\n");
    println!("    // SVG path syntax:");
    println!("    // \"{path:?}\"");
    println!("}}\n\n");

    path
}

use std::panic;

fn find_reduced_test_case_sp<F>(mut polygons: Polygons, cb: &F) -> Polygons
where
    F: Fn(Path) -> bool + panic::UnwindSafe + panic::RefUnwindSafe,
{
    let mut i = 0;
    loop {
        if i >= polygons.len() {
            return polygons;
        }

        let mut cloned = polygons.clone();
        cloned.remove(i);
        let path = polygons_to_path(&cloned);

        let failed = panic::catch_unwind(|| cb(path)).unwrap_or(true);

        if failed {
            polygons = cloned;
            continue;
        }

        i += 1;
    }
}
