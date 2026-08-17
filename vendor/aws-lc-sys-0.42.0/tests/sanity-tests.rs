// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#[test]
fn test_fips_mode() {
    unsafe {
        assert_eq!(aws_lc_sys::FIPS_mode(), 0);
    }
}

#[test]
fn error_checking() {
    unsafe {
        let error = aws_lc_sys::ERR_get_error();
        let err_lib = aws_lc_sys::ERR_GET_LIB(error);
        let err_reason = aws_lc_sys::ERR_GET_REASON(error);
        let err_func = aws_lc_sys::ERR_GET_FUNC(error);
        assert_eq!(err_lib, 0);
        assert_eq!(err_reason, 0);
        assert_eq!(err_func, 0);
    }
}
