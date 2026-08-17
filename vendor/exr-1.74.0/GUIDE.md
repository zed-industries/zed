# Guide

This document talks about the capabilities of OpenEXR and outlines the design of this library. 
In addition to reading this guide, you should also have a look at the examples.

Contents:
- Wording
- Why this is complicated
- One-liners for reading and writing simple images
- Reading a complex image
- The Image data structure
- Writing a complex image

## Wording
Some names in this library differ from the classic OpenEXR conventions.
For example, an OpenEXR "multipart" is called a file with multiple "layers" in this library.
The old OpenEXR "layers" are called "grouped channels" instead.

- `Image` Contains everything that an `.exr` file can contain. Includes metadata and multiple layers.
- `Layer` A grid of pixels that can be placed anywhere on the two-dimensional canvas
- `Channel` All samples of a single color component, such as red or blue. Also contains metadata.
- `Pixel` The color at an exact location in the image. Contains one sample for each channel.
- `Sample` The value (either f16, f32 or u32) of one channel at an exact location in the image.
            Usually a simple number, such as the red value of the bottom left pixel.
- `Grouped Channels` Multiple channels may be grouped my prepending the same prefix to the name.
                    This behaviour is opt-in; it has to be enabled explicitly:
                    By default, channels are stored in a plain list, and channel names are unmodified.
- `pedantic: bool` When reading, pedantic being false will generally ignore 
    invalid information instead of aborting the reading process where possible. 
    When writing, pedantic being false will generally skip some expensive image validation checks.

## OpenEXR | Complexity
This image format supports some features that you won't find in other image formats.
As a consequence, an exr file cannot necessarily be converted to other formats, 
even when the loss of precision is acceptable. Furthermore, 
an arbitrary exr image may include possibly unwanted data. 
Supporting deep data, for example, might be unnecessary for some applications.

To read an image, `exrs` must know which parts of an image you want to end up with, 
and which parts of the file should be skipped. That's why you need
a little more code to read an exr file, compared to simpler file formats.

### Possibly Undesired Features
- Arbitrary Channels: 
  `CMYK`, `YCbCr`, `LAB`, `XYZ` channels might not be interesting for you, 
  maybe you only want to accept `RGBA` images
- Deep Data: Multiple colors per pixel might not be interesting for you 
- Resolution Levels: Mip Maps or Rip Maps might be unnecessary and can be skipped,
  loading only the full resolution image instead
<!-- - TODO: Meta Data: Skip reading meta data -->

# Simple Reading and Writing
There are a few very simple functions for the most common use cases.
For decoding an image file, use one of these functions 
from the `exr::image::read` module (data structure complexity increasing):

1. `read_first_rgba_layer_from_file(path, your_constructor, your_pixel_setter)`
1. `read_all_rgba_layers_from_file(path, your_constructor, your_pixel_setter)`
1. `read_first_flat_layer_from_file(path)`
1. `read_all_flat_layers_from_file(path)`
1. `read_all_data_from_file(path)`

If you don't have a file path, or want to load any other channels than `rgba`, 
then these simple functions will not suffice. The more complex approaches are
described later in this document.

For encoding an image file, use one of these functions in the `exr::image::write` module:

1. `write_rgba_file(path, width, height, |x,y| my_image.get_rgb_at(x,y))`
1. `write_rgb_file(path, width, height, |x,y| my_image.get_rgba_at(x,y))`

These functions are only syntactic sugar. If you want to customize the data type,
the compression method, or write multiple layers, these simple functions will not suffice.
Again, the more complex approaches are described in the following paragraph.

# Reading an Image

Reading an image involves three steps:
1. Specify how to load an image by constructing an image reader.
    1. Start with `read()`
    1. Chain method calls to customize the reader
1. Call `from_file(path)`, `from_buffered(bytes)`, or `from_unbuffered(bytes)` 
   on the reader to actually load an image
1. Process the resulting image data structure or the error in your application

The type of the resulting image depends on the reader you constructed. For example,
if you configure the reader to load mip map levels, the resulting image type
will contain an additional vector with the mip map levels.

### Deep Data
The first choice to be made is whether you want to load deep data or not.
Deep data is where multiple colors are stored in one pixel at the same location.
Currently, deep data is not supported yet, so we always call `no_deep_data()`.

```rust
fn main(){
    use exr::prelude::*;
    let reader = read().no_deep_data();
}
```

### Resolution Levels
Decide whether you want to load the largest resolution level, or all Mip Maps from the file.
Loading only the largest level actually skips portions of the image, which should be faster.

Calling `largest_resolution_level()` will result in a single image (`FlatSamples`),
whereas calling `all_resolution_levels()` will result in multiple levels `Levels<FlatSamples>`.

```rust
fn main(){
    use exr::prelude::*;
    let reader = read().no_deep_data().largest_resolution_level();
    let reader = read().no_deep_data().all_resolution_levels();
}
```

### Channels
Decide whether you want to load all channels in a dynamic list, or only load a fixed set of channels.

Calling `all_channels()` will result in a `Vec<Channel<_>>`. 

```rust
fn main(){
    use exr::prelude::*;
    let reader = read().no_deep_data().largest_resolution_level().all_channels();
}
```

The alternative, `specific_channels()` allows you to exactly specify which channels should be loaded.
The usage follows the same builder pattern as the rest of the library.

First, call `specific_channels()`. Then, for each channel you desire,
call either `required(channel_name)` or `optional(channel_name, default_value)`.
At last, call `collect_pixels()` to define how the pixels should be stored in an image.
This additional mechanism will not simply store the pixels in a `Vec<Pixel>`, but instead
works with a closure. This allows you to instantiate your own existing image type with
the pixel data from the file. 

```rust
fn main(){
    use exr::prelude::*;
    
    let reader = read()
        .no_deep_data().largest_resolution_level()
        
        // load LAB channels, with chroma being optional
        .specific_channels().required("L").optional("A", 0.0).optional("B", 0.0).collect_pixels(
        
            // create our image based on the resolution of the file
            |resolution: Vec2<usize>, (l,a,b): &(ChannelDescription, Option<ChannelDescription>, Option<ChannelDescription>)|{
                if a.is_some() && b.is_some() { MyImage::new_lab(resolution) }
                else { MyImage::new_luma(resolution) }
            },
        
            // insert a single pixel into out image
            |my_image: &mut MyImage, position: Vec<usize>, (l,a,b): (f32, f16, f16)|{
                my_image.set_pixel_at(position.x(), position.y(), (l, a, b));
            }
        
        );
}
```

The first closure is the constructor of your image, and the second closure is the setter for a single pixel in your image.
The tuple containing the channel descriptions and the pixel tuple depend on the channels that you defined earlier.
In this example, as we defined to load L,A and B, each pixel has three values. The arguments of the closure
can usually be inferred, so you don't need to declare the type of your image and the `Vec2<usize>`.
However, the type of the pixel needs to be defined. In this example, we define the pixel type to be `(f32, f16, f16)`.
All luma values will be converted to `f32` and all chroma values will be converted to `f16`.
The pixel type can be any combination of `f16`, `f32`, `u32` or `Sample` values, in a tuple with as many entries as there are channels.
The `Sample` type is a dynamic enum over the other types, which allows you to keep the original sample type of each image.

_Note: Currently, up to 32 channels are supported, which is an implementation problem. 
Open an issue if this is not enough for your use case. Alternatively, 
you can always use `all_channels()`, which has no limitations._

####RGBA Channels
For rgba images, there is a predefined simpler alternative to `specific_channels` called `rgb_channels` and `rgba_channels`.
It works just the same as `specific_channels` and , but you don't need to specify the names of the channels explicitly.

```rust
fn main(){
    use exr::prelude::*;
    
    let reader = read()
        .no_deep_data().largest_resolution_level()
        
        // load rgba channels
        // with alpha being optional, defaulting to 1.0
        .rgba_channels(
        
            // create our image based on the resolution of the file
            |resolution, &(r,g,b,a)|{
                if a.is_some() { MyImage::new_with_alpha(resolution.x(), resolution.y()) }
                else { MyImage::new_without_alpha(resolution.x(), resolution.y()) }
            },
        
            // insert a single pixel into out image
            |my_image, position, (r,g,b,a): (f32, f32, f32, f16)|{
                my_image.set_pixel_at(position.x(), position.y(), (r,g,b,a));
            }
        
        );
}
```


### Layers
Use `all_layers()` to load a `Vec<Layer<_>>` or use `first_valid_layer()` to only load 
the first `Layer<_>` that matches the previously defined requirements 
(for example, the first layer without deep data and cmyk channels).


```rust
fn main() {
    use exr::prelude::*;

    let image = read()
        .no_deep_data().largest_resolution_level()
        .all_channels().all_layers();

    let image = read()
        .no_deep_data().largest_resolution_level()
        .all_channels().first_valid_layer();
}
```

### Attributes
Currently, the only option is to load all attributes by calling `all_attributes()`.

### Progress Notification
This library allows you to listen for the file reading progress by calling `on_progress(callback)`.
If you don't need this, you can just omit this call.

```rust
fn main() {
    use exr::prelude::*;

    let image = read().no_deep_data().largest_resolution_level()
        .all_channels().first_valid_layer().all_attributes()
        .on_progress(|progress: f64| println!("progress: {:.3}", progress));
}
```

### Parallel Decompression
By default, this library uses all the available CPU cores if the pixels are compressed.
You can disable this behaviour by additionally calling `non_parallel()`.

```rust
fn main() {
use exr::prelude::*;

    let image = read().no_deep_data().largest_resolution_level()
        .all_channels().first_valid_layer().all_attributes()
        .non_parallel();
}
```

### Byte Sources
Any `std::io::Read` byte source can be used as input. However, this library also offers a simplification for files.
Call `from_file(path)` to load an image from a file. Internally, this wraps the file in a buffered reader.
Alternatively, you can call `from_buffered` or `from_unbuffered` (which wraps your reader in a buffered reader) to read an image.

```rust
fn main() {
use exr::prelude::*;

    let read = read().no_deep_data().largest_resolution_level()
        .all_channels().first_valid_layer().all_attributes();
    
    let image = read.clone().from_file("D:/images/file.exr"); // also accepts `Path` and `PathBuf` and `String`
    let image = read.clone().from_unbuffered(web_socket);
    let image = read.clone().from_buffered(Cursor::new(byte_vec));
}
```

### Results and Errors
The type of image returned depends on the options you picked.
The image is wrapped in a `Result<..., exr::error::Error>`.
This error type allows you to differentiate between three types of errors:
- `Error::Io(std::io::Error)` for file system errors (for example, "file does not exist" or "missing access rights")
- `Error::NotSupported(str)` for files that may be valid but contain features that are not supported yet
- `Error::Invalid(str)` for files that do not contain a valid exr image (files that are not exr or damaged exr)

## Full Example
Loading all channels from the file:
```rust
fn main() {
    use exr::prelude::*;

    // the type of the this image depends on the chosen options
    let image = read()
        .no_deep_data() // (currently required)
        .largest_resolution_level() // or `all_resolution_levels()`
        .all_channels() // or `rgba_channels` or `specific_channels() ...`
        .all_layers() // or `first_valid_layer()`
        .all_attributes() // (currently required)
        .on_progress(|progress| println!("progress: {:.1}", progress * 100.0)) // optional
        //.non_parallel() // optional. discouraged. just leave this line out
        .from_file("image.exr").unwrap(); // or `from_buffered(my_byte_slice)`
}
```


# The `Image` Data Structure

For great flexibility, this crate does not offer a plain data structure to represent an exr image.
Instead, the `Image` data type has a generic parameter, allowing for different image contents.

```rust
fn main(){
    // this image contains only a single layer
    let single_layer_image: Image<Layer<_>> = Image::from_layer(my_layer);

    // this image contains an arbitrary number of layers (notice the S for plural on `Layers`)
    let multi_layer_image: Image<Layers<_>> = Image::new(attributes, smallvec![ layer1, layer2 ]);

    // this image can contain the compile-time specified channels
    let single_layer_rgb_image : Image<Layer<SpecificChannels<_, _>>> = Image::from_layer(Layer::new(
        dimensions, attributes, encoding,
        RgbaChannels::new(sample_types, rgba_pixels)
    ));
    
    // this image can contain all channels from a file, even unexpected ones
    let single_layer_image : Image<Layer<AnyChannels<_>>> = Image::from_layer(Layer::new(
        dimensions, attributes, encoding,
        AnyChannels::sort(smallvec![ channel_x, channel_y, channel_z ])
    ));
    
}
```

The following pseudo code illustrates the image data structure.
The image should always be constructed using the constructor functions such as `Image::new(...)`,
because these functions watch out for invalid image contents.

```
Image {
    attributes: ImageAttributes,
    
    // the layer data can be either a single layer a list of layers
    layer_data: Layer | SmallVec<Layer> | Vec<Layer> | &[Layer] (writing only),
}

Layer {
    
    // the channel data can either be a fixed set of known channels, or a dynamic list of arbitrary channels
    channel_data: SpecificChannels | AnyChannels,
    
    attributes: LayerAttributes,
    size: Vec2<usize>,
    encoding: Encoding,
}

SpecificChannels {
    channels: [any tuple containing `ChannelDescription` or `Option<ChannelDescription>`],
    
    // the storage is usually a closure or a custom type which implements the `GetPixel` trait
    storage: impl GetPixel | impl Fn(Vec2<usize>) -> Pixel,    
        where Pixel = any tuple containing f16 or f32 or u32 values
}

AnyChannels {
    list: SmallVec<AnyChannel>
}

AnyChannel {
    name: Text,
    sample_data: FlatSamples | Levels,
    quantize_linearly: bool,
    sampling: Vec2<usize>,
}

Levels = Singular(FlatSamples) | Mip(FlatSamples) | Rip(FlatSamples)
FlatSamples = F16(Vec<f16>) | F32(Vec<f32>) | U32(Vec<u32>)
```

As a consequence, one of the simpler image types is `Image<Layer<AnyChannels<FlatSamples>>>`. If you
enable loading multiple resolution levels, you will instead get the type `Image<Layer<AnyChannels<Levels<FlatSamples>>>>`.

While you can put anything inside an image,
it can only be written if the content of the image implements certain traits.
This allows you to potentially write your own channel storage system.


# Writing an Image

Writing an image involves three steps:
1. Construct the image data structure, starting with an `exrs::image::Image`
1. Call `image_data.write()` to obtain an image writer
1. Customize the writer, for example in order to listen for the progress
1. Write the image by calling `to_file(path)`, `to_buffered(bytes)`, or `to_unbuffered(bytes)` on the reader


### Image
You will currently need an `Image<_>` at the top level. The type parameter is the type of layer.  

The following variants are recommended:  
- `Image::from_channels(resolution, channels)` where the pixel data must be `SpecificChannels` or `AnyChannels`.
- `Image::from_layer(layer)` where the layer data must be one `Layer`.
- `Image::empty(attributes).with_layer(layer1).with_layer(layer2)...` where the two layers can have different types
- `Image::new(image_attributes, layer_data)` where the layer data can be `Layers` or `Layer`.
- `Image::from_layers(image_attributes, layer_vec)` where the layer data can be `Layers`.

```rust
fn main() {
    use exr::prelude::*;

    // single layer constructors
    let image = Image::from_layer(layer);
    let image = Image::from_channels(resolution, channels);
    
    // use this if the layers have different types
    let image = Image::empty(attributes).with_layer(layer1).with_layer(layer2);

    // use this if the layers have the same type and the above method does not work for you
    let image = Image::from_layers(attributes, smallvec![ layer1, layer2 ]);

    // this constructor accepts any layers object if it implements a certain trait, use this for custom layers
    let image = Image::new(attributes, layers);


    // create an image writer
    image.write()
        
        // print progress (optional, you can remove this line)
        .on_progress(|progress:f64| println!("progress: {:.3}", progress))

        // use only a single cpu (optional, you should remove this line)
        // .non_parallel()

        // alternatively call to_buffered() or to_unbuffered()
        // the file path can be str, String, Path, PathBuf
        .to_file(path);
}
```

### Layers
The simple way to create layers is to use `Layers<_>` or `Layer<_>`. 
The type parameter is the type of channels.  

Use `Layer::new(resolution, attributes, encoding, channels)` to create a layer.
Alternatively, use `smallvec![ layer1, layer2 ]` to create `Layers<_>`, which is a type alias for a list of layers.

```rust
fn main() {
    use exr::prelude::*;

    let layer = Layer::new(
        (1024, 800),
        LayerAttributes::named("first layer"), // name required, other attributes optional
        Encoding::FAST_LOSSLESS, // or Encoding { .. } or Encoding::default()
        channels
    );

    let image = Image::from_layer(layer);
}
```


### Channels
You can create either `SpecificChannels` to write a fixed set of channels, or `AnyChannels` for a dynamic list of channels.

```rust
fn main() {
    use exr::prelude::*;

    let channels = AnyChannels::sort(smallvec![ channel1, channel2, channel3 ]);
    let image = Image::from_channels((1024, 800), channels);
}
```

Alternatively, write specific channels. Start with `SpecificChannels::build()`, 
then call `with_channel(name)` as many times as desired, then call `collect_pixels(..)` to define the colors.
You need to provide a closure that defines the content of the channels: Given the pixel location,
return a tuple with one element per channel. The tuple can contain `f16`, `f32` or `u32` values, 
which then will be written to the file, without converting any value to a different type.

```rust
fn main() {
    use exr::prelude::*;

    let channels = SpecificChannels::build()
        .with_channel("L").with_channel("B")
        .with_pixel_fn(|position: Vec2<usize>| {
            let (l, b) = my_image.lookup_color_at(position.x(), position.y());
            (l as f32, f16::from_f32(b))
        });
    
    let image = Image::from_channels((1024, 800), channels);
}
```

#### RGB, RGBA
There is an even simpler alternative for rgba images, namely `SpecificChannels::rgb` and `SpecificChannels::rgba`:
This is mostly the same as the `SpecificChannels::build` option. 

The rgb method works with three channels per pixel, 
whereas the rgba method works with four channels per pixel. The default alpha value of `1.0` will be used 
if the image does not contain alpha.
```rust
fn main() {
    use exr::prelude::*;

    let channels = SpecificChannels::rgba(|_position| 
        (0.4_f32, 0.2_f32, 0.1_f32, f16::ONE)
    );
    
    let channels = SpecificChannels::rgb(|_position| 
        (0.4_f32, 0.2_f32, 0.1_f32)
    );
    
    let image = Image::from_channels((1024, 800), channels);
}
```

### Channel
The type `AnyChannel` can describe every possible channel and contains all its samples for this layer.   
Use `AnyChannel::new(channel_name, sample_data)` or `AnyChannel { .. }`.
The samples can currently only be `FlatSamples` or `Levels<FlatSamples>`, and in the future might be `DeepSamples`.

### Samples
Currently, only flat samples are supported. These do not contain deep data.  
Construct flat samples directly using `FlatSamples::F16(samples_vec)`, `FlatSamples::F32(samples_vec)`, or `FlatSamples::U32(samples_vec)`.
The vector contains all samples of the layer, row by row (from top to bottom), from left to right.

### Levels
Optionally include Mip Maps or Rip Maps.  
Construct directly using `Levels::Singular(flat_samples)` or `Levels::Mip { .. }` or `Levels::Rip { .. }`.
Put this into the channel, for example`AnyChannel::new("R", Levels::Singular(FlatSamples::F32(vec)))`.

## Full example
Writing a flexible list of channels: 
```rust
fn main(){
    // construct an image to write
    let image = Image::from_layer(
        Layer::new( // the only layer in this image
            (1920, 1080), // resolution
            LayerAttributes::named("main-rgb-layer"), // the layer has a name and other properties
            Encoding::FAST_LOSSLESS, // compress slightly 
            AnyChannels::sort(smallvec![ // the channels contain the actual pixel data
                AnyChannel::new("R", FlatSamples::F32(vec![0.6; 1920*1080 ])), // this channel contains all red values
                AnyChannel::new("G", FlatSamples::F32(vec![0.7; 1920*1080 ])), // this channel contains all green values
                AnyChannel::new("B", FlatSamples::F32(vec![0.9; 1920*1080 ])), // this channel contains all blue values
            ]),
        )
    );

    image.write()
        .on_progress(|progress| println!("progress: {:.1}", progress*100.0)) // optional
        .to_file("image.exr").unwrap();
}
```


### Pixel Closures
When working with specific channels, the data is not stored directly.
Instead, you provide a closure that stores or loads pixels in your existing image data structure.

If you really do not want to provide your own storage, you can use the predefined structures from
`exr::image::pixel_vec`, such as `PixelVec<(f32,f32,f16)>` or `create_pixel_vec`.
Use this only if you don't already have a pixel storage.

```rust
fn main(){
    let read = read()
        .no_deep_data().largest_resolution_level()
        .rgba_channels(
            PixelVec::<(f32,f32,f32,f16)>::constructor, // how to create an image
            PixelVec::set_pixel, // how to update a single pixel in the image
        )/* ... */;
}
```


## Low Level Operations
The image abstraction builds up on some low level code. 
You can use this low level directly, 
as shown in the examples `custom_write.rs` and `custom_read.rs`. 
This allows you to work with 
raw OpenEXR pixel blocks and chunks directly, 
or use custom parallelization mechanisms.

You can find these low level operations in the `exr::block` module.
Start with the `block::read(...)`
and `block::write(...)` functions.

