// Storage-buffer instance transport. Concatenated after `shaders.wgsl` on backends
// with storage buffer support; `shaders_webgl.wgsl` is the WebGL2 counterpart that
// decodes the same records from a texture instead.
//
// All buffers share `@group(1) @binding(0)` because each pipeline binds only the
// buffer its own entry points read.

@group(1) @binding(0) var<storage, read> b_quads: array<Quad>;
@group(1) @binding(0) var<storage, read> b_shadows: array<Shadow>;
@group(1) @binding(0) var<storage, read> b_underlines: array<Underline>;
@group(1) @binding(0) var<storage, read> b_mono_sprites: array<MonochromeSprite>;
@group(1) @binding(0) var<storage, read> b_poly_sprites: array<PolychromeSprite>;

fn load_quad(instance_id: u32) -> Quad {
    return b_quads[instance_id];
}

fn load_shadow(instance_id: u32) -> Shadow {
    return b_shadows[instance_id];
}

// Path records are only 4-byte aligned on the Rust side, and `vec2` fields would add padding, so
// their storage layouts use scalar fields and are converted on load.
struct PathTileRecord {
    order: u32,
    paint: u32,
    curve_start: u32,
    curve_count: u32,
    top_left: array<f32, 2>,
    bottom_right: array<f32, 2>,
    reference: array<f32, 2>,
    backdrop: i32,
}

struct PathCurveRecord {
    p0: array<f32, 2>,
    p1: array<f32, 2>,
    a: array<f32, 2>,
    b: array<f32, 2>,
    sign: f32,
}

struct PathPaintRecord {
    bounds: array<f32, 4>,
    content_mask: array<f32, 4>,
    color: Background,
    even_odd: u32,
}

@group(1) @binding(0) var<storage, read> b_path_tiles: array<PathTileRecord>;
@group(1) @binding(1) var<storage, read> b_path_curves: array<PathCurveRecord>;
@group(1) @binding(2) var<storage, read> b_path_paints: array<PathPaintRecord>;

fn record_vec2(values: array<f32, 2>) -> vec2<f32> {
    return vec2<f32>(values[0], values[1]);
}

fn record_bounds(values: array<f32, 4>) -> Bounds {
    return Bounds(vec2<f32>(values[0], values[1]), vec2<f32>(values[2], values[3]));
}

fn load_path_tile(instance_id: u32) -> PathTile {
    let record = b_path_tiles[instance_id];
    return PathTile(
        record.order,
        record.paint,
        record.curve_start,
        record.curve_count,
        record_vec2(record.top_left),
        record_vec2(record.bottom_right),
        record_vec2(record.reference),
        record.backdrop,
    );
}

fn load_path_curve(index: u32) -> PathCurve {
    let record = b_path_curves[index];
    return PathCurve(
        record_vec2(record.p0),
        record_vec2(record.p1),
        record_vec2(record.a),
        record_vec2(record.b),
        record.sign,
    );
}

fn load_path_paint(index: u32) -> PathPaint {
    let record = b_path_paints[index];
    return PathPaint(
        record_bounds(record.bounds),
        record_bounds(record.content_mask),
        record.color,
        record.even_odd,
    );
}

fn load_underline(instance_id: u32) -> Underline {
    return b_underlines[instance_id];
}

fn load_mono_sprite(instance_id: u32) -> MonochromeSprite {
    return b_mono_sprites[instance_id];
}

fn load_poly_sprite(instance_id: u32) -> PolychromeSprite {
    return b_poly_sprites[instance_id];
}
