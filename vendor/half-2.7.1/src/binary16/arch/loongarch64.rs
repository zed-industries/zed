use core::{mem::MaybeUninit, ptr};

#[cfg(target_arch = "loongarch64")]
use core::arch::loongarch64::{lsx_vfcvt_h_s, lsx_vfcvtl_s_h, m128, m128i};

/////////////// loongarch64 lsx/lasx ////////////////

#[target_feature(enable = "lsx")]
#[inline]
pub(super) unsafe fn f16_to_f32_lsx(i: u16) -> f32 {
    let mut vec = MaybeUninit::<m128i>::zeroed();
    vec.as_mut_ptr().cast::<u16>().write(i);
    let retval = lsx_vfcvtl_s_h(vec.assume_init());
    *(&retval as *const m128).cast()
}

#[target_feature(enable = "lsx")]
#[inline]
pub(super) unsafe fn f32_to_f16_lsx(f: f32) -> u16 {
    let mut vec = MaybeUninit::<m128>::zeroed();
    vec.as_mut_ptr().cast::<f32>().write(f);
    let retval = lsx_vfcvt_h_s(vec.assume_init(), vec.assume_init());
    *(&retval as *const m128i).cast()
}

#[target_feature(enable = "lsx")]
#[inline]
pub(super) unsafe fn f16x4_to_f32x4_lsx(v: &[u16; 4]) -> [f32; 4] {
    let mut vec = MaybeUninit::<m128i>::zeroed();
    ptr::copy_nonoverlapping(v.as_ptr(), vec.as_mut_ptr().cast(), 4);
    let retval = lsx_vfcvtl_s_h(vec.assume_init());
    *(&retval as *const m128).cast()
}

#[target_feature(enable = "lsx")]
#[inline]
pub(super) unsafe fn f32x4_to_f16x4_lsx(v: &[f32; 4]) -> [u16; 4] {
    let mut vec = MaybeUninit::<m128>::uninit();
    ptr::copy_nonoverlapping(v.as_ptr(), vec.as_mut_ptr().cast(), 4);
    let retval = lsx_vfcvt_h_s(vec.assume_init(), vec.assume_init());
    *(&retval as *const m128i).cast()
}

#[target_feature(enable = "lsx")]
#[inline]
pub(super) unsafe fn f16x4_to_f64x4_lsx(v: &[u16; 4]) -> [f64; 4] {
    let array = f16x4_to_f32x4_lsx(v);
    // Let compiler vectorize this regular cast for now.
    [
        array[0] as f64,
        array[1] as f64,
        array[2] as f64,
        array[3] as f64,
    ]
}

#[target_feature(enable = "lsx")]
#[inline]
pub(super) unsafe fn f64x4_to_f16x4_lsx(v: &[f64; 4]) -> [u16; 4] {
    // Let compiler vectorize this regular cast for now.
    let v = [v[0] as f32, v[1] as f32, v[2] as f32, v[3] as f32];
    f32x4_to_f16x4_lsx(&v)
}
