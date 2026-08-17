/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::frame::{read_message_from, DecodedFrame, MessageFrameDecoder};
use aws_smithy_types::event_stream::{HeaderValue, Message};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error as StdError;

/// Validate that the bodies match, which includes headers and messages
///
/// When `full_stream` is true, it also verifies the length of frames
pub fn validate_body(
    expected_body: &[u8],
    actual_body: &[u8],
    full_stream: bool,
) -> Result<(), Box<dyn StdError>> {
    let expected_frames = decode_frames(expected_body);
    let actual_frames = decode_frames(actual_body);

    if full_stream {
        assert_eq!(
            expected_frames.len(),
            actual_frames.len(),
            "Frame count didn't match.\n\
        Expected: {expected_frames:?}\n\
        Actual:   {actual_frames:?}",
        );
    }

    for ((expected_wrapper, expected_message), (actual_wrapper, actual_message)) in
        expected_frames.into_iter().zip(actual_frames.into_iter())
    {
        assert_eq!(
            header_names(&expected_wrapper),
            header_names(&actual_wrapper)
        );
        if let Some(expected_message) = expected_message {
            let actual_message = actual_message.unwrap();
            assert_eq!(header_map(&expected_message), header_map(&actual_message));
            assert_eq!(expected_message.payload(), actual_message.payload());
        }
    }
    Ok(())
}

// Returned tuples are (SignedWrapperMessage, WrappedMessage).
// Some signed messages don't have payloads, so in those cases, the wrapped message will be None.
fn decode_frames(mut body: &[u8]) -> Vec<(Message, Option<Message>)> {
    let mut result = Vec::new();
    let mut decoder = MessageFrameDecoder::new();
    while let DecodedFrame::Complete(msg) = decoder.decode_frame(&mut body).unwrap() {
        let inner_msg = if msg.payload().is_empty() {
            None
        } else {
            Some(read_message_from(msg.payload().as_ref()).unwrap())
        };
        result.push((msg, inner_msg));
    }
    result
}

fn header_names(msg: &Message) -> BTreeSet<String> {
    msg.headers()
        .iter()
        .map(|h| h.name().as_str().into())
        .collect()
}
fn header_map(msg: &Message) -> BTreeMap<String, &HeaderValue> {
    msg.headers()
        .iter()
        .map(|h| (h.name().as_str().to_string(), h.value()))
        .collect()
}
