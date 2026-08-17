use std::borrow::Cow;

use tiff::decoder::Decoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(image) = std::env::args_os().nth(1) else {
        eprintln!("Usage: decode FILE");
        return Ok(());
    };

    let file = std::fs::File::open(image)?;
    let io = std::io::BufReader::new(file);
    let mut reader = Decoder::new(io)?;

    loop {
        ls_dir(&mut reader);

        if !reader.more_images() {
            break;
        }
    }

    Ok(())
}

fn ls_dir<R: std::io::Read + std::io::Seek>(reader: &mut Decoder<R>) {
    if let Some(ifd) = reader.ifd_pointer() {
        println!("Directory at {ifd:x}");
    }

    println!("Name\tHex\tType\tCount");
    let ifd = reader.image_ifd();

    for (tag, entry) in ifd.directory().iter() {
        let name: &'static str = match tag {
            tiff::tags::Tag::Artist => "Artist",
            tiff::tags::Tag::BitsPerSample => "BitsPerSample",
            tiff::tags::Tag::CellLength => "CellLength",
            tiff::tags::Tag::CellWidth => "CellWidth",
            tiff::tags::Tag::ColorMap => "ColorMap",
            tiff::tags::Tag::Compression => "Compression",
            tiff::tags::Tag::DateTime => "DateTime",
            tiff::tags::Tag::ExtraSamples => "ExtraSamples",
            tiff::tags::Tag::FillOrder => "FillOrder",
            tiff::tags::Tag::FreeByteCounts => "FreeByteCounts",
            tiff::tags::Tag::FreeOffsets => "FreeOffsets",
            tiff::tags::Tag::GrayResponseCurve => "GrayResponseCurve",
            tiff::tags::Tag::GrayResponseUnit => "GrayResponseUnit",
            tiff::tags::Tag::HostComputer => "HostComputer",
            tiff::tags::Tag::ImageDescription => "ImageDescription",
            tiff::tags::Tag::ImageLength => "ImageLength",
            tiff::tags::Tag::ImageWidth => "ImageWidth",
            tiff::tags::Tag::Make => "Make",
            tiff::tags::Tag::MaxSampleValue => "MaxSampleValue",
            tiff::tags::Tag::MinSampleValue => "MinSampleValue",
            tiff::tags::Tag::Model => "Model",
            tiff::tags::Tag::NewSubfileType => "NewSubfileType",
            tiff::tags::Tag::Orientation => "Orientation",
            tiff::tags::Tag::PhotometricInterpretation => "PhotometricInterpretation",
            tiff::tags::Tag::PlanarConfiguration => "PlanarConfiguration",
            tiff::tags::Tag::ResolutionUnit => "ResolutionUnit",
            tiff::tags::Tag::RowsPerStrip => "RowsPerStrip",
            tiff::tags::Tag::SamplesPerPixel => "SamplesPerPixel",
            tiff::tags::Tag::Software => "Software",
            tiff::tags::Tag::StripByteCounts => "StripByteCounts",
            tiff::tags::Tag::StripOffsets => "StripOffsets",
            tiff::tags::Tag::SubfileType => "SubfileType",
            tiff::tags::Tag::Threshholding => "Threshholding",
            tiff::tags::Tag::XResolution => "XResolution",
            tiff::tags::Tag::YResolution => "YResolution",
            tiff::tags::Tag::Predictor => "Predictor",
            tiff::tags::Tag::TileWidth => "TileWidth",
            tiff::tags::Tag::TileLength => "TileLength",
            tiff::tags::Tag::TileOffsets => "TileOffsets",
            tiff::tags::Tag::TileByteCounts => "TileByteCounts",
            tiff::tags::Tag::SubIfd => "SubIfd",
            tiff::tags::Tag::SampleFormat => "SampleFormat",
            tiff::tags::Tag::SMinSampleValue => "SMinSampleValue",
            tiff::tags::Tag::SMaxSampleValue => "SMaxSampleValue",
            tiff::tags::Tag::JPEGTables => "JPEGTables",
            tiff::tags::Tag::ChromaSubsampling => "ChromaSubsampling",
            tiff::tags::Tag::ChromaPositioning => "ChromaPositioning",
            tiff::tags::Tag::ModelPixelScaleTag => "ModelPixelScaleTag",
            tiff::tags::Tag::ModelTransformationTag => "ModelTransformationTag",
            tiff::tags::Tag::ModelTiepointTag => "ModelTiepointTag",
            tiff::tags::Tag::Copyright => "Copyright",
            tiff::tags::Tag::ExifDirectory => "ExifDirectory",
            tiff::tags::Tag::GpsDirectory => "GpsDirectory",
            tiff::tags::Tag::IccProfile => "IccProfile",
            tiff::tags::Tag::GeoKeyDirectoryTag => "GeoKeyDirectoryTag",
            tiff::tags::Tag::GeoDoubleParamsTag => "GeoDoubleParamsTag",
            tiff::tags::Tag::GeoAsciiParamsTag => "GeoAsciiParamsTag",
            tiff::tags::Tag::ExifVersion => "ExifVersion",
            tiff::tags::Tag::GdalNodata => "GdalNodata",
            _ => "<unknown>",
        };

        let ty: Cow<'static, str> = match entry.field_type() {
            tiff::tags::Type::BYTE => "u8".into(),
            tiff::tags::Type::ASCII => "ascii".into(),
            tiff::tags::Type::SHORT => "u16".into(),
            tiff::tags::Type::LONG => "u32".into(),
            tiff::tags::Type::RATIONAL => "r32".into(),
            tiff::tags::Type::SBYTE => "i8".into(),
            tiff::tags::Type::UNDEFINED => "byte".into(),
            tiff::tags::Type::SSHORT => "s16".into(),
            tiff::tags::Type::SLONG => "s32".into(),
            tiff::tags::Type::SRATIONAL => "sr32".into(),
            tiff::tags::Type::FLOAT => "f32".into(),
            tiff::tags::Type::DOUBLE => "f64".into(),
            tiff::tags::Type::IFD => "ifd32".into(),
            tiff::tags::Type::LONG8 => "u64".into(),
            tiff::tags::Type::SLONG8 => "i64".into(),
            tiff::tags::Type::IFD8 => "ifd64".into(),
            other => format!("{:x}", other.to_u16()).into(),
        };

        eprintln!(
            "{name:16}\t{tag:4x}\t{ty}\t{count}",
            tag = tag.to_u16(),
            count = entry.count(),
        );
    }
}
