extern crate bitstream_io;

// Writing the initial STREAMINFO block to a FLAC file,
// as documented in its
// [specification](https://xiph.org/flac/format.html#stream).

use bitstream_io::{
    BigEndian, BitWrite, BitWriter, ByteWrite, ByteWriter, LittleEndian, ToBitStream,
};
use std::convert::TryInto;
use std::num::NonZero;

#[derive(Debug, PartialEq, Eq)]
struct BlockHeader {
    last_block: bool, // 1 bit
    block_type: u8,   // 7 bits
    block_size: u32,  // 24 bits
}

impl ToBitStream for BlockHeader {
    type Error = std::io::Error;

    fn to_writer<W: BitWrite + ?Sized>(&self, w: &mut W) -> std::io::Result<()> {
        w.write_bit(self.last_block)?;
        w.write::<7, _>(self.block_type)?;
        w.write::<24, _>(self.block_size)
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

impl ToBitStream for Streaminfo {
    type Error = std::io::Error;

    fn to_writer<W: BitWrite + ?Sized>(&self, w: &mut W) -> std::io::Result<()> {
        w.write_from(self.minimum_block_size)?;
        w.write_from(self.maximum_block_size)?;
        w.write::<24, _>(self.minimum_frame_size)?;
        w.write::<24, _>(self.maximum_frame_size)?;
        w.write::<20, _>(self.sample_rate)?;
        w.write::<3, _>(self.channels)?;
        w.write::<5, _>(self.bits_per_sample)?;
        w.write::<36, _>(self.total_samples)?;
        w.write_bytes(&self.md5)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct VorbisComment {
    vendor: String,
    comment: Vec<String>,
}

impl VorbisComment {
    fn len(&self) -> usize {
        4 + self.vendor.len() + 4 + self.comment.iter().map(|c| 4 + c.len()).sum::<usize>()
    }

    fn write<W: std::io::Write>(&self, w: &mut ByteWriter<W, LittleEndian>) -> std::io::Result<()> {
        use std::convert::TryInto;

        fn write_entry<W: std::io::Write>(
            w: &mut ByteWriter<W, LittleEndian>,
            s: &str,
        ) -> std::io::Result<()> {
            w.write::<u32>(s.len().try_into().unwrap())?;
            w.write_bytes(s.as_bytes())
        }

        write_entry(w, &self.vendor)?;
        w.write::<u32>(self.comment.len().try_into().unwrap())?;
        self.comment.iter().try_for_each(|s| write_entry(w, s))
    }
}

fn main() {
    let mut flac: Vec<u8> = Vec::new();

    let mut writer = BitWriter::endian(&mut flac, BigEndian);

    // stream marker
    writer.write_bytes(b"fLaC").unwrap();

    // metadata block header
    writer
        .build(&BlockHeader {
            last_block: false,
            block_type: 0,
            block_size: 34,
        })
        .unwrap();

    // STREAMINFO block
    writer
        .build(&Streaminfo {
            minimum_block_size: 4096,
            maximum_block_size: 4096,
            minimum_frame_size: 1542,
            maximum_frame_size: 8546,
            sample_rate: 44100,
            channels: NonZero::new(2).unwrap(),
            bits_per_sample: NonZero::new(16).unwrap(),
            total_samples: 304844,
            md5: *b"\xFA\xF2\x69\x2F\xFD\xEC\x2D\x5B\x30\x01\x76\xB4\x62\x88\x7D\x92",
        })
        .unwrap();

    let comment = VorbisComment {
        vendor: "reference libFLAC 1.1.4 20070213".to_string(),
        comment: vec![
            "title=2ch 44100  16bit".to_string(),
            "album=Test Album".to_string(),
            "artist=Assorted".to_string(),
            "tracknumber=1".to_string(),
        ],
    };

    // metadata block header
    writer
        .build(&BlockHeader {
            last_block: false,
            block_type: 4,
            block_size: comment.len().try_into().unwrap(),
        })
        .unwrap();

    // VORBIS_COMMENT block (little endian)
    comment
        .write(&mut ByteWriter::new(writer.writer().unwrap()))
        .unwrap();

    assert_eq!(flac, include_bytes!("data/metadata-only.flac"));
}
