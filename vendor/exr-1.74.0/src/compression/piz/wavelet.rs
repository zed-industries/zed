
//! Wavelet encoding and decoding.
// see https://github.com/AcademySoftwareFoundation/openexr/blob/8cd1b9210855fa4f6923c1b94df8a86166be19b1/OpenEXR/IlmImf/ImfWav.cpp

use crate::error::IoResult;
use crate::math::Vec2;

#[allow(unused)]
#[inline]
pub fn encode(buffer: &mut [u16], count: Vec2<usize>, size: Vec2<usize>, max_value: u16) -> IoResult<()> {
    if is_14_bit(max_value) { encode_14_or_16_bit(buffer, count, size, true) }
    else { encode_14_or_16_bit(buffer, count, size, false) }
}

#[allow(unused)]
#[inline]
pub fn encode_14_or_16_bit(
    buffer: &mut [u16],
    Vec2(count_x, count_y): Vec2<usize>,
    Vec2(offset_x, offset_y): Vec2<usize>,
    is_14_bit: bool // true if maximum buffer[i] value < (1 << 14)
) -> IoResult<()>
{
    let count = count_x.min(count_y);
    let encode = if is_14_bit { encode_14bit } else { encode_16bit }; // assume inlining and constant propagation

    let mut p: usize = 1; // TODO i32?
    let mut p2: usize = 2; // TODO what is p??

    while p2 <= count {

        let mut position_y = 0;
        let end_y = 0 + offset_y * (count_y - p2);
        let (offset1_x, offset1_y) = (offset_x * p, offset_y * p);
        let (offset2_x, offset2_y) = (offset_x * p2, offset_y * p2);

        // y-loop
        while position_y <= end_y { // TODO: for py in (index..ey).nth(offset_2.0)

            let mut position_x = position_y;
            let end_x = position_x + offset_x * (count_x - p2);

            // x-loop
            while position_x <= end_x {
                let pos_right = position_x + offset1_x;
                let pos_top = position_x + offset1_y;
                let pos_top_right = pos_top + offset1_x;

                assert!(position_x < buffer.len());
                assert!(pos_right < buffer.len());
                assert!(pos_top < buffer.len());
                assert!(pos_top_right < buffer.len());

                if is_14_bit {
                    debug_assert!(self::is_14_bit(buffer[position_x]));
                    debug_assert!(self::is_14_bit(buffer[pos_right]));
                }

                let (center, right) = encode(buffer[position_x], buffer[pos_right]);
                let (top, top_right) = encode(buffer[pos_top], buffer[pos_top_right]);

                let (center, top) = encode(center, top);
                let (right, top_right) = encode(right, top_right);

                buffer[position_x] = center; // TODO rustify
                buffer[pos_top] = top;
                buffer[pos_right] = right;
                buffer[pos_top_right] = top_right;

                position_x += offset2_x;
            }

            // encode remaining odd pixel column
            if count_x & p != 0 {
                let pos_top = position_x + offset1_y;
                let (center, top) = encode(buffer[position_x], buffer[pos_top]);

                buffer[position_x] = center;
                buffer[pos_top] = top;
            }

            position_y += offset2_y;
        }

        // encode possibly remaining odd row
        if count_y & p != 0 {
            let mut position_x = position_y;
            let end_x = position_y + offset_x * (count_x - p2);

            while position_x <= end_x {
                let pos_right = position_x + offset1_x;
                let (center, right) = encode(buffer[position_x], buffer[pos_right]);

                buffer[pos_right] = right;
                buffer[position_x] = center;

                position_x += offset2_x;
            }
        }

        p = p2;
        p2 <<= 1;
    }

    Ok(())
}

#[inline]
pub fn decode(buffer: &mut [u16], count: Vec2<usize>, size: Vec2<usize>, max_value: u16) -> IoResult<()> {
    if is_14_bit(max_value) { decode_14_or_16_bit(buffer, count, size, true) }
    else { decode_14_or_16_bit(buffer, count, size, false) }
}

#[inline]
pub fn decode_14_or_16_bit(
    buffer: &mut [u16],
    Vec2(count_x, count_y): Vec2<usize>,
    Vec2(offset_x, offset_y): Vec2<usize>,
    is_14_bit: bool // true if maximum buffer[i] value < (1 << 14)
) -> IoResult<()>
{
    let count = count_x.min(count_y);
    let decode = if is_14_bit { decode_14bit } else { decode_16bit }; // assume inlining and constant propagation

    let mut p: usize = 1; // TODO i32?
    let mut p2: usize; // TODO i32?

    // search max level
    while p <= count {
        p <<= 1;
    }

    p >>= 1;
    p2 = p;
    p >>= 1;

    while p >= 1 {

        let mut position_y = 0;
        let end_y = 0 + offset_y * (count_y - p2);

        let (offset1_x, offset1_y) = (offset_x * p, offset_y * p);
        let (offset2_x, offset2_y) = (offset_x * p2, offset_y * p2);

        debug_assert_ne!(offset_x, 0, "offset should not be zero");
        debug_assert_ne!(offset_y, 0, "offset should not be zero");

        while position_y <= end_y {
            let mut position_x = position_y;
            let end_x = position_x + offset_x * (count_x - p2);

            while position_x <= end_x {
                let pos_right = position_x + offset1_x;
                let pos_top = position_x + offset1_y;
                let pos_top_right = pos_top + offset1_x;

                assert!(position_x < buffer.len());
                assert!(pos_right < buffer.len());
                assert!(pos_top < buffer.len());
                assert!(pos_top_right < buffer.len());

                let (center, top) = decode(buffer[position_x], buffer[pos_top]);
                let (right, top_right) = decode(buffer[pos_right], buffer[pos_top_right]);

                let (center, right) = decode(center, right);
                let (top, top_right) = decode(top, top_right);

                buffer[position_x] = center; // TODO rustify
                buffer[pos_top] = top;
                buffer[pos_right] = right;
                buffer[pos_top_right] = top_right;

                position_x += offset2_x;
            }

            // decode last odd remaining x value
            if count_x & p != 0 {
                let pos_top = position_x + offset1_y;
                let (center, top) = decode(buffer[position_x], buffer[pos_top]);

                buffer[position_x] = center;
                buffer[pos_top] = top;
            }

            position_y += offset2_y;
        }

        // decode remaining odd row
        if count_y & p != 0 {
            let mut position_x = position_y;
            let end_x = position_x + offset_x * (count_x - p2);

            while position_x <= end_x {
                let pos_right = position_x + offset1_x;
                let (center, right) = decode(buffer[position_x], buffer[pos_right]);

                buffer[position_x] = center;
                buffer[pos_right] = right;

                position_x += offset2_x;
            }
        }

        p2 = p;
        p >>= 1;
    }

    Ok(())
}

#[inline]
fn is_14_bit(value: u16) -> bool {
    value < (1 << 14)
}

/// Untransformed data values should be less than (1 << 14).
#[inline]
#[allow(unused)]
fn encode_14bit(a: u16, b: u16) -> (u16, u16) {
    let (a, b) = (a as i16, b as i16);

    let m = (a + b) >> 1;
    let d = a - b;

    (m as u16, d as u16) // TODO explicitly wrap?
}

#[inline]
#[allow(unused)]
fn decode_14bit(l: u16, h: u16) -> (u16, u16) {
    let (l, h) = (l as i16, h as i16);

    let hi = h as i32;
    let ai = l as i32 + (hi & 1) + (hi >> 1);

    let a = ai as i16; // TODO explicitly wrap?
    let b = (ai - hi) as i16; // TODO explicitly wrap?

    (a as u16, b as u16) // TODO explicitly wrap?
}


const BIT_COUNT: i32 = 16;
const OFFSET: i32 = 1 << (BIT_COUNT - 1);
const MOD_MASK: i32 = (1 << BIT_COUNT) - 1;

#[inline]
fn encode_16bit(a: u16, b: u16) -> (u16, u16) {
    let (a, b) = (a as i32, b as i32);

    let a_offset = (a + OFFSET) & MOD_MASK;
    let mut m = (a_offset + b) >> 1;
    let d = a_offset - b;

    if d < 0 { m = (m + OFFSET) & MOD_MASK; }
    let d = d & MOD_MASK;

    (m as u16, d as u16) // TODO explicitly wrap?
}

#[inline]
fn decode_16bit(l: u16, h: u16) -> (u16, u16) {
    let (m, d) = (l as i32, h as i32);

    let b = (m - (d >> 1)) & MOD_MASK;
    let a = (d + b - OFFSET) & MOD_MASK;

    (a as u16, b as u16) // TODO explicitly wrap?
}



#[cfg(test)]
mod test {
    use crate::math::Vec2;
    use crate::compression::piz::wavelet::is_14_bit;

    #[test]
    fn roundtrip_14_bit_values(){
        let data = [
            (13, 54), (3, 123), (423, 53), (1, 23), (23, 515), (513, 43),
            (16374, 16381), (16284, 3), (2, 1), (0, 0), (0, 4), (3, 0)
        ];

        for &values in &data {
            let (l, h) = super::encode_14bit(values.0, values.1);
            let result = super::decode_14bit(l, h);
            assert_eq!(values, result);
        }
    }

    #[test]
    fn roundtrip_16_bit_values(){
        let data = [
            (13, 54), (3, 123), (423, 53), (1, 23), (23, 515), (513, 43),
            (16385, 56384), (18384, 36384), (2, 1), (0, 0), (0, 4), (3, 0)
        ];

        for &values in &data {
            let (l, h) = super::encode_16bit(values.0, values.1);
            let result = super::decode_16bit(l, h);
            assert_eq!(values, result);
        }
    }

    #[test]
    fn roundtrip_14bit_image(){
        let data: [u16; 6 * 4] = [
            13, 54, 3, 123, 423, 53,
            1, 23, 23, 515, 513, 43,
            16374, 16381, 16284, 3, 2, 1,
            0, 0, 0, 4, 3, 0,
        ];

        let max = *data.iter().max().unwrap();
        debug_assert!(is_14_bit(max));

        let mut transformed = data.clone();

        super::encode(&mut transformed, Vec2(6, 4), Vec2(1,6), max).unwrap();
        super::decode(&mut transformed, Vec2(6, 4), Vec2(1,6), max).unwrap();

        assert_eq!(data, transformed);
    }

    #[test]
    fn roundtrip_16bit_image(){
        let data: [u16; 6 * 4] = [
            13, 54, 3, 123, 423, 53,
            1, 23, 23, 515, 513, 43,
            16385, 56384, 18384, 36384, 2, 1,
            0, 0, 0, 4, 3, 0,
        ];

        let max = *data.iter().max().unwrap();
        debug_assert!(!is_14_bit(max));

        let mut transformed = data.clone();

        super::encode(&mut transformed, Vec2(6, 4), Vec2(1,6), max).unwrap();
        super::decode(&mut transformed, Vec2(6, 4), Vec2(1,6), max).unwrap();

        assert_eq!(data, transformed);
    }

    /// inspired by https://github.com/AcademySoftwareFoundation/openexr/blob/master/OpenEXR/IlmImfTest/testWav.cpp
    #[test]
    fn ground_truth(){
        test_size(1, 1);
        test_size(2, 2);
        test_size(32, 32);
        test_size(1024, 16);
        test_size(16, 1024);
        test_size(997, 37);
        test_size(37, 997);
        test_size(1024, 1024);
        test_size(997, 997);

        fn test_size(x: usize, y: usize) {
            let xy = Vec2(x, y);
            roundtrip(noise_14bit(xy), xy);
            roundtrip(noise_16bit(xy), xy);
            roundtrip(solid(xy, 0), xy);
            roundtrip(solid(xy, 1), xy);
            roundtrip(solid(xy, 0xffff), xy);
            roundtrip(solid(xy, 0x3fff), xy);
            roundtrip(solid(xy, 0x3ffe), xy);
            roundtrip(solid(xy, 0x3fff), xy);
            roundtrip(solid(xy, 0xfffe), xy);
            roundtrip(solid(xy, 0xffff), xy);
            roundtrip(verticals(xy, 0xffff), xy);
            roundtrip(verticals(xy, 0x3fff), xy);
            roundtrip(horizontals(xy, 0xffff), xy);
            roundtrip(horizontals(xy, 0x3fff), xy);
            roundtrip(diagonals(xy, 0xffff), xy);
            roundtrip(diagonals(xy, 0x3fff), xy);
        }

        fn roundtrip(data: Vec<u16>, size: Vec2<usize>){
            assert_eq!(data.len(), size.area());

            let max = *data.iter().max().unwrap();
            let offset = Vec2(1, size.0);

            let mut transformed = data.clone();
            super::encode(&mut transformed, size, offset, max).unwrap();
            super::decode(&mut transformed, size, offset, max).unwrap();

            assert_eq!(data, transformed);
        }

        fn noise_14bit(size: Vec2<usize>) -> Vec<u16> {
            (0..size.area()).map(|_| (rand::random::<i32>() & 0x3fff) as u16).collect()
        }

        fn noise_16bit(size: Vec2<usize>) -> Vec<u16> {
            (0..size.area()).map(|_| rand::random::<u16>()).collect()
        }

        fn solid(size: Vec2<usize>, value: u16) -> Vec<u16> {
            vec![value; size.area()]
        }

        fn verticals(size: Vec2<usize>, max_value: u16) -> Vec<u16> {
            std::iter::repeat_with(|| (0 .. size.0).map(|x| if x & 1 != 0 { 0 } else { max_value }))
                .take(size.1).flatten().collect()
        }

        fn horizontals(size: Vec2<usize>, max_value: u16) -> Vec<u16> {
            (0 .. size.1)
                .flat_map(|y| std::iter::repeat(if y & 1 != 0 { 0 } else { max_value }).take(size.0))
                .collect()
        }

        fn diagonals(size: Vec2<usize>, max_value: u16) -> Vec<u16> {
            (0 .. size.1).flat_map(|y| {
                (0 .. size.0).map(move |x| if (x + y) & 1 != 0 { 0 } else { max_value })
            }).collect()
        }

    }
}