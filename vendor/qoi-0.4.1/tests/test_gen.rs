mod common;

use bytemuck::cast_slice;
use std::borrow::Cow;
use std::fmt::Debug;

use cfg_if::cfg_if;
use rand::{
    distributions::{Distribution, Standard},
    rngs::StdRng,
    Rng, SeedableRng,
};

use libqoi::{qoi_decode, qoi_encode};
use qoi::consts::{
    QOI_HEADER_SIZE, QOI_MASK_2, QOI_OP_DIFF, QOI_OP_INDEX, QOI_OP_LUMA, QOI_OP_RGB, QOI_OP_RGBA,
    QOI_OP_RUN, QOI_PADDING_SIZE,
};
use qoi::{decode_header, decode_to_vec, encode_to_vec};

use self::common::hash;

struct GenState<const N: usize> {
    index: [[u8; N]; 64],
    pixels: Vec<u8>,
    prev: [u8; N],
    len: usize,
}

impl<const N: usize> GenState<N> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            index: [[0; N]; 64],
            pixels: Vec::with_capacity(capacity * N),
            prev: Self::zero(),
            len: 0,
        }
    }
    pub fn write(&mut self, px: [u8; N]) {
        self.index[hash(px) as usize] = px;
        for i in 0..N {
            self.pixels.push(px[i]);
        }
        self.prev = px;
        self.len += 1;
    }

    pub fn pick_from_index(&self, rng: &mut impl Rng) -> [u8; N] {
        self.index[rng.gen_range(0_usize..64)]
    }

    pub fn zero() -> [u8; N] {
        let mut px = [0; N];
        if N >= 4 {
            px[3] = 0xff;
        }
        px
    }
}

struct ImageGen {
    p_new: f64,
    p_index: f64,
    p_repeat: f64,
    p_diff: f64,
    p_luma: f64,
}

impl ImageGen {
    pub fn new_random(rng: &mut impl Rng) -> Self {
        let p: [f64; 6] = rng.gen();
        let t = p.iter().sum::<f64>();
        Self {
            p_new: p[0] / t,
            p_index: p[1] / t,
            p_repeat: p[2] / t,
            p_diff: p[3] / t,
            p_luma: p[4] / t,
        }
    }

    pub fn generate(&self, rng: &mut impl Rng, channels: usize, min_len: usize) -> Vec<u8> {
        match channels {
            3 => self.generate_const::<_, 3>(rng, min_len),
            4 => self.generate_const::<_, 4>(rng, min_len),
            _ => panic!(),
        }
    }

    fn generate_const<R: Rng, const N: usize>(&self, rng: &mut R, min_len: usize) -> Vec<u8>
    where
        Standard: Distribution<[u8; N]>,
    {
        let mut s = GenState::<N>::with_capacity(min_len);
        let zero = GenState::<N>::zero();

        while s.len < min_len {
            let mut p = rng.gen_range(0.0..1.0);

            if p < self.p_new {
                s.write(rng.gen());
                continue;
            }
            p -= self.p_new;

            if p < self.p_index {
                let px = s.pick_from_index(rng);
                s.write(px);
                continue;
            }
            p -= self.p_index;

            if p < self.p_repeat {
                let px = s.prev;
                let n_repeat = rng.gen_range(1_usize..=70);
                for _ in 0..n_repeat {
                    s.write(px);
                }
                continue;
            }
            p -= self.p_repeat;

            if p < self.p_diff {
                let mut px = s.prev;
                px[0] = px[0].wrapping_add(rng.gen_range(0_u8..4).wrapping_sub(2));
                px[1] = px[1].wrapping_add(rng.gen_range(0_u8..4).wrapping_sub(2));
                px[2] = px[2].wrapping_add(rng.gen_range(0_u8..4).wrapping_sub(2));
                s.write(px);
                continue;
            }
            p -= self.p_diff;

            if p < self.p_luma {
                let mut px = s.prev;
                let vg = rng.gen_range(0_u8..64).wrapping_sub(32);
                let vr = rng.gen_range(0_u8..16).wrapping_sub(8).wrapping_add(vg);
                let vb = rng.gen_range(0_u8..16).wrapping_sub(8).wrapping_add(vg);
                px[0] = px[0].wrapping_add(vr);
                px[1] = px[1].wrapping_add(vg);
                px[2] = px[2].wrapping_add(vb);
                s.write(px);
                continue;
            }

            s.write(zero);
        }

        s.pixels
    }
}

fn format_encoded(encoded: &[u8]) -> String {
    let header = decode_header(encoded).unwrap();
    let mut data = &encoded[QOI_HEADER_SIZE..encoded.len() - QOI_PADDING_SIZE];
    let mut s = format!("{}x{}:{} = [", header.width, header.height, header.channels.as_u8());
    while !data.is_empty() {
        let b1 = data[0];
        data = &data[1..];
        match b1 {
            QOI_OP_RGB => {
                s.push_str(&format!("rgb({},{},{})", data[0], data[1], data[2]));
                data = &data[3..];
            }
            QOI_OP_RGBA => {
                s.push_str(&format!("rgba({},{},{},{})", data[0], data[1], data[2], data[3]));
                data = &data[4..];
            }
            _ => match b1 & QOI_MASK_2 {
                QOI_OP_INDEX => s.push_str(&format!("index({})", b1 & 0x3f)),
                QOI_OP_RUN => s.push_str(&format!("run({})", b1 & 0x3f)),
                QOI_OP_DIFF => s.push_str(&format!(
                    "diff({},{},{})",
                    (b1 >> 4) & 0x03,
                    (b1 >> 2) & 0x03,
                    b1 & 0x03
                )),
                QOI_OP_LUMA => {
                    let b2 = data[0];
                    data = &data[1..];
                    s.push_str(&format!("luma({},{},{})", (b2 >> 4) & 0x0f, b1 & 0x3f, b2 & 0x0f))
                }
                _ => {}
            },
        }
        s.push_str(", ");
    }
    s.pop().unwrap();
    s.pop().unwrap();
    s.push(']');
    s
}

fn check_roundtrip<E, D, VE, VD, EE, ED>(
    msg: &str, mut data: &[u8], channels: usize, encode: E, decode: D,
) where
    E: Fn(&[u8], u32) -> Result<VE, EE>,
    D: Fn(&[u8]) -> Result<VD, ED>,
    VE: AsRef<[u8]>,
    VD: AsRef<[u8]>,
    EE: Debug,
    ED: Debug,
{
    macro_rules! rt {
        ($data:expr, $n:expr) => {
            decode(encode($data, $n as _).unwrap().as_ref()).unwrap()
        };
    }
    macro_rules! fail {
        ($msg:expr, $data:expr, $decoded:expr, $encoded:expr, $channels:expr) => {
            assert!(
                false,
                "{} roundtrip failed\n\n  image: {:?}\ndecoded: {:?}\nencoded: {}",
                $msg,
                cast_slice::<_, [u8; $channels]>($data.as_ref()),
                cast_slice::<_, [u8; $channels]>($decoded.as_ref()),
                format_encoded($encoded.as_ref()),
            );
        };
    }

    let mut n_pixels = data.len() / channels;
    assert_eq!(n_pixels * channels, data.len());

    // if all ok, return
    // ... but if roundtrip check fails, try to reduce the example to the smallest we can find
    if rt!(data, n_pixels).as_ref() == data {
        return;
    }

    // try removing pixels from the beginning
    while n_pixels > 1 {
        let slice = &data[..data.len() - channels];
        if rt!(slice, n_pixels - 1).as_ref() != slice {
            data = slice;
            n_pixels -= 1;
        } else {
            break;
        }
    }

    // try removing pixels from the end
    while n_pixels > 1 {
        let slice = &data[channels..];
        if rt!(slice, n_pixels - 1).as_ref() != slice {
            data = slice;
            n_pixels -= 1;
        } else {
            break;
        }
    }

    // try removing pixels from the middle
    let mut data = Cow::from(data);
    let mut pos = 1;
    while n_pixels > 1 && pos < n_pixels - 1 {
        let mut vec = data.to_vec();
        for _ in 0..channels {
            vec.remove(pos * channels);
        }
        if rt!(vec.as_slice(), n_pixels - 1).as_ref() != vec.as_slice() {
            data = Cow::from(vec);
            n_pixels -= 1;
        } else {
            pos += 1;
        }
    }

    let encoded = encode(data.as_ref(), n_pixels as _).unwrap();
    let decoded = decode(encoded.as_ref()).unwrap();
    assert_ne!(decoded.as_ref(), data.as_ref());
    if channels == 3 {
        fail!(msg, data, decoded, encoded, 3);
    } else {
        fail!(msg, data, decoded, encoded, 4);
    }
}

#[test]
fn test_generated() {
    let mut rng = StdRng::seed_from_u64(0);

    let mut n_pixels = 0;
    while n_pixels < 20_000_000 {
        let min_len = rng.gen_range(1..=5000);
        let channels = rng.gen_range(3..=4);
        let gen = ImageGen::new_random(&mut rng);
        let img = gen.generate(&mut rng, channels, min_len);

        let encode = |data: &[u8], size| encode_to_vec(data, size, 1);
        let decode = |data: &[u8]| decode_to_vec(data).map(|r| r.1);
        let encode_c = |data: &[u8], size| qoi_encode(data, size, 1, channels as _);
        let decode_c = |data: &[u8]| qoi_decode(data, channels as _).map(|r| r.1);

        check_roundtrip("qoi-rust -> qoi-rust", &img, channels as _, encode, decode);
        check_roundtrip("qoi-rust -> qoi.h", &img, channels as _, encode, decode_c);
        check_roundtrip("qoi.h -> qoi-rust", &img, channels as _, encode_c, decode);

        let size = (img.len() / channels) as u32;
        let encoded = encode(&img, size).unwrap();
        let encoded_c = encode_c(&img, size).unwrap();
        cfg_if! {
            if #[cfg(feature = "reference")] {
                let eq = encoded.as_slice() == encoded_c.as_ref();
                assert!(eq, "qoi-rust [reference mode] doesn't match qoi.h");
            } else {
                let eq = encoded.len() == encoded_c.len();
                assert!(eq, "qoi-rust [non-reference mode] length doesn't match qoi.h");
            }
        }

        n_pixels += size;
    }
}
