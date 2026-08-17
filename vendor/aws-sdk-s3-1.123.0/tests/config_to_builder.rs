/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#[tokio::test]
async fn test_config_to_builder() {
    use aws_sdk_s3::config::AppName;

    let config = aws_config::load_from_env().await;
    let config = aws_sdk_s3::Config::new(&config);
    // should not panic
    let _ = config
        .to_builder()
        .app_name(AppName::new("SomeAppName").unwrap())
        .build();
}
