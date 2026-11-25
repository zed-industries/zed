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

@group(1) @binding(0) var<storage, read> b_path_tiles: array<PathTile>;
@group(1) @binding(1) var<storage, read> b_path_curves: array<PathCurve>;
@group(1) @binding(2) var<storage, read> b_path_paints: array<PathPaint>;

fn load_path_tile(instance_id: u32) -> PathTile {
    return b_path_tiles[instance_id];
}

fn load_path_curve(index: u32) -> PathCurve {
    return b_path_curves[index];
}

fn load_path_paint(index: u32) -> PathPaint {
    return b_path_paints[index];
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
