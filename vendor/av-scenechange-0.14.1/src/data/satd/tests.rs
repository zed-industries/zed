use v_frame::{pixel::Pixel, plane::Plane};

use crate::{
    data::{
        plane::{Area, AsRegion},
        satd::get_satd,
    },
    CpuFeatureLevel,
};

// Generate plane data for get_sad_same()
fn setup_planes<T: Pixel>() -> (Plane<T>, Plane<T>) {
    // Two planes with different strides
    let mut input_plane = Plane::new(640, 480, 0, 0, 128 + 8, 128 + 8);
    let mut rec_plane = Plane::new(640, 480, 0, 0, 2 * 128 + 8, 2 * 128 + 8);

    // Make the test pattern robust to data alignment
    let xpad_off = (input_plane.cfg.xorigin - input_plane.cfg.xpad) as i32 - 8i32;

    for (i, row) in input_plane
        .data
        .chunks_mut(input_plane.cfg.stride)
        .enumerate()
    {
        for (j, pixel) in row.iter_mut().enumerate() {
            let val = ((j + i) as i32 - xpad_off) & 255i32;
            assert!(val >= u8::MIN.into() && val <= u8::MAX.into());
            *pixel = T::cast_from(val);
        }
    }

    for (i, row) in rec_plane.data.chunks_mut(rec_plane.cfg.stride).enumerate() {
        for (j, pixel) in row.iter_mut().enumerate() {
            let val = (j as i32 - i as i32 - xpad_off) & 255i32;
            assert!(val >= u8::MIN.into() && val <= u8::MAX.into());
            *pixel = T::cast_from(val);
        }
    }

    (input_plane, rec_plane)
}

fn get_satd_same_inner<T: Pixel>() {
    let blocks: Vec<(usize, usize, u32)> = vec![
        (4, 4, 1408),
        (4, 8, 2016),
        (8, 4, 1816),
        (8, 8, 3984),
        (8, 16, 5136),
        (16, 8, 4864),
        (16, 16, 9984),
        (16, 32, 13824),
        (32, 16, 13760),
        (32, 32, 27952),
        (32, 64, 37168),
        (64, 32, 45104),
        (64, 64, 84176),
        (64, 128, 127920),
        (128, 64, 173680),
        (128, 128, 321456),
        (4, 16, 3136),
        (16, 4, 2632),
        (8, 32, 7056),
        (32, 8, 6624),
        (16, 64, 18432),
        (64, 16, 21312),
    ];

    let bit_depth: usize = 8;
    let (input_plane, rec_plane) = setup_planes::<T>();

    for (w, h, distortion) in blocks {
        let area = Area::StartingAt { x: 32, y: 40 };

        let input_region = input_plane.region(area);
        let rec_region = rec_plane.region(area);

        assert_eq!(
            distortion,
            get_satd(
                &input_region,
                &rec_region,
                w,
                h,
                bit_depth,
                CpuFeatureLevel::default()
            )
        );
    }
}

#[test]
fn get_satd_same_u8() {
    get_satd_same_inner::<u8>();
}

#[test]
fn get_satd_same_u16() {
    get_satd_same_inner::<u16>();
}
