/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_sdk_s3::types::Object;

// Tests that `com.amazonaws.s3#Size` is correctly customized to be a long instead of an int.
#[test]
fn size_type() {
    let size = i64::MAX;

    // Should only compile if the type is correctly customized
    let object = Object::builder().size(size).build();
    assert_eq!(Some(size), object.size);
}
