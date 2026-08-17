/// Generate a random frame mask.
#[inline]
pub fn generate_mask() -> [u8; 4] {
    rand::random()
}

/// Mask/unmask a frame.
#[inline]
pub fn apply_mask(buf: &mut [u8], mask: [u8; 4]) {
    apply_mask_fast32(buf, mask);
}

/// A safe unoptimized mask application.
#[inline]
fn apply_mask_fallback(buf: &mut [u8], mask: [u8; 4]) {
    for (i, byte) in buf.iter_mut().enumerate() {
        *byte ^= mask[i & 3];
    }
}

/// Faster version of `apply_mask()` which operates on 4-byte blocks.
#[inline]
pub fn apply_mask_fast32(buf: &mut [u8], mask: [u8; 4]) {
    let mask_u32 = u32::from_ne_bytes(mask);

    let (prefix, words, suffix) = unsafe { buf.align_to_mut::<u32>() };
    apply_mask_fallback(prefix, mask);
    let head = prefix.len() & 3;
    let mask_u32 = if head > 0 {
        if cfg!(target_endian = "big") {
            mask_u32.rotate_left(8 * head as u32)
        } else {
            mask_u32.rotate_right(8 * head as u32)
        }
    } else {
        mask_u32
    };
    for word in words.iter_mut() {
        *word ^= mask_u32;
    }
    apply_mask_fallback(suffix, mask_u32.to_ne_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_mask() {
        let mask = [0x6d, 0xb6, 0xb2, 0x80];
        let unmasked = [
            0xf3, 0x00, 0x01, 0x02, 0x03, 0x80, 0x81, 0x82, 0xff, 0xfe, 0x00, 0x17, 0x74, 0xf9,
            0x12, 0x03,
        ];

        for data_len in 0..=unmasked.len() {
            let unmasked = &unmasked[0..data_len];
            // Check masking with different alignment.
            for off in 0..=3 {
                if unmasked.len() < off {
                    continue;
                }
                let mut masked = unmasked.to_vec();
                apply_mask_fallback(&mut masked[off..], mask);

                let mut masked_fast = unmasked.to_vec();
                apply_mask_fast32(&mut masked_fast[off..], mask);

                assert_eq!(masked, masked_fast);
            }
        }
    }
}
