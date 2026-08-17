extern crate exr;

extern crate smallvec;

use std::{panic};
use std::io::{Cursor};
use std::panic::catch_unwind;
use std::path::{PathBuf, Path};
use std::ffi::OsStr;

use exr::prelude::*;
use exr::error::{Error, UnitResult};
use exr::prelude::pixel_vec::PixelVec;
use exr::image::validate_results::ValidateResult;
use rayon::prelude::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use exr::block::samples::IntoNativeSample;

#[test]
fn roundtrip_all_files_in_repository_x4(){
    check_all_files_in_repo(|path|{
        let file = std::fs::read(path).expect("cannot open file");

        round_trip_simple(&file)?;
        round_trip_full(&file)?;
        round_trip_rgba_file(path, &file)?;
        round_trip_parallel_file(&file)?;

        Ok(())
    });
}


fn round_trip_full(file: &[u8]) -> Result<()> {
    let read_image = read()
        .no_deep_data().all_resolution_levels().all_channels().all_layers().all_attributes()
        .non_parallel();

    let image = read_image.clone().from_buffered(Cursor::new(file))?;

    let mut tmp_bytes = Vec::with_capacity(file.len());
    image.write().non_parallel().to_buffered(Cursor::new(&mut tmp_bytes))?;

    let image2 = read_image.from_buffered(Cursor::new(tmp_bytes))?;

    image.assert_equals_result(&image2);
    Ok(())
}

fn round_trip_simple(file: &[u8]) -> Result<()> {
    let read_image = read()
        .no_deep_data().largest_resolution_level().all_channels().all_layers().all_attributes()
        .non_parallel();

    let image = read_image.clone().from_buffered(Cursor::new(file))?;

    let mut tmp_bytes = Vec::with_capacity(file.len());
    image.write().non_parallel().to_buffered(&mut Cursor::new(&mut tmp_bytes))?;

    let image2 = read_image.from_buffered(Cursor::new(&tmp_bytes))?;

    image.assert_equals_result(&image2);
    Ok(())
}

fn round_trip_rgba_file(path: &Path, file: &[u8]) -> Result<()> {
    // these files are known to be invalid, because they do not contain any rgb channels
    let blacklist = [
        Path::new("tests/images/valid/openexr/LuminanceChroma/Garden.exr"),
        Path::new("tests/images/valid/openexr/MultiView/Fog.exr"),
        Path::new("tests/images/valid/openexr/TestImages/GrayRampsDiagonal.exr"),
        Path::new("tests/images/valid/openexr/TestImages/GrayRampsHorizontal.exr"),
        Path::new("tests/images/valid/openexr/TestImages/WideFloatRange.exr"),
        Path::new("tests/images/valid/openexr/IlmfmlmflmTest/v1.7.test.tiled.exr")
    ];

    if blacklist.contains(&path) { return Ok(()) }

    let image_reader = read()
        .no_deep_data()
        .largest_resolution_level() // TODO all levels
        .rgba_channels(PixelVec::<(f32,f32,f32,f32)>::constructor, PixelVec::set_pixel)
        .first_valid_layer()
        .all_attributes()
        .non_parallel();

    let image = image_reader.clone().from_buffered(Cursor::new(file))?;

    let mut tmp_bytes = Vec::with_capacity(file.len());

    image.write().non_parallel()
        .to_buffered(&mut Cursor::new(&mut tmp_bytes))?;

    let image2 = image_reader.from_buffered(Cursor::new(&tmp_bytes))?;

    image.assert_equals_result(&image2);
    Ok(())
}

// TODO compare rgba vs rgb images for color content, and rgb vs rgb(a?)


fn round_trip_parallel_file(file: &[u8]) -> Result<()> {
    let image = read()
        .no_deep_data().all_resolution_levels().all_channels().all_layers().all_attributes()
        .from_buffered(Cursor::new(file))?;

    let mut tmp_bytes = Vec::with_capacity(file.len());
    image.write().to_buffered(Cursor::new(&mut tmp_bytes))?;

    let image2 = read()
        .no_deep_data().all_resolution_levels().all_channels().all_layers().all_attributes()
        .pedantic()
        .from_buffered(Cursor::new(tmp_bytes.as_slice()))?;

    image.assert_equals_result(&image2);
    Ok(())
}

/// read all images in a directory.
/// does not check any content, just checks whether a read error or panic happened.
fn check_all_files_in_repo<T>(
    operation: impl Sync + std::panic::RefUnwindSafe + Fn(&Path) -> exr::error::Result<T>
) {
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    enum Result { Ok, Unsupported(String), Error(String) }

    let files: Vec<PathBuf> = all_exr_files_in_repo().collect();
    let mut results: Vec<(PathBuf, Result)> = files.into_par_iter()
        .map(|file| {
            let result = catch_unwind(||{
                let prev_hook = panic::take_hook();
                panic::set_hook(Box::new(|_| (/* do not println panics */)));
                let result = operation(&file);
                panic::set_hook(prev_hook);

                result
            });

            let result = match result {
                Ok(Ok(_)) => Result::Ok,
                Ok(Err(Error::NotSupported(message))) => Result::Unsupported(message.to_string()),

                Ok(Err(Error::Io(io))) => Result::Error(format!("IoError: {:?}", io)),
                Ok(Err(Error::Invalid(message))) => Result::Error(format!("Invalid: {:?}", message)),
                Ok(Err(Error::Aborted)) => panic!("a test produced `Error::Abort`"),

                Err(_) => Result::Error("Panic".to_owned()),
            };

            match &result {
                Result::Error(_) => println!("✗ Error when processing {:?}", file),
                _ => println!("✓ No error when processing {:?}", file)
            };

            (file, result)
        })
        .collect();

    results.sort_by(|(_, a), (_, b)| a.cmp(b));

    println!("{:#?}", results.iter().map(|(path, result)| {
        format!("{:?}: {}", result, path.to_str().unwrap())
    }).collect::<Vec<_>>());

    assert!(results.len() > 80, "Not enough files were tested!");

    if let Result::Error(_) = results.last().unwrap().1 {
        panic!("A file triggered a panic");
    }
}

fn all_exr_files_in_repo() -> impl Iterator<Item=PathBuf> {
    walkdir::WalkDir::new("tests/images/valid").into_iter().map(std::result::Result::unwrap)
        .filter(|entry| entry.path().extension() == Some(OsStr::new("exr")))
        .map(walkdir::DirEntry::into_path)
}

#[test]
fn roundtrip_unusual_2() -> UnitResult {
    let random_pixels: Vec<(f16, u32)> = vec![
        ( f16::from_f32(-5.0), 4),
        ( f16::from_f32(4.0), 9),
        ( f16::from_f32(2.0), 6),
        ( f16::from_f32(21.0), 8),
        ( f16::from_f32(64.0), 7),
    ];

    let size = Vec2(3, 2);
    let pixels = (0..size.area())
        .zip(random_pixels.into_iter().cycle())
        .map(|(_index, color)| color).collect::<Vec<_>>();

    let pixels = PixelVec { resolution: size, pixels };

    let channels = SpecificChannels::build()
        .with_channel("N")
        .with_channel("Ploppalori Taranos")
        .with_pixels(pixels.clone()
    );

    let image = Image::from_channels(size, channels);

    let mut tmp_bytes = Vec::new();
    image.write().non_parallel().to_buffered(&mut Cursor::new(&mut tmp_bytes))?;

    let image_reader = read()
        .no_deep_data()
        .largest_resolution_level() // TODO all levels
        .specific_channels().required("N").required("Ploppalori Taranos")
        .collect_pixels(PixelVec::<(f16,u32)>::constructor, PixelVec::set_pixel)
        .first_valid_layer()
        .all_attributes()
        .non_parallel();

    let image2 = image_reader.from_buffered(Cursor::new(&tmp_bytes))?;

    // custom compare function: considers nan equal to nan
    assert_eq!(image.layer_data.size, size, "test is buggy");
    let pixels1 = &image.layer_data.channel_data.pixels;
    let pixels2 = &image2.layer_data.channel_data.pixels;

    assert_eq!(pixels1.pixels, pixels2.pixels);

    Ok(())
}

// TODO test optional reader
// TODO dedup
#[test]
fn roundtrip_unusual_7() -> UnitResult {

    let random_pixels: Vec<(f16, u32, f32,f32,f32,f32,f32)> = vec![
        ( f16::from_f32(-5.0), 4, 1.0,2.0,3.0,4.0,5.0),
        ( f16::from_f32(4.0), 8, 2.0,3.0,4.0,5.0,1.0),
        ( f16::from_f32(2.0), 9, 3.0,4.0,5.0,1.0,2.0),
        ( f16::from_f32(21.0), 6, 4.0,5.0,1.0,2.0,3.0),
        ( f16::from_f32(64.0), 5, 5.0,1.0,2.0,3.0,4.0),
    ];

    let size = Vec2(3, 2);
    let pixels = (0..size.area())
        .zip(random_pixels.into_iter().cycle())
        .map(|(_index, color)| color).collect::<Vec<_>>();

    let pixels = PixelVec { resolution: size, pixels };

    let channels = SpecificChannels::build()
        .with_channel("N")
        .with_channel("Ploppalori Taranos")
        .with_channel("4")
        .with_channel(".")
        .with_channel("____")
        .with_channel(" ")
        .with_channel("  ")
        .with_pixels(pixels.clone()
    );

    let image = Image::from_channels(size, channels);

    let mut tmp_bytes = Vec::new();
    image.write().non_parallel().to_buffered(&mut Cursor::new(&mut tmp_bytes))?;

    let image_reader = read()
        .no_deep_data()
        .largest_resolution_level() // TODO all levels
        .specific_channels()
            .required("N")
            .required("Ploppalori Taranos")
            .required("4")
            .required(".")
            .required("____")
            .required(" ")
            .required("  ")
        .collect_pixels(PixelVec::<(f16, u32, f32,f32,f32,f32,f32)>::constructor, PixelVec::set_pixel)
        .first_valid_layer()
        .all_attributes()
        .non_parallel();

    let image2 = image_reader.from_buffered(Cursor::new(&tmp_bytes))?;

    // custom compare function: considers nan equal to nan
    assert_eq!(image.layer_data.size, size, "test is buggy");
    let pixels1 = &image.layer_data.channel_data.pixels;
    let pixels2 = &image2.layer_data.channel_data.pixels;

    assert_eq!(pixels1.pixels, pixels2.pixels);
    Ok(())
}

#[test]
fn roundtrip_pxr24() {
    test_mixed_roundtrip_with_compression(Compression::PXR24)
}

#[test]
fn roundtrip_rle() {
    test_mixed_roundtrip_with_compression(Compression::RLE)
}

#[test]
fn roundtrip_zip1() {
    test_mixed_roundtrip_with_compression(Compression::ZIP1)
}

#[test]
fn roundtrip_zip16() {
    test_mixed_roundtrip_with_compression(Compression::ZIP16)
}

#[test]
fn roundtrip_b44() {
    test_mixed_roundtrip_with_compression(Compression::B44)
}

#[test]
fn roundtrip_b44a() {
    test_mixed_roundtrip_with_compression(Compression::B44A)
}

#[test]
fn roundtrip_piz() {
    test_mixed_roundtrip_with_compression(Compression::PIZ)
}

#[test]
fn roundtrip_uncompressed() {
    test_mixed_roundtrip_with_compression(Compression::Uncompressed)
}

fn test_mixed_roundtrip_with_compression(compression: Compression) {

    let original_pixels: [(f16,f32,f32); 4] = [
        (0.0.to_f16(), -1.1, std::f32::consts::PI),
        (9.1.to_f16(), -3.1, std::f32::consts::TAU),
        (-10.0.to_f16(), -11.1, f32::EPSILON),
        (half::f16::NAN, 10000.1, -1024.009),
    ];

    let mut file_bytes = Vec::new();
    let original_image = Image::from_encoded_channels(
        (2,2),
        Encoding {
            compression,
            .. Encoding::default()
        },
        SpecificChannels::rgb(
            PixelVec::new(Vec2(2,2), original_pixels.to_vec())
        )
    );

    original_image.write().to_buffered(Cursor::new(&mut file_bytes)).unwrap();

    let lossy_image = read().no_deep_data().largest_resolution_level()
        .rgb_channels(PixelVec::<(f16,f32,f32)>::constructor, PixelVec::set_pixel)
        .first_valid_layer().all_attributes().from_buffered(Cursor::new(&file_bytes)).unwrap();

    // use automatic lossy detection by compression method
    original_image.assert_equals_result(&original_image);
    lossy_image.assert_equals_result(&lossy_image);
    original_image.assert_equals_result(&lossy_image);
}
