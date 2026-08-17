#[cfg(target_arch = "x86_64")]
#[rustversion::since(1.86.0)]
mod msrv_1_86 {
    #![allow(dead_code)]
    use crate::prelude::*;

    #[test]
    fn target_feature_function() {
        #[builder]
        #[target_feature(enable = "avx2")]
        fn building_but_wider(_x: [u8; 32], _y: [u32; 8]) {}

        #[target_feature(enable = "avx2")]
        #[allow(unsafe_code)]
        unsafe fn wider() {
            building_but_wider().x([0; 32]).y([1; 8]).call();
        }
    }

    #[test]
    fn target_feature_method() {
        #[repr(C, align(32))]
        struct Brick([u8; 32]);
        struct Senti;

        #[bon]
        impl Senti {
            #[builder(finish_fn = yatta_but_wide)]
            #[target_feature(enable = "avx2")]
            fn new(brick: Brick) -> Self {
                let Brick(_) = brick;
                Self
            }
            #[target_feature(enable = "avx2")]
            #[allow(unsafe_code)]
            unsafe fn briiick() {
                Self::builder().brick(Brick([0; 32])).yatta_but_wide();
            }
        }
    }
}
