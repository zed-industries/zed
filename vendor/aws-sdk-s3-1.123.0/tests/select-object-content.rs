/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::types::{
    CompressionType, CsvInput, CsvOutput, ExpressionType, FileHeaderInfo, InputSerialization,
    OutputSerialization, SelectObjectContentEventStream,
};
use aws_sdk_s3::Client;
use aws_smithy_http_client::test_util::dvr::{Event, ReplayingClient};
use aws_smithy_protocol_test::{assert_ok, validate_body, MediaType};
use std::error::Error;

#[tokio::test]
async fn test_success() {
    let events: Vec<Event> =
        serde_json::from_str(include_str!("select-object-content.json")).unwrap();
    let replayer = ReplayingClient::new(events);
    let sdk_config = SdkConfig::builder()
        .region(Region::from_static("us-east-2"))
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .http_client(replayer.clone())
        .build();
    let client = Client::new(&sdk_config);

    let mut output = client
        .select_object_content()
        .bucket("aws-rust-sdk")
        .key("sample_data.csv")
        .expression_type(ExpressionType::Sql)
        .expression("SELECT * FROM s3object s WHERE s.\"Name\" = 'Jane'")
        .input_serialization(
            InputSerialization::builder()
                .csv(
                    CsvInput::builder()
                        .file_header_info(FileHeaderInfo::Use)
                        .build(),
                )
                .compression_type(CompressionType::None)
                .build(),
        )
        .output_serialization(
            OutputSerialization::builder()
                .csv(CsvOutput::builder().build())
                .build(),
        )
        .send()
        .await
        .unwrap();

    let mut received = Vec::new();
    while let Some(event) = output.payload.recv().await.unwrap() {
        match event {
            SelectObjectContentEventStream::Records(records) => {
                received.push(
                    std::str::from_utf8(records.payload.as_ref().unwrap().as_ref())
                        .unwrap()
                        .trim()
                        .to_string(),
                );
            }
            SelectObjectContentEventStream::Stats(stats) => {
                let stats = stats.details.unwrap();
                received.push(format!(
                    "scanned:{},processed:{},returned:{}",
                    stats.bytes_scanned.unwrap(),
                    stats.bytes_processed.unwrap(),
                    stats.bytes_returned.unwrap()
                ))
            }
            SelectObjectContentEventStream::End(_) => {}
            otherwise => panic!("unexpected message: {:?}", otherwise),
        }
    }
    assert_eq!(
        vec![
            "Jane,(949) 555-6704,Chicago,Developer".to_string(),
            "scanned:333,processed:333,returned:39".to_string()
        ],
        received
    );

    // Validate the requests
    replayer
        .validate(&["content-type", "content-length"], body_validator)
        .await
        .unwrap();
}

fn body_validator(expected_body: &[u8], actual_body: &[u8]) -> Result<(), Box<dyn Error>> {
    let expected = std::str::from_utf8(expected_body).unwrap();
    let actual = std::str::from_utf8(actual_body).unwrap();
    assert_ok(validate_body(actual, expected, MediaType::Xml));
    Ok(())
}
