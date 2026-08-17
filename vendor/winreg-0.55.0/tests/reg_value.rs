use winreg::{types::ToRegValue, RegValue};

macro_rules! test_display {
    ($f:ident, $v:expr) => {
        #[test]
        fn $f() {
            let val = $v;
            let rval: RegValue = val.to_reg_value();
            assert_eq!(val.to_string(), rval.to_string());
        }
    };
}

test_display!(test_display_string, "Test\\123");
test_display!(test_display_u32, 1234u32);
test_display!(test_display_u64, 1234567890u64);
