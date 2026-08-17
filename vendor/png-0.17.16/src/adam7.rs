//! Utility functions related to handling of
//! [the Adam7 algorithm](https://en.wikipedia.org/wiki/Adam7_algorithm).

/// Describes which stage of
/// [the Adam7 algorithm](https://en.wikipedia.org/wiki/Adam7_algorithm)
/// applies to a decoded row.
///
/// See also [Reader.next_interlaced_row](crate::decoder::Reader::next_interlaced_row).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Adam7Info {
    pub(crate) pass: u8,
    pub(crate) line: u32,
    pub(crate) width: u32,
}

impl Adam7Info {
    /// Creates a new `Adam7Info`.  May panic if the arguments are out of range (e.g. if `pass` is
    /// 0 or greater than 8).
    ///
    /// * `pass` corresponds to a pass of the
    ///   [the Adam7 algorithm](https://en.wikipedia.org/wiki/Adam7_algorithm)
    /// * `line` is the number of a line within a pass (starting with 0).  For example,
    ///   in an image of height 8, `line` can be beteween `0..4` in the 7th `pass`
    ///   (those 4 interlaced rows correspond to 2nd, 4th, 6th, and 8th row of the full image).
    /// * `width` describes how many pixels are in an interlaced row.  For example,
    ///   in the 7th `pass`, the `width` is be the same as full image width, but in
    ///   in the 1st `pass`, the `width` is be 1/8th of the image width (rounded up as
    ///   necessary).
    ///
    /// Note that in typical usage, `Adam7Info`s are returned by [Reader.next_interlaced_row]
    /// and there is no need to create them by calling `Adam7Info::new`.  `Adam7Info::new` is
    /// nevertheless exposed as a public API, because it helps to provide self-contained example
    /// usage of [expand_interlaced_row](crate::expand_interlaced_row).
    pub fn new(pass: u8, line: u32, width: u32) -> Self {
        assert!(1 <= pass && pass <= 7);
        assert!(width > 0);
        Self { pass, line, width }
    }
}

/// This iterator iterates over the different passes of an image Adam7 encoded
/// PNG image
/// The pattern is:
///     16462646
///     77777777
///     56565656
///     77777777
///     36463646
///     77777777
///     56565656
///     77777777
///
#[derive(Clone)]
pub(crate) struct Adam7Iterator {
    line: u32,
    lines: u32,
    line_width: u32,
    current_pass: u8,
    width: u32,
    height: u32,
}

impl Adam7Iterator {
    pub fn new(width: u32, height: u32) -> Adam7Iterator {
        let mut this = Adam7Iterator {
            line: 0,
            lines: 0,
            line_width: 0,
            current_pass: 1,
            width,
            height,
        };
        this.init_pass();
        this
    }

    /// Calculates the bounds of the current pass
    fn init_pass(&mut self) {
        let w = f64::from(self.width);
        let h = f64::from(self.height);
        let (line_width, lines) = match self.current_pass {
            1 => (w / 8.0, h / 8.0),
            2 => ((w - 4.0) / 8.0, h / 8.0),
            3 => (w / 4.0, (h - 4.0) / 8.0),
            4 => ((w - 2.0) / 4.0, h / 4.0),
            5 => (w / 2.0, (h - 2.0) / 4.0),
            6 => ((w - 1.0) / 2.0, h / 2.0),
            7 => (w, (h - 1.0) / 2.0),
            _ => unreachable!(),
        };
        self.line_width = line_width.ceil() as u32;
        self.lines = lines.ceil() as u32;
        self.line = 0;
    }
}

/// Iterates over `Adam7Info`s.
impl Iterator for Adam7Iterator {
    type Item = Adam7Info;
    fn next(&mut self) -> Option<Self::Item> {
        if self.line < self.lines && self.line_width > 0 {
            let this_line = self.line;
            self.line += 1;
            Some(Adam7Info {
                pass: self.current_pass,
                line: this_line,
                width: self.line_width,
            })
        } else if self.current_pass < 7 {
            self.current_pass += 1;
            self.init_pass();
            self.next()
        } else {
            None
        }
    }
}

fn subbyte_pixels(scanline: &[u8], bits_pp: usize) -> impl Iterator<Item = u8> + '_ {
    (0..scanline.len() * 8)
        .step_by(bits_pp)
        .map(move |bit_idx| {
            let byte_idx = bit_idx / 8;

            // sub-byte samples start in the high-order bits
            let rem = 8 - bit_idx % 8 - bits_pp;

            match bits_pp {
                // evenly divides bytes
                1 => (scanline[byte_idx] >> rem) & 1,
                2 => (scanline[byte_idx] >> rem) & 3,
                4 => (scanline[byte_idx] >> rem) & 15,
                _ => unreachable!(),
            }
        })
}

/// Given `row_stride`, interlace `info`, and bits-per-pixel, produce an iterator of bit positions
/// of pixels to copy from the input scanline to the image buffer.  The positions are expressed as
/// bit offsets from position (0,0) in the frame that is currently being decoded.
fn expand_adam7_bits(
    row_stride_in_bytes: usize,
    info: &Adam7Info,
    bits_pp: usize,
) -> impl Iterator<Item = usize> {
    let line_no = info.line as usize;
    let pass = info.pass;
    let interlaced_width = info.width as usize;

    let (line_mul, line_off, samp_mul, samp_off) = match pass {
        1 => (8, 0, 8, 0),
        2 => (8, 0, 8, 4),
        3 => (8, 4, 4, 0),
        4 => (4, 0, 4, 2),
        5 => (4, 2, 2, 0),
        6 => (2, 0, 2, 1),
        7 => (2, 1, 1, 0),
        _ => {
            // `Adam7Info.pass` is a non-`pub`lic field.  `InterlaceInfo` is expected
            // to maintain an invariant that `pass` is valid.
            panic!("Invalid `Adam7Info.pass`");
        }
    };

    // the equivalent line number in progressive scan
    let prog_line = line_mul * line_no + line_off;
    let line_start = prog_line * row_stride_in_bytes * 8;

    (0..interlaced_width)
        .map(move |i| i * samp_mul + samp_off)
        .map(move |i| i * bits_pp)
        .map(move |bits_offset| bits_offset + line_start)
}

/// Copies pixels from `interlaced_row` into the right location in `img`.
///
/// First bytes of `img` should belong to the top-left corner of the currently decoded frame.
///
/// `img_row_stride` specifies an offset in bytes between subsequent rows of `img`.
/// This can be the width of the current frame being decoded, but this is not required - a bigger
/// stride may be useful if the frame being decoded is a sub-region of `img`.
///
/// `interlaced_row` and `interlace_info` typically come from
/// [crate::decoder::Reader::next_interlaced_row], but this is not required.  In particular, before
/// calling `expand_interlaced_row` one may need to expand the decoded row, so that its format and
/// `bits_per_pixel` matches that of `img`.  Note that in initial Adam7 passes the `interlaced_row`
/// may contain less pixels that the width of the frame being decoded (e.g. it contains only 1/8th
/// of pixels in the initial pass).
///
/// Example:
///
/// ```
/// use png::{expand_interlaced_row, Adam7Info};
/// let info = Adam7Info::new(5, 0, 4);  // 1st line of 5th pass has 4 pixels.
/// let mut img = vec![0; 8 * 8];
/// let row = vec![1, 2, 3, 4];
/// expand_interlaced_row(&mut img, 8, &row, &info, 8);
/// assert_eq!(&img, &[
///     0, 0, 0, 0, 0, 0, 0, 0,
///     0, 0, 0, 0, 0, 0, 0, 0,
///     1, 0, 2, 0, 3, 0, 4, 0,  // <= this is where the 1st line of 5s appears
///     0, 0, 0, 0, 0, 0, 0, 0,  //    in the schematic drawing of the passes at
///     0, 0, 0, 0, 0, 0, 0, 0,  //    https://en.wikipedia.org/wiki/Adam7_algorithm
///     0, 0, 0, 0, 0, 0, 0, 0,
///     0, 0, 0, 0, 0, 0, 0, 0,
///     0, 0, 0, 0, 0, 0, 0, 0,
/// ]);
/// ```
pub fn expand_pass(
    img: &mut [u8],
    img_row_stride: usize,
    interlaced_row: &[u8],
    interlace_info: &Adam7Info,
    bits_per_pixel: u8,
) {
    let bits_pp = bits_per_pixel as usize;

    let bit_indices = expand_adam7_bits(img_row_stride, interlace_info, bits_pp);

    if bits_pp < 8 {
        for (pos, px) in bit_indices.zip(subbyte_pixels(interlaced_row, bits_pp)) {
            let rem = 8 - pos % 8 - bits_pp;
            img[pos / 8] |= px << rem as u8;
        }
    } else {
        let bytes_pp = bits_pp / 8;

        for (bitpos, px) in bit_indices.zip(interlaced_row.chunks(bytes_pp)) {
            for (offset, val) in px.iter().enumerate() {
                img[bitpos / 8 + offset] = *val;
            }
        }
    }
}

#[test]
fn test_adam7() {
    /*
        1646
        7777
        5656
        7777
    */
    let it = Adam7Iterator::new(4, 4);
    let passes: Vec<_> = it.collect();
    assert_eq!(
        &*passes,
        &[
            Adam7Info {
                pass: 1,
                line: 0,
                width: 1
            },
            Adam7Info {
                pass: 4,
                line: 0,
                width: 1
            },
            Adam7Info {
                pass: 5,
                line: 0,
                width: 2
            },
            Adam7Info {
                pass: 6,
                line: 0,
                width: 2
            },
            Adam7Info {
                pass: 6,
                line: 1,
                width: 2
            },
            Adam7Info {
                pass: 7,
                line: 0,
                width: 4
            },
            Adam7Info {
                pass: 7,
                line: 1,
                width: 4
            }
        ]
    );
}

#[test]
fn test_subbyte_pixels() {
    let scanline = &[0b10101010, 0b10101010];

    let pixels = subbyte_pixels(scanline, 1).collect::<Vec<_>>();
    assert_eq!(pixels.len(), 16);
    assert_eq!(pixels, [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]);
}

#[test]
fn test_expand_adam7_bits() {
    let width = 32;
    let bits_pp = 1;
    let stride = width / 8;
    let info = |pass, line, img_width| create_adam7_info_for_tests(pass, line as u32, img_width);

    let expected = |offset: usize, step: usize, count: usize| {
        (0..count)
            .map(move |i| step * i + offset)
            .collect::<Vec<_>>()
    };

    for line_no in 0..8 {
        let start = 8 * line_no * width;

        assert_eq!(
            expand_adam7_bits(stride, &info(1, line_no, width), bits_pp).collect::<Vec<_>>(),
            expected(start, 8, 4)
        );

        let start = start + 4;

        assert_eq!(
            expand_adam7_bits(stride, &info(2, line_no, width), bits_pp).collect::<Vec<_>>(),
            expected(start, 8, 4)
        );

        let start = (8 * line_no + 4) * width;

        assert_eq!(
            expand_adam7_bits(stride, &info(3, line_no, width), bits_pp).collect::<Vec<_>>(),
            expected(start, 4, 8)
        );
    }

    for line_no in 0..16 {
        let start = 4 * line_no * width + 2;

        assert_eq!(
            expand_adam7_bits(stride, &info(4, line_no, width), bits_pp).collect::<Vec<_>>(),
            expected(start, 4, 8)
        );

        let start = (4 * line_no + 2) * width;

        assert_eq!(
            expand_adam7_bits(stride, &info(5, line_no, width), bits_pp).collect::<Vec<_>>(),
            expected(start, 2, 16)
        )
    }

    for line_no in 0..32 {
        let start = 2 * line_no * width + 1;

        assert_eq!(
            expand_adam7_bits(stride, &info(6, line_no, width), bits_pp).collect::<Vec<_>>(),
            expected(start, 2, 16),
            "line_no: {}",
            line_no
        );

        let start = (2 * line_no + 1) * width;

        assert_eq!(
            expand_adam7_bits(stride, &info(7, line_no, width), bits_pp).collect::<Vec<_>>(),
            expected(start, 1, 32)
        );
    }
}

#[test]
fn test_expand_adam7_bits_independent_row_stride() {
    let pass = 1;
    let line_no = 1;
    let width = 32;
    let bits_pp = 8;
    let info = create_adam7_info_for_tests;

    {
        let stride = width;
        assert_eq!(
            expand_adam7_bits(stride, &info(pass, line_no, width), bits_pp).collect::<Vec<_>>(),
            vec![2048, 2112, 2176, 2240],
        );
    }

    {
        let stride = 10000;
        assert_eq!(
            expand_adam7_bits(stride, &info(pass, line_no, width), bits_pp).collect::<Vec<_>>(),
            vec![640000, 640064, 640128, 640192],
        );
    }
}

#[test]
fn test_expand_pass_subbyte() {
    let mut img = [0u8; 8];
    let width = 8;
    let stride = width / 8;
    let bits_pp = 1;
    let info = create_adam7_info_for_tests;

    expand_pass(&mut img, stride, &[0b10000000], &info(1, 0, width), bits_pp);
    assert_eq!(img, [0b10000000u8, 0, 0, 0, 0, 0, 0, 0]);

    expand_pass(&mut img, stride, &[0b10000000], &info(2, 0, width), bits_pp);
    assert_eq!(img, [0b10001000u8, 0, 0, 0, 0, 0, 0, 0]);

    expand_pass(&mut img, stride, &[0b11000000], &info(3, 0, width), bits_pp);
    assert_eq!(img, [0b10001000u8, 0, 0, 0, 0b10001000, 0, 0, 0]);

    expand_pass(&mut img, stride, &[0b11000000], &info(4, 0, width), bits_pp);
    assert_eq!(img, [0b10101010u8, 0, 0, 0, 0b10001000, 0, 0, 0]);

    expand_pass(&mut img, stride, &[0b11000000], &info(4, 1, width), bits_pp);
    assert_eq!(img, [0b10101010u8, 0, 0, 0, 0b10101010, 0, 0, 0]);

    expand_pass(&mut img, stride, &[0b11110000], &info(5, 0, width), bits_pp);
    assert_eq!(img, [0b10101010u8, 0, 0b10101010, 0, 0b10101010, 0, 0, 0]);

    expand_pass(&mut img, stride, &[0b11110000], &info(5, 1, width), bits_pp);
    assert_eq!(
        img,
        [0b10101010u8, 0, 0b10101010, 0, 0b10101010, 0, 0b10101010, 0]
    );

    expand_pass(&mut img, stride, &[0b11110000], &info(6, 0, width), bits_pp);
    assert_eq!(
        img,
        [0b11111111u8, 0, 0b10101010, 0, 0b10101010, 0, 0b10101010, 0]
    );

    expand_pass(&mut img, stride, &[0b11110000], &info(6, 1, width), bits_pp);
    assert_eq!(
        img,
        [0b11111111u8, 0, 0b11111111, 0, 0b10101010, 0, 0b10101010, 0]
    );

    expand_pass(&mut img, stride, &[0b11110000], &info(6, 2, width), bits_pp);
    assert_eq!(
        img,
        [0b11111111u8, 0, 0b11111111, 0, 0b11111111, 0, 0b10101010, 0]
    );

    expand_pass(&mut img, stride, &[0b11110000], &info(6, 3, width), bits_pp);
    assert_eq!(
        [0b11111111u8, 0, 0b11111111, 0, 0b11111111, 0, 0b11111111, 0],
        img
    );

    expand_pass(&mut img, stride, &[0b11111111], &info(7, 0, width), bits_pp);
    assert_eq!(
        [
            0b11111111u8,
            0b11111111,
            0b11111111,
            0,
            0b11111111,
            0,
            0b11111111,
            0
        ],
        img
    );

    expand_pass(&mut img, stride, &[0b11111111], &info(7, 1, width), bits_pp);
    assert_eq!(
        [
            0b11111111u8,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0,
            0b11111111,
            0
        ],
        img
    );

    expand_pass(&mut img, stride, &[0b11111111], &info(7, 2, width), bits_pp);
    assert_eq!(
        [
            0b11111111u8,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0
        ],
        img
    );

    expand_pass(&mut img, stride, &[0b11111111], &info(7, 3, width), bits_pp);
    assert_eq!(
        [
            0b11111111u8,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111
        ],
        img
    );
}

#[cfg(test)]
fn create_adam7_info_for_tests(pass: u8, line: u32, img_width: usize) -> Adam7Info {
    let width = {
        let img_height = 8;
        Adam7Iterator::new(img_width as u32, img_height)
            .filter(|info| info.pass == pass)
            .map(|info| info.width)
            .next()
            .unwrap()
    };

    Adam7Info { pass, line, width }
}
