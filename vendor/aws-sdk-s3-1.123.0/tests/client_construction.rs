/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

mod with_sdk_config {
    use aws_config::SdkConfig;
    use aws_sdk_s3 as s3;
    use s3::config::StalledStreamProtectionConfig;

    #[tokio::test]
    async fn using_config_loader() {
        // When using `aws_config::load_from_env`, things should just work
        let config = aws_config::load_from_env().await;
        assert!(config.timeout_config().unwrap().has_timeouts());
        assert!(config.retry_config().unwrap().has_retry());
        let _s3 = s3::Client::new(&config);
    }

    #[test]
    fn manual_config_construction_all_defaults() {
        // When manually constructing `SdkConfig` with everything unset,
        // it should work since there will be no timeouts or retries enabled,
        // and thus, no sleep impl is required.
        let config = SdkConfig::builder()
            .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
            .build();
        assert!(config.timeout_config().is_none());
        assert!(config.retry_config().is_none());
        let _s3 = s3::Client::new(&config);
    }

    #[test]
    fn bytestream_from_path_exists() {
        let _ = aws_sdk_s3::primitives::ByteStream::from_path("a/b.txt");
    }
}

mod with_service_config {
    use aws_sdk_s3 as s3;
    use aws_smithy_runtime_api::client::behavior_version::BehaviorVersion;

    #[test]
    fn manual_config_construction_all_defaults() {
        // When manually constructing `Config` with everything unset,
        // it should work since there will be no timeouts or retries enabled,
        // and thus, no sleep impl is required.
        let config = s3::Config::builder().build();
        let _s3 = s3::Client::from_conf(config);
    }

    #[test]
    fn test_default_retry_enabled_with_bmv_2026_01_12() {
        // With v2026_01_12 and later, retries are enabled by default for AWS SDK clients
        // This test verifies the client builds without panicking about missing sleep impl
        let config = s3::Config::builder()
            .behavior_version(BehaviorVersion::v2026_01_12())
            .region(aws_types::region::Region::new("us-east-1"))
            .credentials_provider(aws_credential_types::Credentials::for_tests())
            .build();

        // Should build successfully even though retries are enabled
        // (sleep impl is provided by default)
        let _client = s3::Client::from_conf(config);
    }

    #[test]
    #[allow(deprecated)]
    fn test_client_with_old_behavior_version_builds_successfully() {
        // With v2024_03_28 (older than v2026_01_12), retries are NOT enabled by default
        // This test verifies the client builds without requiring a sleep impl
        let config = s3::Config::builder()
            .behavior_version(BehaviorVersion::v2024_03_28())
            .region(aws_types::region::Region::new("us-east-1"))
            .credentials_provider(aws_credential_types::Credentials::for_tests())
            .build();

        // Should build successfully (no retries = no sleep impl needed)
        let _client = s3::Client::from_conf(config);
    }
}
