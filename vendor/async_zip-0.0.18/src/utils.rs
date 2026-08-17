// Copyright (c) 2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::error::{Result, ZipError};
use futures_lite::io::{AsyncRead, AsyncReadExt};

// Assert that the next four-byte signature read by a reader which impls AsyncRead matches the expected signature.
pub(crate) async fn assert_signature<R: AsyncRead + Unpin>(reader: &mut R, expected: u32) -> Result<()> {
    let signature = {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer).await?;
        u32::from_le_bytes(buffer)
    };
    match signature {
        actual if actual == expected => Ok(()),
        actual => Err(ZipError::UnexpectedHeaderError(actual, expected)),
    }
}
