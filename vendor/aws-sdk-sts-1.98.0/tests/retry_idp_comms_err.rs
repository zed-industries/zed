/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_sdk_sts as sts;
use aws_smithy_types::error::ErrorMetadata;
use aws_smithy_types::retry::{ErrorKind, ProvideErrorKind};
use sts::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError;
use sts::types::error::IdpCommunicationErrorException;

#[tokio::test]
async fn idp_comms_err_retryable() {
    let error = AssumeRoleWithWebIdentityError::IdpCommunicationErrorException(
        IdpCommunicationErrorException::builder()
            .message("test")
            .meta(
                ErrorMetadata::builder()
                    .code("IDPCommunicationError")
                    .message("test")
                    .build(),
            )
            .build(),
    );
    assert_eq!(
        Some(ErrorKind::ServerError),
        error.retryable_error_kind(),
        "IdpCommunicationErrorException should be a retryable server error"
    );
}
