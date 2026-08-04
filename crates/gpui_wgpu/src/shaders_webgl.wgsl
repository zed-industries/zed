@group(1) @binding(0) var t_instances: texture_2d<u32>;

fn load_instance_word(word_index: u32) -> u32 {
    let dimensions = textureDimensions(t_instances);
    let texel_index = word_index / 4u;
    let coordinate = vec2<i32>(
        i32(texel_index % dimensions.x),
        i32(texel_index / dimensions.x),
    );
    return textureLoad(t_instances, coordinate, 0)[word_index % 4u];
}

fn load_f32(word_index: u32) -> f32 {
    return bitcast<f32>(load_instance_word(word_index));
}

fn load_vec2_f32(word_index: u32) -> vec2<f32> {
    return vec2<f32>(load_f32(word_index), load_f32(word_index + 1u));
}

fn load_hsla(word_index: u32) -> Hsla {
    return Hsla(
        load_f32(word_index),
        load_f32(word_index + 1u),
        load_f32(word_index + 2u),
        load_f32(word_index + 3u),
    );
}

fn load_bounds(word_index: u32) -> Bounds {
    return Bounds(load_vec2_f32(word_index), load_vec2_f32(word_index + 2u));
}

fn load_corners(word_index: u32) -> Corners {
    return Corners(
        load_f32(word_index),
        load_f32(word_index + 1u),
        load_f32(word_index + 2u),
        load_f32(word_index + 3u),
    );
}

fn load_edges(word_index: u32) -> Edges {
    return Edges(
        load_f32(word_index),
        load_f32(word_index + 1u),
        load_f32(word_index + 2u),
        load_f32(word_index + 3u),
    );
}

fn load_color_stop(word_index: u32) -> LinearColorStop {
    return LinearColorStop(load_hsla(word_index), load_f32(word_index + 4u));
}

fn load_background(word_index: u32) -> Background {
    return Background(
        load_instance_word(word_index),
        load_instance_word(word_index + 1u),
        load_hsla(word_index + 2u),
        load_f32(word_index + 6u),
        array<LinearColorStop, 2>(
            load_color_stop(word_index + 7u),
            load_color_stop(word_index + 12u),
        ),
        load_instance_word(word_index + 17u),
    );
}

fn load_atlas_tile(word_index: u32) -> AtlasTile {
    return AtlasTile(
        AtlasTextureId(
            load_instance_word(word_index),
            load_instance_word(word_index + 1u),
        ),
        load_instance_word(word_index + 2u),
        load_instance_word(word_index + 3u),
        AtlasBounds(
            vec2<i32>(
                bitcast<i32>(load_instance_word(word_index + 4u)),
                bitcast<i32>(load_instance_word(word_index + 5u)),
            ),
            vec2<i32>(
                bitcast<i32>(load_instance_word(word_index + 6u)),
                bitcast<i32>(load_instance_word(word_index + 7u)),
            ),
        ),
    );
}

fn load_transformation(word_index: u32) -> TransformationMatrix {
    return TransformationMatrix(
        mat2x2<f32>(
            load_vec2_f32(word_index),
            load_vec2_f32(word_index + 2u),
        ),
        load_vec2_f32(word_index + 4u),
    );
}

fn load_quad(instance_id: u32) -> Quad {
    let word_index = instance_id * 40u;
    return Quad(
        load_instance_word(word_index),
        load_instance_word(word_index + 1u),
        load_bounds(word_index + 2u),
        load_bounds(word_index + 6u),
        load_background(word_index + 10u),
        load_hsla(word_index + 28u),
        load_corners(word_index + 32u),
        load_edges(word_index + 36u),
    );
}

fn load_shadow(instance_id: u32) -> Shadow {
    let word_index = instance_id * 28u;
    return Shadow(
        load_instance_word(word_index),
        load_f32(word_index + 1u),
        load_bounds(word_index + 2u),
        load_corners(word_index + 6u),
        load_bounds(word_index + 10u),
        load_hsla(word_index + 14u),
        load_bounds(word_index + 18u),
        load_corners(word_index + 22u),
        load_instance_word(word_index + 26u),
        load_instance_word(word_index + 27u),
    );
}

fn load_path_vertex(vertex_id: u32) -> PathRasterizationVertex {
    let word_index = vertex_id * 26u;
    return PathRasterizationVertex(
        load_vec2_f32(word_index),
        load_vec2_f32(word_index + 2u),
        load_background(word_index + 4u),
        load_bounds(word_index + 22u),
    );
}

fn load_path_sprite(instance_id: u32) -> PathSprite {
    return PathSprite(load_bounds(instance_id * 4u));
}

fn load_underline(instance_id: u32) -> Underline {
    let word_index = instance_id * 16u;
    return Underline(
        load_instance_word(word_index),
        load_instance_word(word_index + 1u),
        load_bounds(word_index + 2u),
        load_bounds(word_index + 6u),
        load_hsla(word_index + 10u),
        load_f32(word_index + 14u),
        load_instance_word(word_index + 15u),
    );
}

fn load_mono_sprite(instance_id: u32) -> MonochromeSprite {
    let word_index = instance_id * 28u;
    return MonochromeSprite(
        load_instance_word(word_index),
        load_instance_word(word_index + 1u),
        load_bounds(word_index + 2u),
        load_bounds(word_index + 6u),
        load_hsla(word_index + 10u),
        load_atlas_tile(word_index + 14u),
        load_transformation(word_index + 22u),
    );
}

fn load_poly_sprite(instance_id: u32) -> PolychromeSprite {
    let word_index = instance_id * 24u;
    return PolychromeSprite(
        load_instance_word(word_index),
        load_instance_word(word_index + 1u),
        load_instance_word(word_index + 2u),
        load_f32(word_index + 3u),
        load_bounds(word_index + 4u),
        load_bounds(word_index + 8u),
        load_corners(word_index + 12u),
        load_atlas_tile(word_index + 16u),
    );
}
