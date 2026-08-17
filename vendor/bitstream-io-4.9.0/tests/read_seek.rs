use bitstream_io::{BigEndian, BitRead, BitReader, Endianness, LittleEndian};
use std::io::{self, Cursor, SeekFrom};

#[test]
fn test_reader_pos_be() -> io::Result<()> {
    test_reader_pos::<BigEndian>()
}

#[test]
fn test_reader_pos_le() -> io::Result<()> {
    test_reader_pos::<LittleEndian>()
}

fn test_reader_pos<E: Endianness>() -> io::Result<()> {
    let actual_data: [u8; 7] = [
        0b1000_1110,
        0b1000_1110,
        0b1000_1110,
        0b1000_1110,
        0b1000_1110,
        0b1000_1110,
        0b1000_1110,
    ];
    let mut r: BitReader<_, E> = BitReader::new(Cursor::new(&actual_data));

    assert_eq!(r.position_in_bits()?, 0);
    r.read_bit()?;
    r.read_bit()?;
    assert_eq!(r.position_in_bits()?, 2);
    let _: u32 = r.read_var(5)?;
    assert_eq!(r.position_in_bits()?, 7);
    let _: u32 = r.read_var(4)?;
    assert_eq!(r.position_in_bits()?, 11);
    let mut buf = [0u8; 2];
    r.read_bytes(&mut buf)?;
    assert_eq!(r.position_in_bits()?, 27);
    r.read_bit()?;
    r.read_bit()?;
    r.read_bit()?;
    r.read_bit()?;
    r.read_bit()?;
    r.read_bit()?;
    r.read_bit()?;
    let _: i32 = r.read_signed_var(9)?;
    assert_eq!(r.position_in_bits()?, 43);
    let _: i32 = r.read_signed_var(5)?;
    assert_eq!(r.position_in_bits()?, 48);

    Ok(())
}

#[test]
pub fn test_reader_seek_start() -> io::Result<()> {
    let actual_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];
    let mut r = BitReader::endian(Cursor::new(&actual_data), BigEndian);

    r.seek_bits(SeekFrom::Start(0))?;
    assert_eq!(r.position_in_bits()?, 0);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, false);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, false);
    assert_eq!(r.read_bit()?, false);
    assert_eq!(r.read_bit()?, false);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.position_in_bits()?, 8);

    r.seek_bits(SeekFrom::Start(2))?;
    assert_eq!(r.position_in_bits()?, 2);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, false);
    assert_eq!(r.read_bit()?, false);
    assert_eq!(r.read_bit()?, false);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.position_in_bits()?, 8);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.position_in_bits()?, 10);

    r.seek_bits(SeekFrom::Start(7))?;
    assert_eq!(r.position_in_bits()?, 7);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, false);

    Ok(())
}

#[test]
pub fn test_reader_seek_current() -> io::Result<()> {
    let actual_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];
    let mut r = BitReader::endian(Cursor::new(&actual_data), BigEndian);

    r.seek_bits(SeekFrom::Current(2))?;
    assert_eq!(r.position_in_bits()?, 2);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, false);
    assert_eq!(r.read_bit()?, false);
    let _: i32 = r.read_signed_var(11)?;
    assert_eq!(r.position_in_bits()?, 17);

    r.seek_bits(SeekFrom::Current(-3))?;
    assert_eq!(r.position_in_bits()?, 14);
    r.skip(10)?;
    assert_eq!(r.position_in_bits()?, 24);
    r.seek_bits(SeekFrom::Current(0))?;
    assert_eq!(r.position_in_bits()?, 24);

    Ok(())
}

#[test]
pub fn test_reader_seek_end() -> io::Result<()> {
    let actual_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];
    let mut r = BitReader::endian(Cursor::new(&actual_data), BigEndian);

    r.seek_bits(SeekFrom::End(7))?;
    assert_eq!(r.position_in_bits()?, 25);
    assert_eq!(r.read_bit()?, true);
    assert_eq!(r.read_bit()?, false);
    assert_eq!(r.read_bit()?, false);
    assert_eq!(r.read_bit()?, false);
    assert_eq!(r.position_in_bits()?, 29);
    r.seek_bits(SeekFrom::End(0))?;
    assert_eq!(r.position_in_bits()?, 32);

    Ok(())
}
