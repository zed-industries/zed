//! Reading the initial STREAMINFO block from a FLAC file,
//! as documented in its
//! [specification](https://xiph.org/flac/format.html#stream).
//!

extern crate bitstream_io;

use bitstream_io::{BigEndian, BitRead, BitReader, FromBitStream};
use std::num::NonZero;

#[derive(Debug, PartialEq, Eq)]
struct BlockHeader {
    last_block: bool, // 1 bit
    block_type: u8,   // 7 bits
    block_size: u32,  // 24 bits
}

impl FromBitStream for BlockHeader {
    type Error = std::io::Error;

    fn from_reader<R: BitRead + ?Sized>(r: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            last_block: r.read_bit()?,
            block_type: r.read::<7, _>()?,
            block_size: r.read::<24, _>()?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Streaminfo {
    minimum_block_size: u16,      // 16 bits
    maximum_block_size: u16,      // 16 bits
    minimum_frame_size: u32,      // 24 bits
    maximum_frame_size: u32,      // 24 bits
    sample_rate: u32,             // 20 bits
    channels: NonZero<u8>,        // 3 bits
    bits_per_sample: NonZero<u8>, // 5 bits
    total_samples: u64,           // 36 bits
    md5: [u8; 16],                // 16 bytes
}

impl FromBitStream for Streaminfo {
    type Error = std::io::Error;

    fn from_reader<R: BitRead + ?Sized>(r: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            minimum_block_size: r.read_to()?,
            maximum_block_size: r.read_to()?,
            minimum_frame_size: r.read::<24, _>()?,
            maximum_frame_size: r.read::<24, _>()?,
            sample_rate: r.read::<20, _>()?,
            channels: r.read::<3, _>()?,
            bits_per_sample: r.read::<5, _>()?,
            total_samples: r.read::<36, _>()?,
            md5: r.read_to()?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct VorbisComment {
    vendor: String,
    comment: Vec<String>,
}

impl FromBitStream for VorbisComment {
    type Error = Box<dyn std::error::Error>;

    fn from_reader<R: BitRead + ?Sized>(r: &mut R) -> Result<Self, Self::Error> {
        use bitstream_io::LE;

        fn read_entry<R: BitRead + ?Sized>(
            r: &mut R,
        ) -> Result<String, Box<dyn std::error::Error>> {
            use std::convert::TryInto;
            let size = r.read_as_to::<LE, u32>()?.try_into()?;
            Ok(String::from_utf8(r.read_to_vec(size)?)?)
        }

        Ok(Self {
            vendor: read_entry(r)?,
            comment: (0..r.read_as_to::<LE, u32>()?)
                .map(|_| read_entry(r))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn main() {
    // test FLAC file data
    let flac = include_bytes!("data/metadata-only.flac");

    let mut reader = BitReader::endian(flac.as_slice(), BigEndian);

    // stream marker
    assert_eq!(&reader.read_to::<[u8; 4]>().unwrap(), b"fLaC");

    // metadata block header
    assert_eq!(
        reader.parse::<BlockHeader>().unwrap(),
        BlockHeader {
            last_block: false,
            block_type: 0,
            block_size: 34
        }
    );

    // STREAMINFO block
    assert_eq!(
        dbg!(reader.parse::<Streaminfo>().unwrap()),
        Streaminfo {
            minimum_block_size: 4096,
            maximum_block_size: 4096,
            minimum_frame_size: 1542,
            maximum_frame_size: 8546,
            sample_rate: 44100,
            channels: NonZero::new(2).unwrap(),
            bits_per_sample: NonZero::new(16).unwrap(),
            total_samples: 304844,
            md5: *b"\xFA\xF2\x69\x2F\xFD\xEC\x2D\x5B\x30\x01\x76\xB4\x62\x88\x7D\x92",
        }
    );

    // metadata block header
    assert_eq!(
        dbg!(reader.parse::<BlockHeader>().unwrap()),
        BlockHeader {
            last_block: false,
            block_type: 4,
            block_size: 122
        }
    );

    // VORBIS_COMMENT block
    assert_eq!(
        dbg!(reader.parse::<VorbisComment>().unwrap()),
        VorbisComment {
            vendor: "reference libFLAC 1.1.4 20070213".to_string(),
            comment: vec![
                "title=2ch 44100  16bit".to_string(),
                "album=Test Album".to_string(),
                "artist=Assorted".to_string(),
                "tracknumber=1".to_string(),
            ],
        }
    );
}
