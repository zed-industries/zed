
extern crate image as png;

extern crate exr;


/// Read an rgba image, or fail if none can be found.
/// Then crop away transparent pixels,
/// and write the cropped result to another file.
/// This retains only the rgb pixels, and no other layers.
pub fn main() {
    use exr::prelude::*;
    use exr::image::pixel_vec::*; // import predefined pixel storage

    let path = "tests/images/valid/custom/oh crop.exr";

    type DynamicRgbaPixel = (Sample, Sample, Sample, Sample); // `Sample` is an enum containing the original data type (f16,f32, or u32)

    // load an rgba image
    // this specific example discards all but the first valid rgb layers and converts all pixels to f32 values
    // TODO optional alpha channel!
    let image: PixelImage<PixelVec<DynamicRgbaPixel>, RgbaChannels> = read_first_rgba_layer_from_file(
        path,
        PixelVec::<DynamicRgbaPixel>::constructor,

        // use this predefined rgba pixel container from the exr crate, requesting any type of pixels with 3 or 4 values
        PixelVec::set_pixel
    ).expect("this file exists in the exrs repository. download that?");

    // construct a ~simple~ cropped image
    let image: Image<Layer<CroppedChannels<SpecificChannels<PixelVec<DynamicRgbaPixel>, RgbaChannels>>>> = Image {
        attributes: image.attributes,

        // crop each layer
        layer_data: {
            println!("cropping layer {:#?}", image.layer_data);

            // if has alpha, crop it where alpha is zero
            image.layer_data
                .crop_where(|(_r, _g, _b, alpha)| alpha.is_zero())
                .or_crop_to_1x1_if_empty() // do not remove empty layers from image, because it could result in an image without content
        },
    };

    image.write().to_file("cropped_rgba.exr").unwrap();
    println!("cropped file to cropped_rgba.exr");
}

