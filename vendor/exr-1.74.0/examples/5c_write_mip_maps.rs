
extern crate smallvec;
extern crate rand;
extern crate half;


// exr imports
extern crate exr;



/// Writes two layers, each with multiple mip maps.
/// All mip maps have solid color for brevity.
fn main() {
    use exr::prelude::*;
    use exr::math::RoundingMode;
    use smallvec::smallvec;

    let full_size = Vec2(512, 512);
    let size_rounding = RoundingMode::Up;

    let mip_levels_sizes = exr::meta::mip_map_levels(
        size_rounding, full_size
    ).collect::<Vec<_>>();

    let red_mip_levels = mip_levels_sizes.iter()
        .map(|(_index, level_size)|{
            FlatSamples::F32(vec![0.1_f32; level_size.area() ])
        })
        .collect();

    let green_mip_levels = mip_levels_sizes.iter()
        .map(|(_index, level_size)|{
            FlatSamples::F32(vec![0.6_f32; level_size.area() ])
        })
        .collect();

    let blue_mip_levels = mip_levels_sizes.iter()
        .map(|(_index, level_size)|{
            FlatSamples::F32(vec![1.0_f32; level_size.area() ])
        })
        .collect();

    let rgb_mip_maps = AnyChannels::sort(smallvec![
        AnyChannel::new("R", Levels::Mip { level_data: red_mip_levels, rounding_mode: size_rounding }),
        AnyChannel::new("G", Levels::Mip { level_data: green_mip_levels, rounding_mode: size_rounding }),
        AnyChannel::new("B", Levels::Mip { level_data: blue_mip_levels, rounding_mode: size_rounding }),
    ]);

    let layer1 = Layer::new(
        full_size,
        LayerAttributes::named("teal rgb"),
        Encoding::FAST_LOSSLESS,
        rgb_mip_maps
    );

    let mut layer2 = layer1.clone();
    layer2.attributes.layer_name = Some("Copied Layer".into());
    layer2.encoding = Encoding::SMALL_FAST_LOSSLESS;

    // define the visible area of the canvas
    let image_attributes = ImageAttributes::new(
        IntegerBounds::from_dimensions(full_size)
    );

    let image = Image::empty(image_attributes)
        .with_layer(layer1).with_layer(layer2);

    println!("writing image...");
    image.write().to_file("mip_maps.exr").unwrap();

    println!("created file mip_maps.exr");
}