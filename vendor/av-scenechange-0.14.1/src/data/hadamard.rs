pub unsafe fn hadamard4x4(data: &mut [i32]) {
    hadamard2d::<{ 4 * 4 }, 4, 4>(&mut *(data.as_mut_ptr() as *mut [i32; 16]));
}

// SAFETY: The length of data must be 64.
pub unsafe fn hadamard8x8(data: &mut [i32]) {
    hadamard2d::<{ 8 * 8 }, 8, 8>(&mut *(data.as_mut_ptr() as *mut [i32; 64]));
}

fn hadamard2d<const LEN: usize, const W: usize, const H: usize>(data: &mut [i32; LEN]) {
    // Vertical transform.
    let vert_func = if H == 4 {
        hadamard4_1d::<LEN, W, 1, H>
    } else {
        hadamard8_1d::<LEN, W, 1, H>
    };
    vert_func(data);

    // Horizontal transform.
    let horz_func = if W == 4 {
        hadamard4_1d::<LEN, H, W, 1>
    } else {
        hadamard8_1d::<LEN, H, W, 1>
    };
    horz_func(data);
}

#[allow(clippy::erasing_op)]
#[allow(clippy::identity_op)]
fn hadamard4_1d<const LEN: usize, const N: usize, const STRIDE0: usize, const STRIDE1: usize>(
    data: &mut [i32; LEN],
) {
    for i in 0..N {
        let sub: &mut [i32] = &mut data[i * STRIDE0..];
        let (a0, a1) = butterfly(sub[0 * STRIDE1], sub[1 * STRIDE1]);
        let (a2, a3) = butterfly(sub[2 * STRIDE1], sub[3 * STRIDE1]);
        let (b0, b2) = butterfly(a0, a2);
        let (b1, b3) = butterfly(a1, a3);
        sub[0 * STRIDE1] = b0;
        sub[1 * STRIDE1] = b1;
        sub[2 * STRIDE1] = b2;
        sub[3 * STRIDE1] = b3;
    }
}

#[allow(clippy::erasing_op)]
#[allow(clippy::identity_op)]
fn hadamard8_1d<const LEN: usize, const N: usize, const STRIDE0: usize, const STRIDE1: usize>(
    data: &mut [i32; LEN],
) {
    for i in 0..N {
        let sub: &mut [i32] = &mut data[i * STRIDE0..];

        let (a0, a1) = butterfly(sub[0 * STRIDE1], sub[1 * STRIDE1]);
        let (a2, a3) = butterfly(sub[2 * STRIDE1], sub[3 * STRIDE1]);
        let (a4, a5) = butterfly(sub[4 * STRIDE1], sub[5 * STRIDE1]);
        let (a6, a7) = butterfly(sub[6 * STRIDE1], sub[7 * STRIDE1]);

        let (b0, b2) = butterfly(a0, a2);
        let (b1, b3) = butterfly(a1, a3);
        let (b4, b6) = butterfly(a4, a6);
        let (b5, b7) = butterfly(a5, a7);

        let (c0, c4) = butterfly(b0, b4);
        let (c1, c5) = butterfly(b1, b5);
        let (c2, c6) = butterfly(b2, b6);
        let (c3, c7) = butterfly(b3, b7);

        sub[0 * STRIDE1] = c0;
        sub[1 * STRIDE1] = c1;
        sub[2 * STRIDE1] = c2;
        sub[3 * STRIDE1] = c3;
        sub[4 * STRIDE1] = c4;
        sub[5 * STRIDE1] = c5;
        sub[6 * STRIDE1] = c6;
        sub[7 * STRIDE1] = c7;
    }
}

const fn butterfly(a: i32, b: i32) -> (i32, i32) {
    ((a + b), (a - b))
}
