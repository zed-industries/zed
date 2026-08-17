
//! Contains collections of common attributes.
//! Defines some data types that list all standard attributes.

use std::collections::HashMap;
use crate::meta::attribute::*; // FIXME shouldn't this need some more imports????
use crate::meta::*;
use crate::math::Vec2;

// TODO rename header to LayerDescription!

/// Describes a single layer in a file.
/// A file can have any number of layers.
/// The meta data contains one header per layer.
#[derive(Clone, Debug, PartialEq)]
pub struct Header {

    /// List of channels in this layer.
    pub channels: ChannelList,

    /// How the pixel data of all channels in this layer is compressed. May be `Compression::Uncompressed`.
    pub compression: Compression,

    /// Describes how the pixels of this layer are divided into smaller blocks.
    /// A single block can be loaded without processing all bytes of a file.
    ///
    /// Also describes whether a file contains multiple resolution levels: mip maps or rip maps.
    /// This allows loading not the full resolution, but the smallest sensible resolution.
    //
    // Required if file contains deep data or multiple layers.
    // Note: This value must agree with the version field's tile bit and deep data bit.
    // In this crate, this attribute will always have a value, for simplicity.
    pub blocks: BlockDescription,

    /// In what order the tiles of this header occur in the file.
    pub line_order: LineOrder,

    /// The resolution of this layer. Equivalent to the size of the `DataWindow`.
    pub layer_size: Vec2<usize>,

    /// Whether this layer contains deep data.
    pub deep: bool,

    /// This library supports only deep data version 1.
    pub deep_data_version: Option<i32>,

    /// Number of chunks, that is, scan line blocks or tiles, that this image has been divided into.
    /// This number is calculated once at the beginning
    /// of the read process or when creating a header object.
    ///
    /// This value includes all chunks of all resolution levels.
    ///
    ///
    /// __Warning__
    /// _This value is relied upon. You should probably use `Header::with_encoding`,
    /// which automatically updates the chunk count._
    pub chunk_count: usize,

    // Required for deep data (deepscanline and deeptile) layers.
    // Note: Since the value of "maxSamplesPerPixel"
    // maybe be unknown at the time of opening the
    // file, the value “ -1 ” is written to the file to
    // indicate an unknown value. When the file is
    // closed, this will be overwritten with the correct value.
    // If file writing does not complete
    // correctly due to an error, the value -1 will
    // remain. In this case, the value must be derived
    // by decoding each chunk in the layer
    /// Maximum number of samples in a single pixel in a deep image.
    pub max_samples_per_pixel: Option<usize>,

    /// Includes mandatory fields like pixel aspect or display window
    /// which must be the same for all layers.
    pub shared_attributes: ImageAttributes,

    /// Does not include the attributes required for reading the file contents.
    /// Excludes standard fields that must be the same for all headers.
    pub own_attributes: LayerAttributes,
}

/// Includes mandatory fields like pixel aspect or display window
/// which must be the same for all layers.
/// For more attributes, see struct `LayerAttributes`.
#[derive(Clone, PartialEq, Debug)]
pub struct ImageAttributes {

    /// The rectangle anywhere in the global infinite 2D space
    /// that clips all contents of the file.
    pub display_window: IntegerBounds,

    /// Aspect ratio of each pixel in this header.
    pub pixel_aspect: f32,

    /// The chromaticities attribute of the image. See the `Chromaticities` type.
    pub chromaticities: Option<Chromaticities>,

    /// The time code of the image.
    pub time_code: Option<TimeCode>,

    /// Contains custom attributes.
    /// Does not contain the attributes already present in the `ImageAttributes`.
    /// Contains only attributes that are standardized to be the same for all headers: chromaticities and time codes.
    pub other: HashMap<Text, AttributeValue>,
}

/// Does not include the attributes required for reading the file contents.
/// Excludes standard fields that must be the same for all headers.
/// For more attributes, see struct `ImageAttributes`.
#[derive(Clone, PartialEq)]
pub struct LayerAttributes {

    /// The name of this layer.
    /// Required if this file contains deep data or multiple layers.
    // As this is an attribute value, it is not restricted in length, may even be empty
    pub layer_name: Option<Text>,

    /// The top left corner of the rectangle that positions this layer
    /// within the global infinite 2D space of the whole file.
    /// This represents the position of the `DataWindow`.
    pub layer_position: Vec2<i32>,

    /// Part of the perspective projection. Default should be `(0, 0)`.
    // TODO same for all layers?
    pub screen_window_center: Vec2<f32>,

    // TODO same for all layers?
    /// Part of the perspective projection. Default should be `1`.
    pub screen_window_width: f32,

    /// The white luminance of the colors.
    /// Defines the luminance in candelas per square meter, Nits, of the rgb value `(1, 1, 1)`.
    // If the chromaticities and the whiteLuminance of an RGB image are
    // known, then it is possible to convert the image's pixels from RGB
    // to CIE XYZ tristimulus values (see function RGBtoXYZ() in header
    // file ImfChromaticities.h).
    pub white_luminance: Option<f32>,

    /// The adopted neutral of the colors. Specifies the CIE (x,y) frequency coordinates that should
    /// be considered neutral during color rendering. Pixels in the image
    /// whose CIE (x,y) frequency coordinates match the adopted neutral value should
    /// be mapped to neutral values on the given display.
    pub adopted_neutral: Option<Vec2<f32>>,

    /// Name of the color transform function that is applied for rendering the image.
    pub rendering_transform_name: Option<Text>,

    /// Name of the color transform function that computes the look modification of the image.
    pub look_modification_transform_name: Option<Text>,

    /// The horizontal density, in pixels per inch.
    /// The image's vertical output density can be computed using `horizontal_density * pixel_aspect_ratio`.
    pub horizontal_density: Option<f32>,

    /// Name of the owner.
    pub owner: Option<Text>,

    /// Additional textual information.
    pub comments: Option<Text>,

    /// The date of image creation, in `YYYY:MM:DD hh:mm:ss` format.
    // TODO parse!
    pub capture_date: Option<Text>,

    /// Time offset from UTC.
    pub utc_offset: Option<f32>,

    /// Geographical image location.
    pub longitude: Option<f32>,

    /// Geographical image location.
    pub latitude: Option<f32>,

    /// Geographical image location.
    pub altitude: Option<f32>,

    /// Camera focus in meters.
    pub focus: Option<f32>,

    /// Exposure time in seconds.
    pub exposure: Option<f32>,

    /// Camera aperture measured in f-stops. Equals the focal length
    /// of the lens divided by the diameter of the iris opening.
    pub aperture: Option<f32>,

    /// Iso-speed of the camera sensor.
    pub iso_speed: Option<f32>,

    /// If this is an environment map, specifies how to interpret it.
    pub environment_map: Option<EnvironmentMap>,

    /// Identifies film manufacturer, film type, film roll and frame position within the roll.
    pub film_key_code: Option<KeyCode>,

    /// Specifies how texture map images are extrapolated.
    /// Values can be `black`, `clamp`, `periodic`, or `mirror`.
    pub wrap_mode_name: Option<Text>,

    /// Frames per second if this is a frame in a sequence.
    pub frames_per_second: Option<Rational>,

    /// Specifies the view names for multi-view, for example stereo, image files.
    pub multi_view_names: Option<Vec<Text>>,

    /// The matrix that transforms 3D points from the world to the camera coordinate space.
    /// Left-handed coordinate system, y up, z forward.
    pub world_to_camera: Option<Matrix4x4>,

    /// The matrix that transforms 3D points from the world to the "Normalized Device Coordinate" space.
    /// Left-handed coordinate system, y up, z forward.
    pub world_to_normalized_device: Option<Matrix4x4>,

    /// Specifies whether the pixels in a deep image are sorted and non-overlapping.
    pub deep_image_state: Option<Rational>,

    /// If the image was cropped, contains the original data window.
    pub original_data_window: Option<IntegerBounds>,

    /// An 8-bit rgba image representing the rendered image.
    pub preview: Option<Preview>,

    /// Name of the view, which is typically either `"right"` or `"left"` for a stereoscopic image.
    pub view_name: Option<Text>,

    /// The name of the software that produced this image.
    pub software_name: Option<Text>,

    /// The near clip plane of the virtual camera projection.
    pub near_clip_plane: Option<f32>,

    /// The far clip plane of the virtual camera projection.
    pub far_clip_plane: Option<f32>,

    /// The field of view angle, along the horizontal axis, in degrees.
    pub horizontal_field_of_view: Option<f32>,

    /// The field of view angle, along the horizontal axis, in degrees.
    pub vertical_field_of_view: Option<f32>,

    /// Contains custom attributes.
    /// Does not contain the attributes already present in the `Header` or `LayerAttributes` struct.
    /// Does not contain attributes that are standardized to be the same for all layers: no chromaticities and no time codes.
    pub other: HashMap<Text, AttributeValue>,
}


impl LayerAttributes {

    /// Create default layer attributes with a data position of zero.
    pub fn named(layer_name: impl Into<Text>) -> Self {
        Self {
            layer_name: Some(layer_name.into()),
            .. Self::default()
        }
    }

    /// Set the data position of this layer.
    pub fn with_position(self, data_position: Vec2<i32>) -> Self {
        Self { layer_position: data_position, ..self }
    }

    /// Set all common camera projection attributes at once.
    pub fn with_camera_frustum(
        self,
        world_to_camera: Matrix4x4,
        world_to_normalized_device: Matrix4x4,
        field_of_view: impl Into<Vec2<f32>>,
        depth_clip_range: std::ops::Range<f32>,
    ) -> Self
    {
        let fov = field_of_view.into();

        Self {
            world_to_normalized_device: Some(world_to_normalized_device),
            world_to_camera: Some(world_to_camera),
            horizontal_field_of_view: Some(fov.x()),
            vertical_field_of_view: Some(fov.y()),
            near_clip_plane: Some(depth_clip_range.start),
            far_clip_plane: Some(depth_clip_range.end),
            ..self
        }
    }
}

impl ImageAttributes {

    /// Set the display position and size of this image.
    pub fn new(display_window: IntegerBounds) -> Self {
        Self {
            pixel_aspect: 1.0,
            chromaticities: None,
            time_code: None,
            other: Default::default(),
            display_window,
        }
    }

    /// Set the display position to zero and use the specified size for this image.
    pub fn with_size(size: impl Into<Vec2<usize>>) -> Self {
        Self::new(IntegerBounds::from_dimensions(size))
    }
}




impl Header {

    /// Create a new Header with the specified name, display window and channels.
    /// Use `Header::with_encoding` and the similar methods to add further properties to the header.
    ///
    /// The other settings are left to their default values:
    /// - RLE compression
    /// - display window equal to data window
    /// - tiles (64 x 64 px)
    /// - unspecified line order
    /// - no custom attributes
    pub fn new(name: Text, data_size: impl Into<Vec2<usize>>, channels: SmallVec<[ChannelDescription; 5]>) -> Self {
        let data_size: Vec2<usize> = data_size.into();

        let compression = Compression::RLE;
        let blocks = BlockDescription::Tiles(TileDescription {
            tile_size: Vec2(64, 64),
            level_mode: LevelMode::Singular,
            rounding_mode: RoundingMode::Down
        });

        Self {
            layer_size: data_size,
            compression,
            blocks,

            channels: ChannelList::new(channels),
            line_order: LineOrder::Unspecified,

            shared_attributes: ImageAttributes::with_size(data_size),
            own_attributes: LayerAttributes::named(name),

            chunk_count: compute_chunk_count(compression, data_size, blocks),

            deep: false,
            deep_data_version: None,
            max_samples_per_pixel: None,
        }
    }

    /// Set the display window, that is, the global clipping rectangle.
    /// __Must be the same for all headers of a file.__
    pub fn with_display_window(mut self, display_window: IntegerBounds) -> Self {
        self.shared_attributes.display_window = display_window;
        self
    }

    /// Set the offset of this layer.
    pub fn with_position(mut self, position: Vec2<i32>) -> Self {
        self.own_attributes.layer_position = position;
        self
    }

    /// Set compression, tiling, and line order. Automatically computes chunk count.
    pub fn with_encoding(self, compression: Compression, blocks: BlockDescription, line_order: LineOrder) -> Self {
        Self {
            chunk_count: compute_chunk_count(compression, self.layer_size, blocks),
            compression, blocks, line_order,
            .. self
        }
    }

    /// Set **all** attributes of the header that are not shared with all other headers in the image.
    pub fn with_attributes(self, own_attributes: LayerAttributes) -> Self {
        Self { own_attributes, .. self }
    }

    /// Set **all** attributes of the header that are shared with all other headers in the image.
    pub fn with_shared_attributes(self, shared_attributes: ImageAttributes) -> Self {
        Self { shared_attributes, .. self }
    }

    /// Iterate over all blocks, in the order specified by the headers line order attribute.
    /// Unspecified line order is treated as increasing line order.
    /// Also enumerates the index of each block in the header, as if it were sorted in increasing line order.
    pub fn enumerate_ordered_blocks(&self) -> impl Iterator<Item=(usize, TileIndices)> + Send {
        let increasing_y = self.blocks_increasing_y_order().enumerate();

        // TODO without box?
        let ordered: Box<dyn Send + Iterator<Item=(usize, TileIndices)>> = {
            if self.line_order == LineOrder::Decreasing { Box::new(increasing_y.rev()) }
            else { Box::new(increasing_y) }
        };

        ordered
    }

    /*/// Iterate over all blocks, in the order specified by the headers line order attribute.
    /// Also includes an index of the block if it were `LineOrder::Increasing`, starting at zero for this header.
    pub fn enumerate_ordered_blocks(&self) -> impl Iterator<Item = (usize, TileIndices)> + Send {
        let increasing_y = self.blocks_increasing_y_order().enumerate();

        let ordered: Box<dyn Send + Iterator<Item = (usize, TileIndices)>> = {
            if self.line_order == LineOrder::Decreasing {
                Box::new(increasing_y.rev()) // TODO without box?
            }
            else {
                Box::new(increasing_y)
            }
        };

        ordered
    }*/

    /// Iterate over all tile indices in this header in `LineOrder::Increasing` order.
    pub fn blocks_increasing_y_order(&self) -> impl Iterator<Item = TileIndices> + ExactSizeIterator + DoubleEndedIterator {
        fn tiles_of(image_size: Vec2<usize>, tile_size: Vec2<usize>, level_index: Vec2<usize>) -> impl Iterator<Item=TileIndices> {
            fn divide_and_rest(total_size: usize, block_size: usize) -> impl Iterator<Item=(usize, usize)> {
                let block_count = compute_block_count(total_size, block_size);
                (0..block_count).map(move |block_index| (
                    block_index, calculate_block_size(total_size, block_size, block_index).expect("block size calculation bug")
                ))
            }

            divide_and_rest(image_size.height(), tile_size.height()).flat_map(move |(y_index, tile_height)|{
                divide_and_rest(image_size.width(), tile_size.width()).map(move |(x_index, tile_width)|{
                    TileIndices {
                        size: Vec2(tile_width, tile_height),
                        location: TileCoordinates { tile_index: Vec2(x_index, y_index), level_index, },
                    }
                })
            })
        }

        let vec: Vec<TileIndices> = {
            if let BlockDescription::Tiles(tiles) = self.blocks {
                match tiles.level_mode {
                    LevelMode::Singular => {
                        tiles_of(self.layer_size, tiles.tile_size, Vec2(0, 0)).collect()
                    },
                    LevelMode::MipMap => {
                        mip_map_levels(tiles.rounding_mode, self.layer_size)
                            .flat_map(move |(level_index, level_size)|{
                                tiles_of(level_size, tiles.tile_size, Vec2(level_index, level_index))
                            })
                            .collect()
                    },
                    LevelMode::RipMap => {
                        rip_map_levels(tiles.rounding_mode, self.layer_size)
                            .flat_map(move |(level_index, level_size)| {
                                tiles_of(level_size, tiles.tile_size, level_index)
                            })
                            .collect()
                    }
                }
            }
            else {
                let tiles = Vec2(self.layer_size.0, self.compression.scan_lines_per_block());
                tiles_of(self.layer_size, tiles, Vec2(0, 0)).collect()
            }
        };

        vec.into_iter() // TODO without collect
    }

    /* TODO
    /// The block indices of this header, ordered as they would appear in the file.
    pub fn ordered_block_indices<'s>(&'s self, layer_index: usize) -> impl 's + Iterator<Item=BlockIndex> {
        self.enumerate_ordered_blocks().map(|(chunk_index, tile)|{
            let data_indices = self.get_absolute_block_pixel_coordinates(tile.location).expect("tile coordinate bug");

            BlockIndex {
                layer: layer_index,
                level: tile.location.level_index,
                pixel_position: data_indices.position.to_usize("data indices start").expect("data index bug"),
                pixel_size: data_indices.size,
            }
        })
    }*/

    // TODO reuse this function everywhere
    /// The default pixel resolution of a single block (tile or scan line block).
    /// Not all blocks have this size, because they may be cutoff at the end of the image.
    pub fn max_block_pixel_size(&self) -> Vec2<usize> {
        match self.blocks {
            BlockDescription::ScanLines => Vec2(self.layer_size.0, self.compression.scan_lines_per_block()),
            BlockDescription::Tiles(tiles) => tiles.tile_size,
        }
    }

    /// Calculate the position of a block in the global infinite 2D space of a file. May be negative.
    pub fn get_block_data_window_pixel_coordinates(&self, tile: TileCoordinates) -> Result<IntegerBounds> {
        let data = self.get_absolute_block_pixel_coordinates(tile)?;
        Ok(data.with_origin(self.own_attributes.layer_position))
    }

    /// Calculate the pixel index rectangle inside this header. Is not negative. Starts at `0`.
    pub fn get_absolute_block_pixel_coordinates(&self, tile: TileCoordinates) -> Result<IntegerBounds> {
        if let BlockDescription::Tiles(tiles) = self.blocks {
            let Vec2(data_width, data_height) = self.layer_size;

            let data_width = compute_level_size(tiles.rounding_mode, data_width, tile.level_index.x());
            let data_height = compute_level_size(tiles.rounding_mode, data_height, tile.level_index.y());
            let absolute_tile_coordinates = tile.to_data_indices(tiles.tile_size, Vec2(data_width, data_height))?;

            if absolute_tile_coordinates.position.x() as i64 >= data_width as i64 || absolute_tile_coordinates.position.y() as i64 >= data_height as i64 {
                return Err(Error::invalid("data block tile index"))
            }

            Ok(absolute_tile_coordinates)
        }
        else { // this is a scanline image
            debug_assert_eq!(tile.tile_index.0, 0, "block index calculation bug");

            let (y, height) = calculate_block_position_and_size(
                self.layer_size.height(),
                self.compression.scan_lines_per_block(),
                tile.tile_index.y()
            )?;

            Ok(IntegerBounds {
                position: Vec2(0, usize_to_i32(y, "tile y")?),
                size: Vec2(self.layer_size.width(), height)
            })
        }

        // TODO deep data?
    }

    /// Return the tile index, converting scan line block coordinates to tile indices.
    /// Starts at `0` and is not negative.
    pub fn get_block_data_indices(&self, block: &CompressedBlock) -> Result<TileCoordinates> {
        Ok(match block {
            CompressedBlock::Tile(ref tile) => {
                tile.coordinates
            },

            CompressedBlock::ScanLine(ref block) => {
                let size = self.compression.scan_lines_per_block() as i32;

                let diff = block.y_coordinate.checked_sub(self.own_attributes.layer_position.y()).ok_or(Error::invalid("invalid header"))?;
                let y = diff.checked_div(size).ok_or(Error::invalid("invalid header"))?;

                if y < 0 {
                    return Err(Error::invalid("scan block y coordinate"));
                }

                TileCoordinates {
                    tile_index: Vec2(0, y as usize),
                    level_index: Vec2(0, 0)
                }
            },

            _ => return Err(Error::unsupported("deep data not supported yet"))
        })
    }

    /// Computes the absolute tile coordinate data indices, which start at `0`.
    pub fn get_scan_line_block_tile_coordinates(&self, block_y_coordinate: i32) -> Result<TileCoordinates> {
        let size = self.compression.scan_lines_per_block() as i32;

        let diff = block_y_coordinate.checked_sub(self.own_attributes.layer_position.1).ok_or(Error::invalid("invalid header"))?;
        let y = diff.checked_div(size).ok_or(Error::invalid("invalid header"))?;

        if y < 0 {
            return Err(Error::invalid("scan block y coordinate"));
        }

        Ok(TileCoordinates {
            tile_index: Vec2(0, y as usize),
            level_index: Vec2(0, 0)
        })
    }

    /// Maximum byte length of an uncompressed or compressed block, used for validation.
    pub fn max_block_byte_size(&self) -> usize {
        self.channels.bytes_per_pixel * match self.blocks {
            BlockDescription::Tiles(tiles) => tiles.tile_size.area(),
            BlockDescription::ScanLines => self.compression.scan_lines_per_block() * self.layer_size.width()
            // TODO What about deep data???
        }
    }

    /// Returns the number of bytes that the pixels of this header will require
    /// when stored without compression. Respects multi-resolution levels and subsampling.
    pub fn total_pixel_bytes(&self) -> usize {
        assert!(!self.deep);

        let pixel_count_of_levels = |size: Vec2<usize>| -> usize {
            match self.blocks {
                BlockDescription::ScanLines => size.area(),
                BlockDescription::Tiles(tile_description) => match tile_description.level_mode {
                    LevelMode::Singular => size.area(),

                    LevelMode::MipMap => mip_map_levels(tile_description.rounding_mode, size)
                        .map(|(_, size)| size.area()).sum(),

                    LevelMode::RipMap => rip_map_levels(tile_description.rounding_mode, size)
                        .map(|(_, size)| size.area()).sum(),
                }
            }
        };

        self.channels.list.iter()
            .map(|channel: &ChannelDescription|
                pixel_count_of_levels(channel.subsampled_resolution(self.layer_size)) * channel.sample_type.bytes_per_sample()
            )
            .sum()

    }

    /// Approximates the maximum number of bytes that the pixels of this header will consume in a file.
    /// Due to compression, the actual byte size may be smaller.
    pub fn max_pixel_file_bytes(&self) -> usize {
        assert!(!self.deep);

        self.chunk_count * 64 // at most 64 bytes overhead for each chunk (header index, tile description, chunk size, and more)
            + self.total_pixel_bytes()
    }

    /// Validate this instance.
    pub fn validate(&self, is_multilayer: bool, long_names: &mut bool, strict: bool) -> UnitResult {

        self.data_window().validate(None)?;
        self.shared_attributes.display_window.validate(None)?;

        if strict {
            if is_multilayer {
                if self.own_attributes.layer_name.is_none() {
                    return Err(missing_attribute("layer name for multi layer file"));
                }
            }

            if self.blocks == BlockDescription::ScanLines && self.line_order == LineOrder::Unspecified {
                return Err(Error::invalid("unspecified line order in scan line images"));
            }

            if self.layer_size == Vec2(0, 0) {
                return Err(Error::invalid("empty data window"));
            }

            if self.shared_attributes.display_window.size == Vec2(0,0) {
                return Err(Error::invalid("empty display window"));
            }

            if !self.shared_attributes.pixel_aspect.is_normal() || self.shared_attributes.pixel_aspect < 1.0e-6 || self.shared_attributes.pixel_aspect > 1.0e6 {
                return Err(Error::invalid("pixel aspect ratio"));
            }

            if self.own_attributes.screen_window_width < 0.0 {
                return Err(Error::invalid("screen window width"));
            }
        }

        let allow_subsampling = !self.deep && self.blocks == BlockDescription::ScanLines;
        self.channels.validate(allow_subsampling, self.data_window(), strict)?;

        for (name, value) in &self.shared_attributes.other {
            attribute::validate(name, value, long_names, allow_subsampling, self.data_window(), strict)?;
        }

        for (name, value) in &self.own_attributes.other {
            attribute::validate(name, value, long_names, allow_subsampling, self.data_window(), strict)?;
        }

        // this is only to check whether someone tampered with our precious values, to avoid writing an invalid file
        if self.chunk_count != compute_chunk_count(self.compression, self.layer_size, self.blocks) {
            return Err(Error::invalid("chunk count attribute")); // TODO this may be an expensive check?
        }

        // check if attribute names appear twice
        if strict {
            for (name, _) in &self.shared_attributes.other {
                if self.own_attributes.other.contains_key(name) {
                    return Err(Error::invalid(format!("duplicate attribute name: `{}`", name)));
                }
            }

            for &reserved in header::standard_names::ALL.iter() {
                let name  = Text::from_bytes_unchecked(SmallVec::from_slice(reserved));
                if self.own_attributes.other.contains_key(&name) || self.shared_attributes.other.contains_key(&name) {
                    return Err(Error::invalid(format!(
                        "attribute name `{}` is reserved and cannot be custom",
                        Text::from_bytes_unchecked(reserved.into())
                    )));
                }
            }
        }

        if self.deep {
            if strict {
                if self.own_attributes.layer_name.is_none() {
                    return Err(missing_attribute("layer name for deep file"));
                }

                if self.max_samples_per_pixel.is_none() {
                    return Err(Error::invalid("missing max samples per pixel attribute for deepdata"));
                }
            }

            match self.deep_data_version {
                Some(1) => {},
                Some(_) => return Err(Error::unsupported("deep data version")),
                None => return Err(missing_attribute("deep data version")),
            }

            if !self.compression.supports_deep_data() {
                return Err(Error::invalid("compression method does not support deep data"));
            }
        }

        Ok(())
    }

    /// Read the headers without validating them.
    pub fn read_all(read: &mut PeekRead<impl Read>, version: &Requirements, pedantic: bool) -> Result<Headers> {
        if !version.is_multilayer() {
            Ok(smallvec![ Header::read(read, version, pedantic)? ])
        }
        else {
            let mut headers = SmallVec::new();

            while !sequence_end::has_come(read)? {
                headers.push(Header::read(read, version, pedantic)?);
            }

            Ok(headers)
        }
    }

    /// Without validation, write the headers to the byte stream.
    pub fn write_all(headers: &[Header], write: &mut impl Write, is_multilayer: bool) -> UnitResult {
        for header in headers {
            header.write(write)?;
        }

        if is_multilayer {
            sequence_end::write(write)?;
        }

        Ok(())
    }

    /// Iterate over all `(name, attribute_value)` pairs in this header that would be written to a file.
    /// The order of attributes is arbitrary and may change in future versions.
    /// Will always contain all strictly required attributes, such as channels, compression, data window, and similar.
    /// Hint: Use `attribute.kind_name()` to obtain the standardized name of the attribute type.
    /// Does not validate the header or attributes.
    // This function is used for writing the attributes to files.
    #[inline]
    pub fn all_named_attributes(&self) -> impl '_ + Iterator<Item=(&TextSlice, AttributeValue)> {
        use std::iter::{once, once_with, empty};
        use crate::meta::header::standard_names::*;
        use AttributeValue::*;

        #[inline] fn optional<'t, T: Clone>(
            name: &'t TextSlice,
            to_attribute: impl Fn(T) -> AttributeValue,
            value: &'t Option<T>
        )
           -> impl Iterator<Item=(&'t TextSlice, AttributeValue)>
        {
            value.as_ref().map(move |value| (name, to_attribute(value.clone()))).into_iter()
        }

        #[inline] fn required<'s, T: Clone>(name: &'s TextSlice, to_attribute: impl Fn(T) -> AttributeValue, value: &'s T)
            -> impl Iterator<Item=(&'s TextSlice, AttributeValue)>
        {
            once((name, to_attribute((*value).clone())))
        }

        // used to type-check local variables. only requried because you cannot do `let i: impl Iterator<> = ...`
        #[inline] fn expect_is_iter<'s, T: Iterator<Item=(&'s TextSlice, AttributeValue)>>(val: T) -> T { val }

        macro_rules! iter_all {
            ( $( $value:expr ),* ) => {
                empty() $( .chain( $value ) )*
            };
        }

        macro_rules! required_attributes {
            ( $($name: ident : $variant: ident = $value: expr),* ) => {
                expect_is_iter(iter_all!(
                    $( required($name, $variant, $value) ),*
                ))
            };
        }

        macro_rules! optional_attributes {
            ( $($name: ident : $variant: ident = $value: expr),* ) => {
                expect_is_iter(iter_all!(
                    $( optional($name, $variant, $value) ),*
                ))
            };
        }

        #[inline] fn usize_as_i32(value: usize) -> AttributeValue {
            I32(i32::try_from(value).expect("usize exceeds i32 range"))
        }


        let block_type_and_tiles = expect_is_iter(once_with(move ||{
            let (block_type, tiles) = match self.blocks {
                BlockDescription::ScanLines => (attribute::BlockType::ScanLine, None),
                BlockDescription::Tiles(tiles) => (attribute::BlockType::Tile, Some(tiles))
            };

            once((BLOCK_TYPE, BlockType(block_type)))
                .chain(tiles.map(|tiles| (TILES, TileDescription(tiles))))
        }).flatten());

        let data_window = expect_is_iter(once_with(move ||{
            (DATA_WINDOW, IntegerBounds(self.data_window()))
        }));

        // dwa writes compression parameters as attribute.
        let dwa_compr_level = expect_is_iter(
            once_with(move ||{
                match self.compression {
                    attribute::Compression::DWAA(Some(level)) |
                    attribute::Compression::DWAB(Some(level)) =>
                        Some((DWA_COMPRESSION_LEVEL, F32(level))),

                    _ => None
                }
            }).flatten()
        );

        let opt_core_attrs = optional_attributes!(
            DEEP_DATA_VERSION: I32 = &self.deep_data_version,
            MAX_SAMPLES: usize_as_i32 = &self.max_samples_per_pixel
        ).chain(block_type_and_tiles).chain(dwa_compr_level);

        let req_core_attrs = required_attributes!(
            // chunks is not actually required, but always computed in this library anyways
            CHUNKS: usize_as_i32 = &self.chunk_count,

            CHANNELS: ChannelList = &self.channels,
            COMPRESSION: Compression = &self.compression,
            LINE_ORDER: LineOrder = &self.line_order,

            DISPLAY_WINDOW: IntegerBounds = &self.shared_attributes.display_window,
            PIXEL_ASPECT: F32 = &self.shared_attributes.pixel_aspect,

            WINDOW_CENTER: FloatVec2 = &self.own_attributes.screen_window_center,
            WINDOW_WIDTH: F32 = &self.own_attributes.screen_window_width
        ).chain(data_window);

        let opt_attr = optional_attributes!(
            NAME: Text = &self.own_attributes.layer_name,
            WHITE_LUMINANCE: F32 = &self.own_attributes.white_luminance,
            ADOPTED_NEUTRAL: FloatVec2 = &self.own_attributes.adopted_neutral,
            RENDERING_TRANSFORM: Text = &self.own_attributes.rendering_transform_name,
            LOOK_MOD_TRANSFORM: Text = &self.own_attributes.look_modification_transform_name,
            X_DENSITY: F32 = &self.own_attributes.horizontal_density,
            OWNER: Text = &self.own_attributes.owner,
            COMMENTS: Text = &self.own_attributes.comments,
            CAPTURE_DATE: Text = &self.own_attributes.capture_date,
            UTC_OFFSET: F32 = &self.own_attributes.utc_offset,
            LONGITUDE: F32 = &self.own_attributes.longitude,
            LATITUDE: F32 = &self.own_attributes.latitude,
            ALTITUDE: F32 = &self.own_attributes.altitude,
            FOCUS: F32 = &self.own_attributes.focus,
            EXPOSURE_TIME: F32 = &self.own_attributes.exposure,
            APERTURE: F32 = &self.own_attributes.aperture,
            ISO_SPEED: F32 = &self.own_attributes.iso_speed,
            ENVIRONMENT_MAP: EnvironmentMap = &self.own_attributes.environment_map,
            KEY_CODE: KeyCode = &self.own_attributes.film_key_code,
            TIME_CODE: TimeCode = &self.shared_attributes.time_code,
            WRAP_MODES: Text = &self.own_attributes.wrap_mode_name,
            FRAMES_PER_SECOND: Rational = &self.own_attributes.frames_per_second,
            MULTI_VIEW: TextVector = &self.own_attributes.multi_view_names,
            WORLD_TO_CAMERA: Matrix4x4 = &self.own_attributes.world_to_camera,
            WORLD_TO_NDC: Matrix4x4 = &self.own_attributes.world_to_normalized_device,
            DEEP_IMAGE_STATE: Rational = &self.own_attributes.deep_image_state,
            ORIGINAL_DATA_WINDOW: IntegerBounds = &self.own_attributes.original_data_window,
            CHROMATICITIES: Chromaticities = &self.shared_attributes.chromaticities,
            PREVIEW: Preview = &self.own_attributes.preview,
            VIEW: Text = &self.own_attributes.view_name,
            NEAR: F32 = &self.own_attributes.near_clip_plane,
            FAR: F32 = &self.own_attributes.far_clip_plane,
            FOV_X: F32 = &self.own_attributes.horizontal_field_of_view,
            FOV_Y: F32 = &self.own_attributes.vertical_field_of_view,
            SOFTWARE: Text = &self.own_attributes.software_name
        );

        let other = self.own_attributes.other.iter()
            .chain(self.shared_attributes.other.iter())
            .map(|(name, val)| (name.as_slice(), val.clone())); // TODO no clone

        req_core_attrs
            .chain(opt_core_attrs)
            .chain(opt_attr)
            .chain(other)
    }

    /// Read the value without validating.
    pub fn read(read: &mut PeekRead<impl Read>, requirements: &Requirements, pedantic: bool) -> Result<Self> {
        let max_string_len = if requirements.has_long_names { 256 } else { 32 }; // TODO DRY this information

        // these required attributes will be filled when encountered while parsing
        let mut tiles = None;
        let mut block_type = None;
        let mut version = None;
        let mut chunk_count = None;
        let mut max_samples_per_pixel = None;
        let mut channels = None;
        let mut compression = None;
        let mut data_window = None;
        let mut display_window = None;
        let mut line_order = None;
        let mut dwa_compression_level = None;

        let mut layer_attributes = LayerAttributes::default();
        let mut image_attributes = ImageAttributes::new(IntegerBounds::zero());

        // read each attribute in this header
        while !sequence_end::has_come(read)? {
            let (attribute_name, value) = attribute::read(read, max_string_len)?;

            // if the attribute value itself is ok, record it
            match value {
                Ok(value) => {
                    use crate::meta::header::standard_names as name;
                    use crate::meta::attribute::AttributeValue::*;

                    // if the attribute is a required attribute, set the corresponding variable directly.
                    // otherwise, add the attribute to the vector of custom attributes

                    // the following attributes will only be set if the type matches the commonly used type for that attribute
                    match (attribute_name.as_slice(), value) {
                        (name::BLOCK_TYPE, Text(value)) => block_type = Some(attribute::BlockType::parse(value)?),
                        (name::TILES, TileDescription(value)) => tiles = Some(value),
                        (name::CHANNELS, ChannelList(value)) => channels = Some(value),
                        (name::COMPRESSION, Compression(value)) => compression = Some(value),
                        (name::DATA_WINDOW, IntegerBounds(value)) => data_window = Some(value),
                        (name::DISPLAY_WINDOW, IntegerBounds(value)) => display_window = Some(value),
                        (name::LINE_ORDER, LineOrder(value)) => line_order = Some(value),
                        (name::DEEP_DATA_VERSION, I32(value)) => version = Some(value),

                        (name::MAX_SAMPLES, I32(value)) => max_samples_per_pixel = Some(
                            i32_to_usize(value, "max sample count")?
                        ),

                        (name::CHUNKS, I32(value)) => chunk_count = Some(
                            i32_to_usize(value, "chunk count")?
                        ),

                        (name::NAME, Text(value)) => layer_attributes.layer_name = Some(value),
                        (name::WINDOW_CENTER, FloatVec2(value)) => layer_attributes.screen_window_center = value,
                        (name::WINDOW_WIDTH, F32(value)) => layer_attributes.screen_window_width = value,

                        (name::WHITE_LUMINANCE, F32(value)) => layer_attributes.white_luminance = Some(value),
                        (name::ADOPTED_NEUTRAL, FloatVec2(value)) => layer_attributes.adopted_neutral = Some(value),
                        (name::RENDERING_TRANSFORM, Text(value)) => layer_attributes.rendering_transform_name = Some(value),
                        (name::LOOK_MOD_TRANSFORM, Text(value)) => layer_attributes.look_modification_transform_name = Some(value),
                        (name::X_DENSITY, F32(value)) => layer_attributes.horizontal_density = Some(value),

                        (name::OWNER, Text(value)) => layer_attributes.owner = Some(value),
                        (name::COMMENTS, Text(value)) => layer_attributes.comments = Some(value),
                        (name::CAPTURE_DATE, Text(value)) => layer_attributes.capture_date = Some(value),
                        (name::UTC_OFFSET, F32(value)) => layer_attributes.utc_offset = Some(value),
                        (name::LONGITUDE, F32(value)) => layer_attributes.longitude = Some(value),
                        (name::LATITUDE, F32(value)) => layer_attributes.latitude = Some(value),
                        (name::ALTITUDE, F32(value)) => layer_attributes.altitude = Some(value),
                        (name::FOCUS, F32(value)) => layer_attributes.focus = Some(value),
                        (name::EXPOSURE_TIME, F32(value)) => layer_attributes.exposure = Some(value),
                        (name::APERTURE, F32(value)) => layer_attributes.aperture = Some(value),
                        (name::ISO_SPEED, F32(value)) => layer_attributes.iso_speed = Some(value),
                        (name::ENVIRONMENT_MAP, EnvironmentMap(value)) => layer_attributes.environment_map = Some(value),
                        (name::KEY_CODE, KeyCode(value)) => layer_attributes.film_key_code = Some(value),
                        (name::WRAP_MODES, Text(value)) => layer_attributes.wrap_mode_name = Some(value),
                        (name::FRAMES_PER_SECOND, Rational(value)) => layer_attributes.frames_per_second = Some(value),
                        (name::MULTI_VIEW, TextVector(value)) => layer_attributes.multi_view_names = Some(value),
                        (name::WORLD_TO_CAMERA, Matrix4x4(value)) => layer_attributes.world_to_camera = Some(value),
                        (name::WORLD_TO_NDC, Matrix4x4(value)) => layer_attributes.world_to_normalized_device = Some(value),
                        (name::DEEP_IMAGE_STATE, Rational(value)) => layer_attributes.deep_image_state = Some(value),
                        (name::ORIGINAL_DATA_WINDOW, IntegerBounds(value)) => layer_attributes.original_data_window = Some(value),
                        (name::DWA_COMPRESSION_LEVEL, F32(value)) => dwa_compression_level = Some(value),
                        (name::PREVIEW, Preview(value)) => layer_attributes.preview = Some(value),
                        (name::VIEW, Text(value)) => layer_attributes.view_name = Some(value),

                        (name::NEAR, F32(value)) => layer_attributes.near_clip_plane = Some(value),
                        (name::FAR, F32(value)) => layer_attributes.far_clip_plane = Some(value),
                        (name::FOV_X, F32(value)) => layer_attributes.horizontal_field_of_view = Some(value),
                        (name::FOV_Y, F32(value)) => layer_attributes.vertical_field_of_view = Some(value),
                        (name::SOFTWARE, Text(value)) => layer_attributes.software_name = Some(value),

                        (name::PIXEL_ASPECT, F32(value)) => image_attributes.pixel_aspect = value,
                        (name::TIME_CODE, TimeCode(value)) => image_attributes.time_code = Some(value),
                        (name::CHROMATICITIES, Chromaticities(value)) => image_attributes.chromaticities = Some(value),

                        // insert unknown attributes of these types into image attributes,
                        // as these must be the same for all headers
                        (_, value @ Chromaticities(_)) |
                        (_, value @ TimeCode(_)) => {
                            image_attributes.other.insert(attribute_name, value);
                        },

                        // insert unknown attributes into layer attributes
                        (_, value) => {
                            layer_attributes.other.insert(attribute_name, value);
                        },

                    }
                },

                // in case the attribute value itself is not ok, but the rest of the image is
                // only abort reading the image if desired
                Err(error) => {
                    if pedantic { return Err(error); }
                }
            }
        }

        // construct compression with parameters from properties
        let compression = match (dwa_compression_level, compression) {
            (Some(level), Some(Compression::DWAA(_))) => Some(Compression::DWAA(Some(level))),
            (Some(level), Some(Compression::DWAB(_))) => Some(Compression::DWAB(Some(level))),
            (_, other) => other,
            // FIXME dwa compression level gets lost if any other compression is used later in the process
        };

        let compression = compression.ok_or(missing_attribute("compression"))?;
        image_attributes.display_window = display_window.ok_or(missing_attribute("display window"))?;

        let data_window = data_window.ok_or(missing_attribute("data window"))?;
        data_window.validate(None)?; // validate now to avoid errors when computing the chunk_count
        layer_attributes.layer_position = data_window.position;


        // validate now to avoid errors when computing the chunk_count
        if let Some(tiles) = tiles { tiles.validate()?; }
        let blocks = match block_type {
            None if requirements.is_single_layer_and_tiled => {
                BlockDescription::Tiles(tiles.ok_or(missing_attribute("tiles"))?)
            },
            Some(BlockType::Tile) | Some(BlockType::DeepTile) => {
                BlockDescription::Tiles(tiles.ok_or(missing_attribute("tiles"))?)
            },

            _ => BlockDescription::ScanLines,
        };

        let computed_chunk_count = compute_chunk_count(compression, data_window.size, blocks);
        if chunk_count.is_some() && pedantic && chunk_count != Some(computed_chunk_count) {
            return Err(Error::invalid("chunk count not matching data size"));
        }

        let header = Header {
            compression,

            // always compute ourselves, because we cannot trust anyone out there 😱
            chunk_count: computed_chunk_count,

            layer_size: data_window.size,

            shared_attributes: image_attributes,
            own_attributes: layer_attributes,

            channels: channels.ok_or(missing_attribute("channels"))?,
            line_order: line_order.unwrap_or(LineOrder::Unspecified),

            blocks,
            max_samples_per_pixel,
            deep_data_version: version,
            deep: block_type == Some(BlockType::DeepScanLine) || block_type == Some(BlockType::DeepTile),
        };

        Ok(header)
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write(&self, write: &mut impl Write) -> UnitResult {
        for (name, value) in self.all_named_attributes() {
            attribute::write(name, &value, write)?;
        }

        sequence_end::write(write)?;
        Ok(())
    }

    /// The rectangle describing the bounding box of this layer
    /// within the infinite global 2D space of the file.
    pub fn data_window(&self) -> IntegerBounds {
        IntegerBounds::new(self.own_attributes.layer_position, self.layer_size)
    }
}



/// Collection of required attribute names.
pub mod standard_names {
    macro_rules! define_required_attribute_names {
        ( $($name: ident  :  $value: expr),* ) => {

            /// A list containing all reserved names.
            pub const ALL: &'static [&'static [u8]] = &[
                $( $value ),*
            ];

            $(
                /// The byte-string name of this required attribute as it appears in an exr file.
                pub const $name: &'static [u8] = $value;
            )*
        };
    }

    define_required_attribute_names! {
        TILES: b"tiles",
        NAME: b"name",
        BLOCK_TYPE: b"type",
        DEEP_DATA_VERSION: b"version",
        CHUNKS: b"chunkCount",
        MAX_SAMPLES: b"maxSamplesPerPixel",
        CHANNELS: b"channels",
        COMPRESSION: b"compression",
        DATA_WINDOW: b"dataWindow",
        DISPLAY_WINDOW: b"displayWindow",
        LINE_ORDER: b"lineOrder",
        PIXEL_ASPECT: b"pixelAspectRatio",
        WINDOW_CENTER: b"screenWindowCenter",
        WINDOW_WIDTH: b"screenWindowWidth",
        WHITE_LUMINANCE: b"whiteLuminance",
        ADOPTED_NEUTRAL: b"adoptedNeutral",
        RENDERING_TRANSFORM: b"renderingTransform",
        LOOK_MOD_TRANSFORM: b"lookModTransform",
        X_DENSITY: b"xDensity",
        OWNER: b"owner",
        COMMENTS: b"comments",
        CAPTURE_DATE: b"capDate",
        UTC_OFFSET: b"utcOffset",
        LONGITUDE: b"longitude",
        LATITUDE: b"latitude",
        ALTITUDE: b"altitude",
        FOCUS: b"focus",
        EXPOSURE_TIME: b"expTime",
        APERTURE: b"aperture",
        ISO_SPEED: b"isoSpeed",
        ENVIRONMENT_MAP: b"envmap",
        KEY_CODE: b"keyCode",
        TIME_CODE: b"timeCode",
        WRAP_MODES: b"wrapmodes",
        FRAMES_PER_SECOND: b"framesPerSecond",
        MULTI_VIEW: b"multiView",
        WORLD_TO_CAMERA: b"worldToCamera",
        WORLD_TO_NDC: b"worldToNDC",
        DEEP_IMAGE_STATE: b"deepImageState",
        ORIGINAL_DATA_WINDOW: b"originalDataWindow",
        DWA_COMPRESSION_LEVEL: b"dwaCompressionLevel",
        PREVIEW: b"preview",
        VIEW: b"view",
        CHROMATICITIES: b"chromaticities",
        NEAR: b"near",
        FAR: b"far",
        FOV_X: b"fieldOfViewHorizontal",
        FOV_Y: b"fieldOfViewVertical",
        SOFTWARE: b"software"
    }
}


impl Default for LayerAttributes {
    fn default() -> Self {
        Self {
            layer_position: Vec2(0, 0),
            screen_window_center: Vec2(0.0, 0.0),
            screen_window_width: 1.0,
            layer_name: None,
            white_luminance: None,
            adopted_neutral: None,
            rendering_transform_name: None,
            look_modification_transform_name: None,
            horizontal_density: None,
            owner: None,
            comments: None,
            capture_date: None,
            utc_offset: None,
            longitude: None,
            latitude: None,
            altitude: None,
            focus: None,
            exposure: None,
            aperture: None,
            iso_speed: None,
            environment_map: None,
            film_key_code: None,
            wrap_mode_name: None,
            frames_per_second: None,
            multi_view_names: None,
            world_to_camera: None,
            world_to_normalized_device: None,
            deep_image_state: None,
            original_data_window: None,
            preview: None,
            view_name: None,
            software_name: None,
            near_clip_plane: None,
            far_clip_plane: None,
            horizontal_field_of_view: None,
            vertical_field_of_view: None,
            other: Default::default()
        }
    }
}

impl std::fmt::Debug for LayerAttributes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let default_self = Self::default();

        let mut debug = formatter.debug_struct("LayerAttributes (default values omitted)");

        // always debug the following field
        debug.field("name", &self.layer_name);

        macro_rules! debug_non_default_fields {
            ( $( $name: ident ),* ) => { $(

                if self.$name != default_self.$name {
                    debug.field(stringify!($name), &self.$name);
                }

            )* };
        }

        // only debug these fields if they are not the default value
        debug_non_default_fields! {
            screen_window_center, screen_window_width,
            white_luminance, adopted_neutral, horizontal_density,
            rendering_transform_name, look_modification_transform_name,
            owner, comments,
            capture_date, utc_offset,
            longitude, latitude, altitude,
            focus, exposure, aperture, iso_speed,
            environment_map, film_key_code, wrap_mode_name,
            frames_per_second, multi_view_names,
            world_to_camera, world_to_normalized_device,
            deep_image_state, original_data_window,
            preview, view_name,
            vertical_field_of_view, horizontal_field_of_view,
            near_clip_plane, far_clip_plane, software_name
        }

        for (name, value) in &self.other {
            debug.field(&format!("\"{}\"", name), value);
        }

        // debug.finish_non_exhaustive() TODO
        debug.finish()
    }
}
