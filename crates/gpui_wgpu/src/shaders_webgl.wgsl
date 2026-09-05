@group(1) @binding(0) var t_instances: texture_2d<u32>;

// Each texel of `t_instances` packs four 32-bit words of instance data. Records
// are read strictly front to back, so all readers share a cursor that keeps the
// most recently fetched texel and only touches the texture again when the next
// word crosses a texel boundary. This fetches each texel exactly once per
// record load instead of once per word. The `read_*` functions must therefore
// consume their words in struct declaration order; skipping or re-reading words
// would still return correct data (the cursor re-fetches whenever the texel
// index changes) but would forfeit the single-fetch-per-texel guarantee.
struct InstanceCursor {
    word_index: u32,
    texel_index: u32,
    texel: vec4<u32>,
    width: u32,
}

fn fetch_instance_texel(texel_index: u32, width: u32) -> vec4<u32> {
    let coordinate = vec2<i32>(
        i32(texel_index % width),
        i32(texel_index / width),
    );
    return textureLoad(t_instances, coordinate, 0);
}

fn instance_cursor(word_index: u32) -> InstanceCursor {
    let width = textureDimensions(t_instances).x;
    let texel_index = word_index / 4u;
    return InstanceCursor(
        word_index,
        texel_index,
        fetch_instance_texel(texel_index, width),
        width,
    );
}

fn read_word(cursor: ptr<function, InstanceCursor>) -> u32 {
    let word_index = (*cursor).word_index;
    let texel_index = word_index / 4u;
    if texel_index != (*cursor).texel_index {
        (*cursor).texel = fetch_instance_texel(texel_index, (*cursor).width);
        (*cursor).texel_index = texel_index;
    }
    (*cursor).word_index = word_index + 1u;
    return (*cursor).texel[word_index % 4u];
}

fn read_f32(cursor: ptr<function, InstanceCursor>) -> f32 {
    return bitcast<f32>(read_word(cursor));
}

fn read_i32(cursor: ptr<function, InstanceCursor>) -> i32 {
    return bitcast<i32>(read_word(cursor));
}

fn read_vec2_f32(cursor: ptr<function, InstanceCursor>) -> vec2<f32> {
    return vec2<f32>(read_f32(cursor), read_f32(cursor));
}

fn read_vec2_i32(cursor: ptr<function, InstanceCursor>) -> vec2<i32> {
    return vec2<i32>(read_i32(cursor), read_i32(cursor));
}

fn read_hsla(cursor: ptr<function, InstanceCursor>) -> Hsla {
    return Hsla(
        read_f32(cursor),
        read_f32(cursor),
        read_f32(cursor),
        read_f32(cursor),
    );
}

fn read_bounds(cursor: ptr<function, InstanceCursor>) -> Bounds {
    return Bounds(read_vec2_f32(cursor), read_vec2_f32(cursor));
}

fn read_corners(cursor: ptr<function, InstanceCursor>) -> Corners {
    return Corners(
        read_f32(cursor),
        read_f32(cursor),
        read_f32(cursor),
        read_f32(cursor),
    );
}

fn read_edges(cursor: ptr<function, InstanceCursor>) -> Edges {
    return Edges(
        read_f32(cursor),
        read_f32(cursor),
        read_f32(cursor),
        read_f32(cursor),
    );
}

fn read_color_stop(cursor: ptr<function, InstanceCursor>) -> LinearColorStop {
    return LinearColorStop(read_hsla(cursor), read_f32(cursor));
}

fn read_background(cursor: ptr<function, InstanceCursor>) -> Background {
    return Background(
        read_word(cursor),
        read_word(cursor),
        read_hsla(cursor),
        read_f32(cursor),
        array<LinearColorStop, 2>(
            read_color_stop(cursor),
            read_color_stop(cursor),
        ),
        read_word(cursor),
    );
}

fn read_atlas_tile(cursor: ptr<function, InstanceCursor>) -> AtlasTile {
    return AtlasTile(
        AtlasTextureId(
            read_word(cursor),
            read_word(cursor),
        ),
        read_word(cursor),
        read_word(cursor),
        AtlasBounds(
            read_vec2_i32(cursor),
            read_vec2_i32(cursor),
        ),
    );
}

fn read_transformation(cursor: ptr<function, InstanceCursor>) -> TransformationMatrix {
    return TransformationMatrix(
        mat2x2<f32>(
            read_vec2_f32(cursor),
            read_vec2_f32(cursor),
        ),
        read_vec2_f32(cursor),
    );
}

fn load_quad(instance_id: u32) -> Quad {
    var cursor = instance_cursor(instance_id * 44u);
    return Quad(
        read_word(&cursor),
        read_word(&cursor),
        read_bounds(&cursor),
        read_bounds(&cursor),
        read_background(&cursor),
        read_hsla(&cursor),
        read_corners(&cursor),
        read_edges(&cursor),
        read_corners(&cursor),
    );
}

fn load_shadow(instance_id: u32) -> Shadow {
    var cursor = instance_cursor(instance_id * 28u);
    return Shadow(
        read_word(&cursor),
        read_f32(&cursor),
        read_bounds(&cursor),
        read_corners(&cursor),
        read_bounds(&cursor),
        read_hsla(&cursor),
        read_bounds(&cursor),
        read_corners(&cursor),
        read_word(&cursor),
        read_word(&cursor),
    );
}

fn load_path_vertex(vertex_id: u32) -> PathRasterizationVertex {
    var cursor = instance_cursor(vertex_id * 26u);
    return PathRasterizationVertex(
        read_vec2_f32(&cursor),
        read_vec2_f32(&cursor),
        read_background(&cursor),
        read_bounds(&cursor),
    );
}

fn load_path_sprite(instance_id: u32) -> PathSprite {
    var cursor = instance_cursor(instance_id * 4u);
    return PathSprite(read_bounds(&cursor));
}

fn load_underline(instance_id: u32) -> Underline {
    var cursor = instance_cursor(instance_id * 16u);
    return Underline(
        read_word(&cursor),
        read_word(&cursor),
        read_bounds(&cursor),
        read_bounds(&cursor),
        read_hsla(&cursor),
        read_f32(&cursor),
        read_word(&cursor),
    );
}

fn load_mono_sprite(instance_id: u32) -> MonochromeSprite {
    var cursor = instance_cursor(instance_id * 28u);
    return MonochromeSprite(
        read_word(&cursor),
        read_word(&cursor),
        read_bounds(&cursor),
        read_bounds(&cursor),
        read_hsla(&cursor),
        read_atlas_tile(&cursor),
        read_transformation(&cursor),
    );
}

fn load_poly_sprite(instance_id: u32) -> PolychromeSprite {
    var cursor = instance_cursor(instance_id * 24u);
    return PolychromeSprite(
        read_word(&cursor),
        read_word(&cursor),
        read_word(&cursor),
        read_f32(&cursor),
        read_bounds(&cursor),
        read_bounds(&cursor),
        read_corners(&cursor),
        read_atlas_tile(&cursor),
    );
}
