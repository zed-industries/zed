use std::io::Read;

use num_rational::Rational32;
use v_frame::{
    frame::Frame,
    pixel::{ChromaSampling, Pixel},
};

use crate::decoder::VideoDetails;

pub fn get_video_details<R: Read>(dec: &y4m::Decoder<R>) -> VideoDetails {
    let width = dec.get_width();
    let height = dec.get_height();
    let color_space = dec.get_colorspace();
    let bit_depth = color_space.get_bit_depth();
    let chroma_sampling = map_y4m_color_space(color_space);
    let framerate = dec.get_framerate();
    let time_base = Rational32::new(framerate.den as i32, framerate.num as i32);

    VideoDetails {
        width,
        height,
        bit_depth,
        chroma_sampling,
        time_base,
    }
}

const fn map_y4m_color_space(color_space: y4m::Colorspace) -> ChromaSampling {
    use y4m::Colorspace::{
        C420jpeg,
        C420mpeg2,
        C420p10,
        C420p12,
        C420paldv,
        C422p10,
        C422p12,
        C444p10,
        C444p12,
        Cmono,
        Cmono12,
        C420,
        C422,
        C444,
    };
    use ChromaSampling::{Cs400, Cs420, Cs422, Cs444};
    match color_space {
        Cmono | Cmono12 => Cs400,
        C420jpeg | C420paldv => Cs420,
        C420mpeg2 => Cs420,
        C420 | C420p10 | C420p12 => Cs420,
        C422 | C422p10 | C422p12 => Cs422,
        C444 | C444p10 | C444p12 => Cs444,
        _ => unimplemented!(),
    }
}

pub fn read_video_frame<R: Read, T: Pixel>(
    dec: &mut y4m::Decoder<R>,
    cfg: &VideoDetails,
) -> anyhow::Result<Frame<T>> {
    const SB_SIZE_LOG2: usize = 6;
    const SB_SIZE: usize = 1 << SB_SIZE_LOG2;
    const SUBPEL_FILTER_SIZE: usize = 8;
    const FRAME_MARGIN: usize = 16 + SUBPEL_FILTER_SIZE;
    const LUMA_PADDING: usize = SB_SIZE + FRAME_MARGIN;

    let bytes = dec.get_bytes_per_sample();
    dec.read_frame()
        .map(|frame| {
            let mut f: Frame<T> =
                Frame::new_with_padding(cfg.width, cfg.height, cfg.chroma_sampling, LUMA_PADDING);

            let (chroma_width, _) = cfg
                .chroma_sampling
                .get_chroma_dimensions(cfg.width, cfg.height);

            f.planes[0].copy_from_raw_u8(frame.get_y_plane(), cfg.width * bytes, bytes);
            f.planes[1].copy_from_raw_u8(frame.get_u_plane(), chroma_width * bytes, bytes);
            f.planes[2].copy_from_raw_u8(frame.get_v_plane(), chroma_width * bytes, bytes);
            f
        })
        .map_err(|e| e.into())
}
