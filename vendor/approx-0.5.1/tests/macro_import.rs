extern crate approx;

mod test_macro_import {
    use approx::{
        assert_abs_diff_eq, assert_abs_diff_ne, assert_relative_eq, assert_relative_ne,
        assert_ulps_eq, assert_ulps_ne,
    };

    #[test]
    fn test() {
        assert_abs_diff_eq!(1.0f32, 1.0f32);
        assert_abs_diff_ne!(1.0f32, 2.0f32);
        assert_relative_eq!(1.0f32, 1.0f32);
        assert_relative_ne!(1.0f32, 2.0f32);
        assert_ulps_eq!(1.0f32, 1.0f32);
        assert_ulps_ne!(1.0f32, 2.0f32);
    }
}
