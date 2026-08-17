#[cfg(feature = "diff")]
use std::{borrow::Cow, mem::size_of};

#[cfg(feature = "diff")]
use v_frame::{
    frame::Frame,
    prelude::{CastFromPrimitive, ChromaSampling, Pixel},
};

#[cfg(feature = "diff")]
pub fn frame_into_u8<T: Pixel>(frame: &Frame<T>, bit_depth: usize) -> Cow<'_, Frame<u8>> {
    if size_of::<T>() == 1 {
        assert_eq!(bit_depth, 8);
        // SAFETY: We know from the size check that this must be a `Frame<u8>`
        Cow::Borrowed(unsafe { &*(frame as *const Frame<T>).cast::<Frame<u8>>() })
    } else if size_of::<T>() == 2 {
        assert!(bit_depth > 8 && bit_depth <= 16);
        let mut u8_frame: Frame<u8> = Frame::new_with_padding(
            frame.planes[0].cfg.width,
            frame.planes[0].cfg.height,
            match frame.planes[1].cfg.xdec + frame.planes[1].cfg.ydec {
                0 => ChromaSampling::Cs444,
                1 => ChromaSampling::Cs422,
                2 => ChromaSampling::Cs420,
                _ => unreachable!(),
            },
            frame.planes[0].cfg.xpad,
        );
        for i in 0..3 {
            let out_plane = &mut u8_frame.planes[i];
            for (i, o) in frame.planes[i]
                .data_origin()
                .iter()
                .zip(out_plane.data_origin_mut().iter_mut())
            {
                *o = (u16::cast_from(*i) >> (bit_depth - 8usize)) as u8;
            }
        }
        Cow::Owned(u8_frame)
    } else {
        unimplemented!("Bit depths greater than 16 are not currently supported");
    }
}
